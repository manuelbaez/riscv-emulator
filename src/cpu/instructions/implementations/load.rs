use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor, instructions::decoder::b32::ITypeDecoder,
        side_effects::OperationSideEffect, Cpu,
    },
    error::{AppErrors, AppResult},
};

use super::SubFunctions;

///Funct3 field Sub-instructions
impl SubFunctions {
    /// Load Byte
    pub const LB: u8 = 0b000;
    /// Load Half Word (16-bit)
    pub const LH: u8 = 0b001;
    /// Load Word (32-bit)
    pub const LW: u8 = 0b010;
    /// Load Double Word (64-bit)
    pub const LD: u8 = 0b011;
    /// Load Byte Usigned
    pub const LBU: u8 = 0b100;
    /// Load Half Word Unsigned (16-bit)
    pub const LHU: u8 = 0b101;
    /// Load Word Unsigned (32-bit)
    pub const LWU: u8 = 0b110;
}

impl InstructionsExecutor {
    #[inline(always)]
    pub fn load(cpu: &mut Cpu, decoder: impl ITypeDecoder) -> AppResult<OperationSideEffect> {
        let addr: u64 =
            cpu.registers[decoder.get_rs1_field() as usize].wrapping_add(decoder.get_i_imm());
        match decoder.get_funct3_field() {
            SubFunctions::LB => match cpu.system_bus.load8(addr) {
                Ok(value) => {
                    cpu.write_reg(decoder.get_rd_field() as usize, value as i8 as i64 as u64)
                }
                Err(err) => Err(err),
            },
            SubFunctions::LH => match cpu.system_bus.load16(addr) {
                Ok(value) => {
                    cpu.write_reg(decoder.get_rd_field() as usize, value as i16 as i64 as u64)
                }
                Err(err) => Err(err),
            },
            SubFunctions::LW => match cpu.system_bus.load32(addr) {
                Ok(value) => {
                    cpu.write_reg(decoder.get_rd_field() as usize, value as i32 as i64 as u64)
                }
                Err(err) => Err(err),
            },
            SubFunctions::LD => match cpu.system_bus.load64(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd_field() as usize, value),
                Err(err) => Err(err),
            },
            SubFunctions::LBU => match cpu.system_bus.load8(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd_field() as usize, value as u64),
                Err(err) => Err(err),
            },
            SubFunctions::LHU => match cpu.system_bus.load16(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd_field() as usize, value as u64),
                Err(err) => Err(err),
            },
            SubFunctions::LWU => match cpu.system_bus.load32(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd_field() as usize, value as u64),
                Err(err) => Err(err),
            },
            _ => Err(AppErrors::InstructionNotImplemented {
                instruction: decoder.get_raw_instruction(),
            }),
        }
    }
}
