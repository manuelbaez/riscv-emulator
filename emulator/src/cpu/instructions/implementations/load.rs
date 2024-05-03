use crate::{
    cpu::{instructions::decoder::IFormatInstruction, Cpu},
    error::{AppErrors, AppResult},
};

pub const OP_CODE: u8 = 0x03;

///Funct3 field Sub-instructions
pub struct SubInstructions;
impl SubInstructions {
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

impl Cpu {
    pub fn load(&mut self, instruction: IFormatInstruction) -> AppResult<()> {
        let addr: u64 = self.registers[instruction.rs1 as usize].wrapping_add(instruction.imm);
        match instruction.funct3 {
            SubInstructions::LB => match self.system_bus.load8(addr) {
                Ok(value) => {
                    self.write_reg(instruction.rd as usize, value as i8 as i64 as u64)
                }
                Err(err) => Err(err),
            },
            SubInstructions::LH => match self.system_bus.load16(addr) {
                Ok(value) => {
                    self.write_reg(instruction.rd as usize, value as i16 as i64 as u64)
                }
                Err(err) => Err(err),
            },
            SubInstructions::LW => match self.system_bus.load32(addr) {
                Ok(value) => {
                    self.write_reg(instruction.rd as usize, value as i32 as i64 as u64)
                }
                Err(err) => Err(err),
            },
            SubInstructions::LD => match self.system_bus.load64(addr) {
                Ok(value) => {
                    self.write_reg(instruction.rd as usize, value)
                }
                Err(err) => Err(err),
            },
            SubInstructions::LBU => match self.system_bus.load8(addr) {
                Ok(value) => self.write_reg(instruction.rd as usize, value as u64),
                Err(err) => Err(err),
            },
            SubInstructions::LHU => match self.system_bus.load16(addr) {
                Ok(value) => self.write_reg(instruction.rd as usize, value as u64),
                Err(err) => Err(err),
            },
            SubInstructions::LWU => match self.system_bus.load32(addr) {
                Ok(value) => self.write_reg(instruction.rd as usize, value as u64),
                Err(err) => Err(err),
            },
            _ => Err(AppErrors::FuctionNotImplemented),
        }
    }
}
