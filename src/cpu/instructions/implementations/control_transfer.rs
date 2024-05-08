use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::{
            decoder::{JTypeDecoder, RdDecoder},
            DEFAULT_INSTRUCTION_SIZE_BYTES,
        },
        Cpu,
    },
    error::AppResult,
};

impl InstructionsExecutor {
    /// Adds the immediate value to the Curent Program Counter
    /// and then sets the current Program Counter
    #[inline(always)]
    pub fn jal(cpu: &mut Cpu, instruction: JTypeDecoder) -> AppResult<()> {
        cpu.write_reg(instruction.get_rd() as usize, cpu.program_counter)
            .unwrap();
        Ok(cpu.program_counter = cpu
            .program_counter
            .wrapping_add(instruction.get_imm())
            .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64))
    }
}
