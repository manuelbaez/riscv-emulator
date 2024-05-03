
pub struct RFormatInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}
impl From<u32> for RFormatInstruction {
    fn from(instruction: u32) -> Self {
        Self {
            opcode: (instruction & 0x7f) as u8,
            rd: ((instruction >> 7) & 0x1f) as u8,
            funct3: ((instruction) >> 12 & 0x07) as u8,
            rs1: ((instruction >> 15) & 0x1f) as u8,
            rs2: ((instruction >> 20) & 0x1f) as u8,
            funct7: ((instruction >> 25) & 0x3f) as u8,
        }
    }
}

pub struct IFormatInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub imm: u64, //sign-extended and u64 to match register size
}
impl From<u32> for IFormatInstruction {
    fn from(instruction: u32) -> Self {
        Self {
            opcode: (instruction & 0x7f) as u8,
            rd: ((instruction >> 7) & 0x1f) as u8,
            funct3: ((instruction) >> 12 & 0x07) as u8,
            rs1: ((instruction >> 15) & 0x1f) as u8,
            imm: ((instruction as i32 as i64) >> 20) as u64,
        }
    }
}

pub struct SFormatInstruction {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    /// imm[11:5|4:0] = inst[31:25|11:7]
    pub imm: u64, //sign-extended and u64 to match register size
}
impl From<u32> for SFormatInstruction {
    fn from(instruction: u32) -> Self {
        Self {
            opcode: (instruction & 0x7f) as u8,
            funct3: ((instruction) >> 12 & 0x07) as u8,
            rs1: ((instruction >> 15) & 0x1f) as u8,
            rs2: ((instruction >> 20) & 0x1f) as u8,
            imm: ((instruction & 0xfe00_0000) as i32 as i64 >> 20) as u64
                | ((instruction >> 7) & 0x1f) as u64,
        }
    }
}

pub struct UFormatInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub imm: u64,
}
impl From<u32> for UFormatInstruction {
    fn from(instruction: u32) -> Self {
        Self {
            opcode: (instruction & 0x7f) as u8,
            rd: ((instruction >> 7) & 0x1f) as u8,
            imm: (instruction & 0xffff_f000_u32) as i32 as i64 as u64, // remove the first 12 bits for load upper instructions
        }
    }
}

pub struct InstructionsDecoder;
impl InstructionsDecoder {
    pub fn decode_r_format_instruction(instruction: u32) -> RFormatInstruction {
        RFormatInstruction::from(instruction)
    }
    pub fn decode_i_format_instruction(instruction: u32) -> IFormatInstruction {
        IFormatInstruction::from(instruction)
    }
    pub fn decode_s_format_instruction(instruction: u32) -> SFormatInstruction {
        SFormatInstruction::from(instruction)
    }
    pub fn decode_u_format_instruction(instruction: u32) -> UFormatInstruction {
        UFormatInstruction::from(instruction)
    }
    pub fn get_op_code(instruction: u32) -> u8 {
        (instruction & 0x7f) as u8
    }
}
