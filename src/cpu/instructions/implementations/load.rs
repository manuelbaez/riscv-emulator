use crate::{
    cpu::{
        instruction_excecutors::InstructionsExecutor,
        instructions::decoder::{self, Funct3Decoder, ITypeDecoder, RdDecoder, Rs1Decoder},
        Cpu,
    },
    error::{AppErrors, AppResult},
};

use super::SubFunctions;

///Funct3 field Sub-instructions
impl SubFunctions {
    /// Load Byte
    pub const LB: u8 = 0x0;
    /// Load Half Word (16-bit)
    pub const LH: u8 = 0x1;
    /// Load Word (32-bit)
    pub const LW: u8 = 0x2;
    /// Load Double Word (64-bit)
    pub const LD: u8 = 0x3;
    /// Load Byte Usigned
    pub const LBU: u8 = 0x4;
    /// Load Half Word Unsigned (16-bit)
    pub const LHU: u8 = 0x5;
    /// Load Word Unsigned (32-bit)
    pub const LWU: u8 = 0x6;
}

impl InstructionsExecutor {
    #[inline(always)]
    pub fn load(cpu: &mut Cpu, instruction: u32) -> AppResult<()> {
        let decoder = ITypeDecoder::new(instruction);
        let addr: u64 = cpu.registers[decoder.get_rs1() as usize].wrapping_add(decoder.get_imm());
        match decoder.get_funct3() {
            SubFunctions::LB => match cpu.system_bus.load8(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd() as usize, value as i8 as i64 as u64),
                Err(err) => Err(err),
            },
            SubFunctions::LH => match cpu.system_bus.load16(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd() as usize, value as i16 as i64 as u64),
                Err(err) => Err(err),
            },
            SubFunctions::LW => match cpu.system_bus.load32(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd() as usize, value as i32 as i64 as u64),
                Err(err) => Err(err),
            },
            SubFunctions::LD => match cpu.system_bus.load64(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd() as usize, value),
                Err(err) => Err(err),
            },
            SubFunctions::LBU => match cpu.system_bus.load8(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd() as usize, value as u64),
                Err(err) => Err(err),
            },
            SubFunctions::LHU => match cpu.system_bus.load16(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd() as usize, value as u64),
                Err(err) => Err(err),
            },
            SubFunctions::LWU => match cpu.system_bus.load32(addr) {
                Ok(value) => cpu.write_reg(decoder.get_rd() as usize, value as u64),
                Err(err) => Err(err),
            },
            _ => Err(AppErrors::InstructionNotImplemented(decoder::get_op_code(
                instruction,
            ))),
        }
    }
}
