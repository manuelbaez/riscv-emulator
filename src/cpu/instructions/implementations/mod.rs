pub mod int_register_immediate;
pub mod int_registers;
pub mod load;
pub mod store;
pub mod control_transfer;

pub struct SubFunctions;

pub struct CpuInstructionsOpCodes;
impl CpuInstructionsOpCodes {
    pub const INT_REG_IMMEDIATE: u8 = 0x13;
    pub const INT_REG_IMMEDIATE_LUI: u8 = 0x37;
    pub const INT_REG_IMMEDIATE_AUIPC: u8 = 0x17;
    pub const INT_REG_REG: u8 = 0x33;
    pub const JAL: u8 = 0b1101111;
    pub const LOAD: u8 = 0x03;
    pub const STORE: u8 = 0x23;
}