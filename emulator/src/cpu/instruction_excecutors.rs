use crate::error::AppResult;

use super::{instructions::decoder::RFormatInstruction, Cpu};

impl Cpu {
    pub fn add(&mut self, instruction: RFormatInstruction) -> AppResult<()> {
        self.registers[instruction.rd as usize] = self.registers[instruction.rs1 as usize]
            .wrapping_add(self.registers[instruction.rs2 as usize]);
        Ok(())
    }
}
