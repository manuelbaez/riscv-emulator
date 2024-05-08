use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::decoder::{RTypeDecoder, RdDecoder, Rs1Decoder, Rs2Decoder},
        Cpu,
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
}

impl InstructionsExecutor {
    /// Adds the value held on rs2 to rs1 and sets to rd:
    /// rd = rs1 + rs2
    #[inline(always)]
    pub fn add(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            cpu.registers[instruction.get_rs1() as usize]
                .wrapping_add(cpu.registers[instruction.get_rs2() as usize]),
        )
    }
    /// Substract the value held on rs2 to rs1 and sets to rd:
    /// rd = rs1 - rs2
    #[inline(always)]
    pub fn sub(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            cpu.registers[instruction.get_rs1() as usize]
                .wrapping_sub(cpu.registers[instruction.get_rs2() as usize]),
        )
    }
    /// Compares the values held in registers as signed by rs1 < rs2
    /// and sets the bool result on rd
    #[inline(always)]
    pub fn slt(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            ((cpu.registers[instruction.get_rs1() as usize] as i64)
                < (cpu.registers[instruction.get_rs2() as usize] as i64)) as u64,
        )
    }
    /// Compares the values held in registers as unsigned by rs1 < rs2
    /// and sets the bool result on rd
    #[inline(always)]
    pub fn sltu(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            (cpu.registers[instruction.get_rs1() as usize]
                < cpu.registers[instruction.get_rs2() as usize]) as u64,
        )
    }
    ///Bitwise AND
    #[inline(always)]
    pub fn and(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            (cpu.registers[instruction.get_rs1() as usize]
                & cpu.registers[instruction.get_rs2() as usize]) as u64,
        )
    }
    ///Bitwise OR
    #[inline(always)]
    pub fn or(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            (cpu.registers[instruction.get_rs1() as usize]
                | cpu.registers[instruction.get_rs2() as usize]) as u64,
        )
    }
    ///Bitwise XOR
    #[inline(always)]
    pub fn xor(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            (cpu.registers[instruction.get_rs1() as usize]
                ^ cpu.registers[instruction.get_rs2() as usize]) as u64,
        )
    }
    /// Performs a logical left shift on rs1 by the shift amount
    /// in the first 6 bits held in rs2; rd = rs1 << (rs2 & 0x3f)
    #[inline(always)]
    pub fn sll(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        // here rs2
        cpu.write_reg(
            instruction.get_rd() as usize,
            cpu.registers[instruction.get_rs1() as usize]
                .wrapping_shl((cpu.registers[instruction.get_rs2() as usize] & 0x3f) as u32),
        )
    }
    /// Performs a logical right shift on rs1 by the shift amount
    /// in the first 6 bits held in rs2; rd = rs1 >> (rs2 & 0x3f)
    #[inline(always)]
    pub fn srl(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            cpu.registers[instruction.get_rs1() as usize]
                .wrapping_shr((cpu.registers[instruction.get_rs2() as usize] & 0x3f) as u32),
        )
    }
    /// Performs a arimetric right shift (sign-extended) on rs1 by the shift amount
    /// in the first 6 bits held in rs2; rd = (rs1 as i64) >> (rs2 & 0x3f)
    #[inline(always)]
    pub fn sra(cpu: &mut Cpu, instruction: RTypeDecoder) -> AppResult<()> {
        cpu.write_reg(
            instruction.get_rd() as usize,
            (cpu.registers[instruction.get_rs1() as usize] as i64)
                .wrapping_shr((cpu.registers[instruction.get_rs2() as usize] & 0x3f) as u32)
                as u64,
        )
    }
}
