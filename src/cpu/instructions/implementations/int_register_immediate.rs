use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::{
            decoder::b32::{ITypeDecoder, UTypeDecoder},
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
    ///Add Immediate Word
    pub const ADDIW: (u8, u8) = (0b000, 0b000);
    ///Set less than immediate
    pub const SLTI: u8 = 0x2;
    ///Set less than immediate unsigned
    pub const SLTIU: u8 = 0x3;
    pub const XORI: u8 = 0x4;
    pub const ORI: u8 = 0x6;
    pub const ANDI: u8 = 0x7;
    ///Shift Left Logical Immediate
    pub const SLLI: u8 = 0x01;
    ///Shift Left Logical Immediate Word
    pub const SLLIW: (u8, u8) = (0b001, 0b0000000);
    pub const SRLI_SRAI_F3: u8 = 0b101;
    ///Shift Right Logical Immediate
    pub const SRLI: (u8, u8) = (Self::SRLI_SRAI_F3, 0x00);
    ///Shift Right Arithmetic Immediate (Sign extend)
    pub const SRAI: (u8, u8) = (Self::SRLI_SRAI_F3, 0x10);
    ///Shift Right Logical Immediate Word
    pub const SRLIW: (u8, u8) = (Self::SRLI_SRAI_F3, 0x00);
    pub const SRAIW: (u8, u8) = (Self::SRLI_SRAI_F3, 0b010_0000);
}

impl InstructionsExecutor {
    /// Adds an immediate value to the value held on the rs1 register
    #[inline(always)]
    pub fn addi(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize]
                .wrapping_add(instruction.get_i_imm() as u64),
        )
    }
    ///Set less than immediate
    #[inline(always)]
    pub fn slti(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        let value = (cpu.registers[instruction.get_rs1_field() as usize] as i64)
            < (instruction.get_i_imm() as i64);
        cpu.write_reg(instruction.get_rd_field() as usize, value as u64)
    }

    ///Set less than immediate unsigned
    #[inline(always)]
    pub fn sltiu(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        let value = cpu.registers[instruction.get_rs1_field() as usize] < instruction.get_i_imm();
        cpu.write_reg(instruction.get_rd_field() as usize, value as u64)
    }

    #[inline(always)]
    pub fn ori(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize] | instruction.get_i_imm(),
        )
    }

    #[inline(always)]
    pub fn xori(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize] ^ instruction.get_i_imm(),
        )
    }

    #[inline(always)]
    pub fn andi(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize] & instruction.get_i_imm(),
        )
    }

    #[inline(always)]
    pub fn slli(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        let shamt = (instruction.get_i_imm() & 0x3f) as u32; // shamt is encoded in the lower 6bit of the imm for RV64I
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize].wrapping_shl(shamt),
        )
    }

    #[inline(always)]
    pub fn slliw(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        let shamt = (instruction.get_i_imm() & 0x1f) as u32; // shamt is encoded in the lower 5bit of the imm for RV64I
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize].wrapping_shl(shamt) as i32 as i64
                as u64,
        )
    }

    #[inline(always)]
    pub fn srli(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        let shamt = (instruction.get_i_imm() & 0x3f) as u32; // shamt is encoded in the lower 6bit of the imm for RV64I
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize].wrapping_shr(shamt),
        )
    }

    #[inline(always)]
    pub fn srliw(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        let shamt = (instruction.get_i_imm() & 0x1f) as u32; // shamt is encoded in the lower 6bit of the imm for RV64I
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize].wrapping_shr(shamt) as i32 as i64
                as u64,
        )
    }

    #[inline(always)]
    pub fn srai(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        let shamt = (instruction.get_i_imm() & 0x3f) as u32; // shamt is encoded in the lower 6bit of the imm for RV64I
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            (cpu.registers[instruction.get_rs1_field() as usize] as i64).wrapping_shr(shamt) as u64,
        )
    }

    #[inline(always)]
    pub fn sraiw(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        let shamt = (instruction.get_i_imm() & 0x1f) as u32; // shamt is encoded in the lower 6bit of the imm for RV64I
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            (cpu.registers[instruction.get_rs1_field() as usize] as i32).wrapping_shr(shamt) as i64
                as u64,
        )
    }

    #[inline(always)]
    pub fn lui(cpu: &mut Cpu, instruction: impl UTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(instruction.get_rd_field() as usize, instruction.get_u_imm())
    }

    #[inline(always)]
    pub fn auipc(cpu: &mut Cpu, instruction: impl UTypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.program_counter
                .wrapping_add(instruction.get_u_imm())
                //subtract the isntruction size as we move the program counter by that at the begining of the execution
                .wrapping_sub(DEFAULT_INSTRUCTION_SIZE_BYTES as u64),
        )
    }

    #[inline(always)]
    pub fn addiw(cpu: &mut Cpu, instruction: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        cpu.write_reg(
            instruction.get_rd_field() as usize,
            cpu.registers[instruction.get_rs1_field() as usize]
                .wrapping_add(instruction.get_i_imm()) as i32 as i64 as u64,
        )
    }
}
