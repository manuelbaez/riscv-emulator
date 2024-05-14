use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::{
            decoder::{ITypeDecoder, JTypeDecoder, RdDecoder, Rs1Decoder},
            DEFAULT_INSTRUCTION_SIZE_BYTES,
        },
        side_effects::OperationSideEffect,
        Cpu,
    },
    error::AppResult,
};

impl InstructionsExecutor {
    /// Adds the immediate value to the Curent Program Counter
    /// and then sets the current Program Counter, also stores
    /// the current pc to the rd register + 4 to be used to
    /// return to the next instruction later
    #[inline(always)]
    pub fn jal(cpu: &mut Cpu, instruction: JTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            cpu.program_counter
                .wrapping_add(DEFAULT_INSTRUCTION_SIZE_BYTES as u64),
        )
        .unwrap();
        cpu.program_counter = cpu.program_counter.wrapping_add(instruction.get_imm());
        // .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64);
        Ok(OperationSideEffect::SkipPCIncrease)
    }
    /// Performs a program counter jump by adding the 12 bit immediate ro the rs1 register
    /// and setting the first bit to zero, it also stores the next program counter position
    /// to the rd register
    #[inline(always)]
    pub fn jalr(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(instruction.get_rd() as usize, cpu.program_counter)
            .unwrap();
        cpu.program_counter = cpu.registers[instruction.get_rs1() as usize]
            .wrapping_add(instruction.get_imm())
            & !0x1_u64;
        Ok(OperationSideEffect::SkipPCIncrease)
    }
}
