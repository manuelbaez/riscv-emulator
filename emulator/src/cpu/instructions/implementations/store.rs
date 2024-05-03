use crate::{
    cpu::{instructions::decoder::SFormatInstruction, Cpu},
    error::{AppErrors, AppResult},
    memory::MemoryOpSize,
};

pub const OP_CODE: u8 = 0x23;

///Funct3 field Sub-instructions
pub struct SubInstructions;
impl SubInstructions {
    /// Store Byte
    pub const SB: u8 = 0x0;
    /// Store Half Word (16-bit)
    pub const SH: u8 = 0x1;
    /// Store Word (32-bit)
    pub const SW: u8 = 0x2;
    /// Store Double Word (64-bit)
    pub const SD: u8 = 0x3;
}

impl Cpu {
    pub fn store(&mut self, instruction: SFormatInstruction) -> AppResult<()> {
        let addr: u64 = self.registers[instruction.rs1 as usize].wrapping_add(instruction.imm);
        match instruction.funct3 {
            SubInstructions::SB => self.system_bus.store(
                addr,
                MemoryOpSize::B8,
                self.registers[instruction.rs2 as usize],
            ),
            SubInstructions::SH => self.system_bus.store(
                addr,
                MemoryOpSize::B16,
                self.registers[instruction.rs2 as usize],
            ),
            SubInstructions::SW => self.system_bus.store(
                addr,
                MemoryOpSize::B32,
                self.registers[instruction.rs2 as usize],
            ),
            SubInstructions::SD => self.system_bus.store(
                addr,
                MemoryOpSize::B64,
                self.registers[instruction.rs2 as usize],
            ),
            _ => Err(AppErrors::FuctionNotImplemented),
        }
    }
}
