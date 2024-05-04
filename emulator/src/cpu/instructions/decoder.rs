pub fn get_op_code(instruction: u32) -> u8 {
    (instruction & 0x7f) as u8
}

//Standard Filed decoders

pub trait InstructionGetter {
    fn get_raw_instruction(&self) -> u32;
}
pub trait OpcodeDecoder: InstructionGetter {
    #[inline(always)]
    fn get_opcode(&self) -> u8 {
        (self.get_raw_instruction() & 0x7f) as u8
    }
}
pub trait RdDecoder: InstructionGetter {
    #[inline(always)]
    fn get_rd(&self) -> u8 {
        ((self.get_raw_instruction() >> 7) & 0x1f) as u8
    }
}
pub trait Funct3Decoder: InstructionGetter {
    #[inline(always)]
    fn get_funct3(&self) -> u8 {
        ((self.get_raw_instruction()) >> 12 & 0x07) as u8
    }
}
pub trait Rs1Decoder: InstructionGetter {
    #[inline(always)]
    fn get_rs1(&self) -> u8 {
        ((self.get_raw_instruction() >> 15) & 0x1f) as u8
    }
}
pub trait Rs2Decoder: InstructionGetter {
    #[inline(always)]
    fn get_rs2(&self) -> u8 {
        ((self.get_raw_instruction() >> 20) & 0x1f) as u8
    }
}
pub trait Funct7Decoder: InstructionGetter {
    #[inline(always)]
    fn get_funct7(&self) -> u8 {
        ((self.get_raw_instruction() >> 25) & 0x3f) as u8
    }
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
    /// Sign extended imm
    #[inline(always)]
    pub fn get_imm(&self) -> u64 {
        // & 0xffff_f000_u32 just in case this mask sets the lower 12 bits to 0
        ((self.instruction) as i32 as i64).wrapping_shr(12) as u64
    }
}
