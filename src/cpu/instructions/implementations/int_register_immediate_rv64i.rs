use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::decoder::{ITypeDecoder, RdDecoder, Rs1Decoder},
        Cpu,
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
    pub fn addiw(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            cpu.registers[instruction.get_rs1() as usize].wrapping_add(instruction.get_imm()) as i32
                as i64 as u64,
        )
    }
}
