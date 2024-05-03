use crate::{
    cpu::{instructions::decoder::IFormatInstruction, Cpu},
    error::AppResult,
};

pub const OP_CODE: u8 = 0x13;
///Funct3 field Sub-instructions
pub struct SubInstructions;
impl SubInstructions {
    ///Add Immediate
    pub const ADDI: u8 = 0x0;
    ///Set less than immediate
    pub const SLTI: u8 = 0x2;
    ///Set less than immediate unsigned
    pub const SLTIU: u8 = 0x3;
    pub const XORI: u8 = 0x4;
    pub const ORI: u8 = 0x7;
}

impl Cpu {
    pub fn int_reg_immediate(&mut self, instruction: IFormatInstruction) -> AppResult<()> {
        match instruction.funct3 {
            SubInstructions::ADDI => {
                let value =
                    self.registers[instruction.rs1 as usize].wrapping_add(instruction.imm as u64);
                self.write_reg(instruction.rd as usize, value)
            }
            SubInstructions::SLTI => {
                let value =
                    (self.registers[instruction.rs1 as usize] as i64) < (instruction.imm as i64);
                self.write_reg(instruction.rd as usize, value as u64)
            }
            SubInstructions::SLTIU => {
                let value = self.registers[instruction.rs1 as usize] < instruction.imm;
                self.write_reg(instruction.rd as usize, value as u64)
            }
            _ => {
                dbg!(instruction.opcode);
                dbg!(instruction.funct3);
                Err(crate::error::AppErrors::InstructionNotImplemented)
            }
        }
    }
}
