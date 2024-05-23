use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor, instructions::decoder::b32::RTypeDecoder,
        side_effects::OperationSideEffect, Cpu,
    },
    error::AppResult,
};

use super::SubFunctions;

impl SubFunctions {
    pub const ADDW: (u8, u8) = (0x00, 0x00);
    pub const SUBW: (u8, u8) = (0x00, 0b0100000);
}

impl InstructionsExecutor {
    /// Adds the value held on rs2 to rs1 and sets to rd:
    /// rd = rs1 + rs2
    /// This instruction only sets the lower 32 bits and
    /// sign extends the value to 64
    #[inline(always)]
    pub fn addw(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize]
                .wrapping_add(cpu.registers[instruction.get_rs2_field() as usize])
                as i32 as i64 as u64,
        )
    }
    /// Substract the value held on rs2 to rs1 and sets to rd:
    /// rd = rs1 - rs2
    /// This instruction only sets the lower 32 bits and
    /// sign extends the value to 64
    #[inline(always)]
    pub fn subw(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize]
                .wrapping_sub(cpu.registers[instruction.get_rs2_field() as usize])
                as i32 as i64 as u64,
        )
    }
}
