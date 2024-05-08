use crate::cpu::Cpu;

use super::SubFunctions;

///Funct3 field Sub-instructions
impl SubFunctions {
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
    // pub fn store(&mut self, instruction: u32) -> AppResult<()> {
    //     let decoder = STypeDecoder::new(instruction);
    //     let addr: u64 = self.registers[decoder.get_rs1() as usize].wrapping_add(decoder.get_imm());
    //     match decoder.get_funct3() {
    //         SubFunctions::SB => self.system_bus.store(
    //             addr,
    //             MemoryOpSize::B8,
    //             self.registers[decoder.get_rs2() as usize],
    //         ),
    //         SubFunctions::SH => self.system_bus.store(
    //             addr,
    //             MemoryOpSize::B16,
    //             self.registers[decoder.get_rs2() as usize],
    //         ),
    //         SubFunctions::SW => self.system_bus.store(
    //             addr,
    //             MemoryOpSize::B32,
    //             self.registers[decoder.get_rs2() as usize],
    //         ),
    //         SubFunctions::SD => self.system_bus.store(
    //             addr,
    //             MemoryOpSize::B64,
    //             self.registers[decoder.get_rs2() as usize],
    //         ),
    //         _ => Err(AppErrors::FuctionNotImplemented),
    //     }
    // }
}
