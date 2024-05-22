use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::{
            decoder::{
                b32::{Rs1Decoder, Rs2Decoder},
                BTypeDecoder,
            },
            DEFAULT_INSTRUCTION_SIZE_BYTES,
        },
        side_effects::OperationSideEffect,
        Cpu,
    },
    error::AppResult,
};

use super::SubFunctions;

impl SubFunctions {
    pub const BEQ: u8 = 0b000;
    pub const BNE: u8 = 0b001;
    pub const BLT: u8 = 0b100;
    pub const BLTU: u8 = 0b110;
    pub const BGE: u8 = 0b101;
    pub const BGEU: u8 = 0b111;
}

impl InstructionsExecutor {
    /// Compares the values held on rs1 and rs2, if the values are equal
    /// it adds the immediate value to the current pc address to continue
    /// program execution in that address
    #[inline(always)]
    pub fn beq(cpu: &mut Cpu, instruction: BTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.program_counter = cpu
            .program_counter
            .wrapping_add(
                (cpu.registers[instruction.get_rs1_field() as usize]
                    == cpu.registers[instruction.get_rs2_field() as usize]) as u64
                    * (instruction
                        .get_imm()
                        .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64)),
            )
            .wrapping_add(DEFAULT_INSTRUCTION_SIZE_BYTES as u64);
        //Subtracting the instruction size as we have advanced the program counter beforehand
        Ok(OperationSideEffect::SkipPCIncrease)
    }

    /// Compares the values held on rs1 and rs2, if the values are not equal
    /// it adds the immediate value to the current pc address to continue
    /// program execution in that address
    #[inline(always)]
    pub fn bne(cpu: &mut Cpu, instruction: BTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.program_counter = cpu
            .program_counter
            .wrapping_add(
                (cpu.registers[instruction.get_rs1_field() as usize]
                    != cpu.registers[instruction.get_rs2_field() as usize]) as u64
                    * (instruction
                        .get_imm()
                        //Subtracting the instruction size as we have advanced the program counter beforehand
                        .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64)),
            )
            .wrapping_add(DEFAULT_INSTRUCTION_SIZE_BYTES as u64);
        Ok(OperationSideEffect::SkipPCIncrease)
    }

    /// Compares the signed values held on rs1 and rs2, if rs1 is less than rs2
    /// it adds the immediate value to the current pc address to continue
    /// program execution in that address
    #[inline(always)]
    pub fn blt(cpu: &mut Cpu, instruction: BTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.program_counter = cpu
            .program_counter
            .wrapping_add(
                ((cpu.registers[instruction.get_rs1_field() as usize] as i64)
                    < (cpu.registers[instruction.get_rs2_field() as usize] as i64))
                    as u64
                    * (instruction
                        .get_imm()
                        //Subtracting the instruction size as we have advanced the program counter beforehand
                        .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64)),
            )
            .wrapping_add(DEFAULT_INSTRUCTION_SIZE_BYTES as u64);
        Ok(OperationSideEffect::SkipPCIncrease)
    }

    /// Compares the unsigned values held on rs1 and rs2, if rs1 is less than rs2
    /// it adds the immediate value to the current pc address to continue
    /// program execution in that address
    #[inline(always)]
    pub fn bltu(cpu: &mut Cpu, instruction: BTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.program_counter = cpu
            .program_counter
            .wrapping_add(
                (cpu.registers[instruction.get_rs1_field() as usize]
                    < cpu.registers[instruction.get_rs2_field() as usize]) as u64
                    * (instruction
                        .get_imm()
                        //Subtracting the instruction size as we have advanced the program counter beforehand
                        .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64)),
            )
            .wrapping_add(DEFAULT_INSTRUCTION_SIZE_BYTES as u64);
        Ok(OperationSideEffect::SkipPCIncrease)
    }

    /// Compares the signed values held on rs1 and rs2, if rs1 is greater than or equals rs2
    /// it adds the immediate value to the current pc address to continue
    /// program execution in that address
    #[inline(always)]
    pub fn bge(cpu: &mut Cpu, instruction: BTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.program_counter = cpu
            .program_counter
            .wrapping_add(
                ((cpu.registers[instruction.get_rs1_field() as usize] as i64)
                    >= (cpu.registers[instruction.get_rs2_field() as usize] as i64))
                    as u64
                    * (instruction
                        .get_imm()
                        //Subtracting the instruction size as we have advanced the program counter beforehand
                        .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64)),
            )
            .wrapping_add(DEFAULT_INSTRUCTION_SIZE_BYTES as u64);
        Ok(OperationSideEffect::SkipPCIncrease)
    }

    /// Compares the unsigned values held on rs1 and rs2, if rs1 is greater than
    /// or equals rs2, it adds the immediate value to the current pc address to
    /// continue program execution in that address
    #[inline(always)]
    pub fn bgeu(cpu: &mut Cpu, instruction: BTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.program_counter = cpu
            .program_counter
            .wrapping_add(
                (cpu.registers[instruction.get_rs1_field() as usize]
                    >= cpu.registers[instruction.get_rs2_field() as usize]) as u64
                    * (instruction
                        .get_imm()
                        //Subtracting the instruction size as we have advanced the program counter beforehand
                        .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64)),
            )
            .wrapping_add(DEFAULT_INSTRUCTION_SIZE_BYTES as u64);
        Ok(OperationSideEffect::SkipPCIncrease)
    }
}
