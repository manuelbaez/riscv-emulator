use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor, instructions::decoder::b32::RTypeDecoder,
        side_effects::OperationSideEffect, Cpu,
    },
    error::AppResult,
};

use super::SubFunctions;

impl SubFunctions {
    pub const ADD: (u8, u8) = (0x00, 0x00);
    pub const SUB: (u8, u8) = (0x00, 0b0100000);
    pub const SLT: (u8, u8) = (0b010, 0b0000000);
    pub const SLTU: (u8, u8) = (0b011, 0b0000000);
    pub const AND: (u8, u8) = (0b111, 0b0000000);
    pub const OR: (u8, u8) = (0b110, 0b0000000);
    pub const XOR: (u8, u8) = (0b100, 0b0000000);
    pub const SLL: (u8, u8) = (0b001, 0b0000000);
    pub const SRL: (u8, u8) = (0b101, 0b0000000);
    pub const SRA: (u8, u8) = (0b101, 0b0100000);
    pub const ADDW: (u8, u8) = (0x00, 0x00);
    pub const SUBW: (u8, u8) = (0x00, 0b0100000);
}

impl InstructionsExecutor {
    /// Adds the value held on rs2 to rs1 and sets to rd:
    /// rd = rs1 + rs2
    #[inline(always)]
    pub fn add(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize]
                .wrapping_add(cpu.registers[instruction.get_rs2_field() as usize]),
        )
    }
    /// Substract the value held on rs2 to rs1 and sets to rd:
    /// rd = rs1 - rs2
    #[inline(always)]
    pub fn sub(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize]
                .wrapping_sub(cpu.registers[instruction.get_rs2_field() as usize]),
        )
    }
    /// Compares the values held in registers as signed by rs1 < rs2
    /// and sets the bool result on rd
    #[inline(always)]
    pub fn slt(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            ((cpu.registers[instruction.get_rs1_field() as usize] as i64)
                < (cpu.registers[instruction.get_rs2_field() as usize] as i64)) as u64,
        )
    }
    /// Compares the values held in registers as unsigned by rs1 < rs2
    /// and sets the bool result on rd
    #[inline(always)]
    pub fn sltu(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            (cpu.registers[instruction.get_rs1_field() as usize]
                < cpu.registers[instruction.get_rs2_field() as usize]) as u64,
        )
    }
    ///Bitwise AND
    #[inline(always)]
    pub fn and(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            (cpu.registers[instruction.get_rs1_field() as usize]
                & cpu.registers[instruction.get_rs2_field() as usize]) as u64,
        )
    }
    ///Bitwise OR
    #[inline(always)]
    pub fn or(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            (cpu.registers[instruction.get_rs1_field() as usize]
                | cpu.registers[instruction.get_rs2_field() as usize]) as u64,
        )
    }
    ///Bitwise XOR
    #[inline(always)]
    pub fn xor(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            (cpu.registers[instruction.get_rs1_field() as usize]
                ^ cpu.registers[instruction.get_rs2_field() as usize]) as u64,
        )
    }
    /// Performs a logical left shift on rs1 by the shift amount
    /// in the first 6 bits held in rs2; rd = rs1 << (rs2 & 0x3f)
    #[inline(always)]
    pub fn sll(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize]
                .wrapping_shl((cpu.registers[instruction.get_rs2_field() as usize] & 0x3f) as u32),
        )
    }
    /// Performs a logical right shift on rs1 by the shift amount
    /// in the first 6 bits held in rs2; rd = rs1 >> (rs2 & 0x3f)
    #[inline(always)]
    pub fn srl(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize]
                .wrapping_shr((cpu.registers[instruction.get_rs2_field() as usize] & 0x3f) as u32),
        )
    }
    /// Performs a arimetric right shift (sign-extended) on rs1 by the shift amount
    /// in the first 6 bits held in rs2; rd = (rs1 as i64) >> (rs2 & 0x3f)
    #[inline(always)]
    pub fn sra(cpu: &mut Cpu, instruction: impl RTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            (cpu.registers[instruction.get_rs1_field() as usize] as i64)
                .wrapping_shr((cpu.registers[instruction.get_rs2_field() as usize] & 0x3f) as u32)
                as u64,
        )
    }

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
