use super::InstructionGetter;

#[allow(dead_code)]
pub enum InstructionFormat {
    R,
    I,
    S,
    B,
    U,
    J,
}

pub struct Instrunction32Decoder {
    instruction: u32,
}

impl InstructionGetter for Instrunction32Decoder {
    #[inline(always)]
    fn get_raw_instruction(&self) -> u32 {
        self.instruction
    }
}
impl OpcodeDecoder for Instrunction32Decoder {}
impl RdDecoder for Instrunction32Decoder {}
impl Funct3Decoder for Instrunction32Decoder {}
impl Rs1Decoder for Instrunction32Decoder {}
impl Rs2Decoder for Instrunction32Decoder {}
impl Funct7Decoder for Instrunction32Decoder {}

impl RTypeDecoder for Instrunction32Decoder {}
impl ITypeDecoder for Instrunction32Decoder {}
impl STypeDecoder for Instrunction32Decoder {}
impl UTypeDecoder for Instrunction32Decoder {}
impl JTypeDecoder for Instrunction32Decoder {}
impl BTypeDecoder for Instrunction32Decoder {}

impl Instrunction32Decoder {
    #[inline(always)]
    pub fn new(instruction: u32) -> Self {
        Self { instruction }
    }

    #[inline(always)]
    pub fn get_imm_field(&self, format: InstructionFormat) -> u64 {
        match format {
            InstructionFormat::I => self.get_i_imm(),
            InstructionFormat::S => self.get_s_imm(),
            InstructionFormat::U => self.get_u_imm(),
            InstructionFormat::J => self.get_j_imm(),
            InstructionFormat::B => self.get_b_imm(),
            _ => panic!(),
        }
    }
}

//Standard Filed decoders for 3.02 bit instructions
#[allow(dead_code)]
pub trait OpcodeDecoder: InstructionGetter {
    #[inline(always)]
    fn get_opcode(&self) -> u8 {
        (self.get_raw_instruction() & 0x7f) as u8
    }
}
pub trait RdDecoder: InstructionGetter {
    #[inline(always)]
    fn get_rd_field(&self) -> u8 {
        ((self.get_raw_instruction() >> 7) & 0x1f) as u8
    }
}
pub trait Funct3Decoder: InstructionGetter {
    #[inline(always)]
    fn get_funct3_field(&self) -> u8 {
        ((self.get_raw_instruction()) >> 12 & 0x07) as u8
    }
}
pub trait Rs1Decoder: InstructionGetter {
    #[inline(always)]
    fn get_rs1_field(&self) -> u8 {
        ((self.get_raw_instruction() >> 15) & 0x1f) as u8
    }
}
pub trait Rs2Decoder: InstructionGetter {
    #[inline(always)]
    fn get_rs2_field(&self) -> u8 {
        ((self.get_raw_instruction() >> 20) & 0x1f) as u8
    }
}
pub trait Funct7Decoder: InstructionGetter {
    #[inline(always)]
    fn get_funct7_field(&self) -> u8 {
        ((self.get_raw_instruction() >> 25) & 0x3f) as u8
    }
}
// Standard formats decoder traits
pub trait RTypeDecoder:
    OpcodeDecoder + RdDecoder + Funct3Decoder + Rs1Decoder + Rs2Decoder + Funct7Decoder
{
}

pub trait ITypeDecoder:
    InstructionGetter + OpcodeDecoder + RdDecoder + Funct3Decoder + Rs1Decoder
{
    #[inline(always)]
    fn get_i_imm(&self) -> u64 {
        ((self.get_raw_instruction() as i32 as i64) >> 20) as u64
    }
}

pub trait STypeDecoder:
    InstructionGetter + OpcodeDecoder + Funct3Decoder + Rs1Decoder + Rs2Decoder
{
    #[inline(always)]
    fn get_s_imm(&self) -> u64 {
        ((self.get_raw_instruction() & 0xfe00_0000) as i32 as i64 >> 20) as u64
            | ((self.get_raw_instruction() >> 7) & 0x1f) as u64
    }
}

pub trait UTypeDecoder: InstructionGetter + OpcodeDecoder + RdDecoder {
    #[inline(always)]
    fn get_u_imm(&self) -> u64 {
        // & 0xffff_f000_u32 just in case this mask sets the lower 12 bits to 0
        (self.get_raw_instruction() & 0xffff_f000_u32) as i32 as i64 as u64
    }
}

pub trait JTypeDecoder: InstructionGetter + OpcodeDecoder + RdDecoder {
    #[inline(always)]
    fn get_j_imm(&self) -> u64 {
        (((self.get_raw_instruction() & 0x80000000) as i32 as i64 >>11) as u64) //Bit [20]
        | (self.get_raw_instruction() & 0xff000) as u64 //Bits [19:12]
        | ((self.get_raw_instruction() & 0x100000) >> 9) as u64 // Bit [11]
        | ((self.get_raw_instruction() & 0x7fe00000) >> 20) as u64 // Bits [10:1]
    }
}

pub trait BTypeDecoder:
    InstructionGetter + OpcodeDecoder + Funct3Decoder + Rs1Decoder + Rs2Decoder
{
    #[inline(always)]
    fn get_b_imm(&self) -> u64 {
        ((self.get_raw_instruction() & 0x8000_0000) as i32 as i64 >> 19) as u64 // Bit [12]
        | ((self.get_raw_instruction() as u64 & 0x80) << 4) // Bit [11]
        | ((self.get_raw_instruction() as u64 >> 20) & 0x7e0)   // Bit [10:5];
        | (self.get_raw_instruction() as u64 >> 7) & 0x1e // Bit [4:1]
    }
}
