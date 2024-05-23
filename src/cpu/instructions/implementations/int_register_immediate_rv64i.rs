use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor, instructions::decoder::b32::ITypeDecoder,
        side_effects::OperationSideEffect, Cpu,
    },
    error::AppResult,
};

use super::SubFunctions;

///Funct3/7 field Sub-instructions
impl SubFunctions {
    pub const ADDIW: (u8, u8) = (0b000, 0b000);
}

impl InstructionsExecutor {
    #[inline(always)]
    pub fn addiw(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize].wrapping_add(instruction.get_i_imm())
                as i32 as i64 as u64,
        )
    }
}
