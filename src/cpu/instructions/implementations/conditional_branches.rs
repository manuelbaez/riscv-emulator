use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::{
            decoder::{BTypeDecoder, Rs1Decoder, Rs2Decoder},
            DEFAULT_INSTRUCTION_SIZE_BYTES,
        },
        Cpu,
    },
    error::AppResult,
};

use super::SubFunctions;

impl SubFunctions {
    pub const BEQ: u8 = 0b000;
    pub const BNE: u8 = 0b001;
}

impl InstructionsExecutor {
    /// Compares the values held on rs1 and rs2, if the values are equal
    /// it adds the immediate value to the current pc address to continue
    /// program execution in that branch
    #[inline(always)]
    pub fn beq(cpu: &mut Cpu, instruction: BTypeDecoder) -> AppResult<()> {
        cpu.program_counter = cpu.program_counter.wrapping_add(
            (cpu.registers[instruction.get_rs1() as usize]
                == cpu.registers[instruction.get_rs2() as usize]) as u64
                * (instruction
                    .get_imm()
                    //Subtracting the instruction size as we have advanced the program counter beforehand
                    .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64)),
        );
        Ok(())
    }

    /// Compares the values held on rs1 and rs2, if the values are not equal
    /// it adds the immediate value to the current pc address to continue
    /// program execution in that branch
    #[inline(always)]
    pub fn bne(cpu: &mut Cpu, instruction: BTypeDecoder) -> AppResult<()> {
        cpu.program_counter = cpu.program_counter.wrapping_add(
            (cpu.registers[instruction.get_rs1() as usize]
                != cpu.registers[instruction.get_rs2() as usize]) as u64
                * (instruction
                    .get_imm()
                    //Subtracting the instruction size as we have advanced the program counter beforehand
                    .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64)),
        );
        Ok(())
    }
}
