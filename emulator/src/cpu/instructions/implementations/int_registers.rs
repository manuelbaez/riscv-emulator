use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::decoder::{RTypeDecoder, RdDecoder, Rs1Decoder, Rs2Decoder},
        Cpu,
    },
    error::AppResult,
};

impl InstructionsExecutor {
    #[inline(always)]
    pub fn add(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        Ok(cpu.registers[instruction.get_rd() as usize] = cpu.registers
            [instruction.get_rs1() as usize]
            .wrapping_add(cpu.registers[instruction.get_rs2() as usize]))
    }
}
