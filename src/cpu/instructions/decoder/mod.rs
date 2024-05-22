pub mod b32;

use crate::error::AppResult;

use self::b32::{Funct3Decoder, Funct7Decoder, OpcodeDecoder, RdDecoder, Rs1Decoder, Rs2Decoder};

#[inline(always)]
pub fn get_op_code(instruction: u32) -> u8 {
    (instruction & 0x7f) as u8
}
#[inline(always)]
pub fn get_instruction_size(opcode: u8) -> AppResult<InstructionSize> {
    match opcode {
        _ if opcode & 0b11 == 0b11 => Ok(InstructionSize::B32),
        _ if (opcode & 0b11 == 0b00) || (opcode & 0b11 == 0b01) || (opcode & 0b11 == 0b10) => {
            Ok(InstructionSize::B16)
        }
        _ => Err(crate::error::AppErrors::InstructionSizeNotSupported),
    }
}

pub enum InstructionSize {
    B16 = 2,
    B32 = 4,
}

pub trait InstructionGetter {
    fn get_raw_instruction(&self) -> u32;
}

//Format decoders

pub struct RTypeDecoder {
    instruction: u32,
}

impl InstructionGetter for RTypeDecoder {
    #[inline(always)]
    fn get_raw_instruction(&self) -> u32 {
        self.instruction
    }
}
impl OpcodeDecoder for RTypeDecoder {}
impl RdDecoder for RTypeDecoder {}
impl Funct3Decoder for RTypeDecoder {}
impl Rs1Decoder for RTypeDecoder {}
impl Rs2Decoder for RTypeDecoder {}
impl Funct7Decoder for RTypeDecoder {}
impl RTypeDecoder {
    pub fn new(instruction: u32) -> Self {
        Self { instruction }
    }
}

pub struct ITypeDecoder {
    instruction: u32,
}
impl InstructionGetter for ITypeDecoder {
    #[inline(always)]
    fn get_raw_instruction(&self) -> u32 {
        self.instruction
    }
}
impl OpcodeDecoder for ITypeDecoder {}
impl RdDecoder for ITypeDecoder {}
impl Funct3Decoder for ITypeDecoder {}
impl Rs1Decoder for ITypeDecoder {}
impl ITypeDecoder {
    pub fn new(instruction: u32) -> Self {
        Self { instruction }
    }
    #[inline(always)]
    pub fn get_imm(&self) -> u64 {
        ((self.instruction as i32 as i64) >> 20) as u64
    }
}

pub struct STypeDecoder {
    instruction: u32,
}
impl InstructionGetter for STypeDecoder {
    #[inline(always)]
    fn get_raw_instruction(&self) -> u32 {
        self.instruction
    }
}
impl OpcodeDecoder for STypeDecoder {}
impl Funct3Decoder for STypeDecoder {}
impl Rs1Decoder for STypeDecoder {}
impl Rs2Decoder for STypeDecoder {}
impl STypeDecoder {
    pub fn new(instruction: u32) -> Self {
        Self { instruction }
    }
    #[inline(always)]
    pub fn get_imm(&self) -> u64 {
        ((self.instruction & 0xfe00_0000) as i32 as i64 >> 20) as u64
            | ((self.instruction >> 7) & 0x1f) as u64
    }
}

pub struct UTypeDecoder {
    instruction: u32,
}
impl InstructionGetter for UTypeDecoder {
    #[inline(always)]
    fn get_raw_instruction(&self) -> u32 {
        self.instruction
    }
}
impl OpcodeDecoder for UTypeDecoder {}
impl RdDecoder for UTypeDecoder {}
impl UTypeDecoder {
    pub fn new(instruction: u32) -> Self {
        Self { instruction }
    }
    /// Sign extended imm to u64 with the lower 12-bits set to zero
    #[inline(always)]
    pub fn get_imm(&self) -> u64 {
        // & 0xffff_f000_u32 just in case this mask sets the lower 12 bits to 0
        (self.instruction & 0xffff_f000_u32) as i32 as i64 as u64
    }
}

pub struct JTypeDecoder {
    instruction: u32,
}
impl InstructionGetter for JTypeDecoder {
    #[inline(always)]
    fn get_raw_instruction(&self) -> u32 {
        self.instruction
    }
}
impl RdDecoder for JTypeDecoder {}
impl JTypeDecoder {
    pub fn new(instruction: u32) -> Self {
        Self { instruction }
    }

    #[inline(always)]
    pub fn get_imm(&self) -> u64 {
        (((self.instruction & 0x80000000) as i32 as i64 >>11) as u64) //Bit [20]
            | (self.instruction & 0xff000) as u64 //Bits [19:12]
            | ((self.instruction & 0x100000) >> 9) as u64 // Bit [11]
            | ((self.instruction & 0x7fe00000) >> 20) as u64 // Bits [10:1]
    }
}

pub struct BTypeDecoder {
    instruction: u32,
}
impl InstructionGetter for BTypeDecoder {
    #[inline(always)]
    fn get_raw_instruction(&self) -> u32 {
        self.instruction
    }
}
impl Funct3Decoder for BTypeDecoder {}
impl Rs1Decoder for BTypeDecoder {}
impl Rs2Decoder for BTypeDecoder {}
impl BTypeDecoder {
    pub fn new(instruction: u32) -> Self {
        Self { instruction }
    }

    #[inline(always)]
    pub fn get_imm(&self) -> u64 {
        ((self.instruction & 0x8000_0000) as i32 as i64 >> 19) as u64 // Bit [12]
        | ((self.instruction as u64 & 0x80) << 4) // Bit [11]
        | ((self.instruction as u64 >> 20) & 0x7e0)   // Bit [10:5];
        | (self.instruction as u64 >> 7) & 0x1e // Bit [4:1]
    }
}
