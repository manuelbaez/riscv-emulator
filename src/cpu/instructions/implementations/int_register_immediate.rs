use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::{
            decoder::{
                b32::{RdDecoder, Rs1Decoder},
                ITypeDecoder, UTypeDecoder,
            },
            DEFAULT_INSTRUCTION_SIZE_BYTES,
        },
        side_effects::OperationSideEffect,
        Cpu,
    },
    error::AppResult,
};

use super::SubFunctions;

///Funct3/7 field Sub-instructions
impl SubFunctions {
    //For opcode 0010011(0x13)
    ///Add Immediate
    pub const ADDI: u8 = 0x0;
    ///Set less than immediate
    pub const SLTI: u8 = 0x2;
    ///Set less than immediate unsigned
    pub const SLTIU: u8 = 0x3;
    pub const XORI: u8 = 0x4;
    pub const ORI: u8 = 0x6;
    pub const ANDI: u8 = 0x7;
    pub const SLLI: u8 = 0x01;

    pub const SRLI_SRAI_F3: u8 = 0x05;
    pub const SRLI: (u8, u8) = (Self::SRLI_SRAI_F3, 0x00);
    pub const SRAI: (u8, u8) = (Self::SRLI_SRAI_F3, 0x10);
}

impl InstructionsExecutor {
    #[inline(always)]
    pub fn addi(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize]
                .wrapping_add(instruction.get_imm() as u64),
        )
    }

    #[inline(always)]
    pub fn slti(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        let value = (cpu.registers[instruction.get_rs1_field() as usize] as i64)
            < (instruction.get_imm() as i64);
        cpu.write_reg(instruction.get_rd_field() as usize, value as u64)
    }

    #[inline(always)]
    pub fn sltiu(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        let value = cpu.registers[instruction.get_rs1_field() as usize] < instruction.get_imm();
        cpu.write_reg(instruction.get_rd_field() as usize, value as u64)
    }

    #[inline(always)]
    pub fn ori(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize] | instruction.get_imm(),
        )
    }

    #[inline(always)]
    pub fn xori(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize] ^ instruction.get_imm(),
        )
    }

    #[inline(always)]
    pub fn andi(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize] & instruction.get_imm(),
        )
    }

    #[inline(always)]
    pub fn slli(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        let shamt = (instruction.get_imm() & 0x3f) as u32; // shamt is encoded in the lower 6bit of the imm for RV64I
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize].wrapping_shl(shamt),
        )
    }

    #[inline(always)]
    pub fn srli(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        let shamt = (instruction.get_imm() & 0x3f) as u32; // shamt is encoded in the lower 6bit of the imm for RV64I
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize].wrapping_shr(shamt),
        )
    }

    #[inline(always)]
    pub fn srai(cpu: &mut Cpu, instruction: ITypeDecoder) -> AppResult<OperationSideEffect> {
        let shamt = (instruction.get_imm() & 0x3f) as u32; // shamt is encoded in the lower 6bit of the imm for RV64I
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            (cpu.registers[instruction.get_rs1_field() as usize] as i64).wrapping_shr(shamt) as u64,
        )
    }

    #[inline(always)]
    pub fn lui(cpu: &mut Cpu, instruction: UTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(instruction.get_rd_field() as usize, instruction.get_imm())
    }

    #[inline(always)]
    pub fn auipc(cpu: &mut Cpu, instruction: UTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.program_counter
                .wrapping_add(instruction.get_imm())
                //subtract the isntruction size as we move the program counter by that at the begining of the execution
                .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64),
        )
    }
}
