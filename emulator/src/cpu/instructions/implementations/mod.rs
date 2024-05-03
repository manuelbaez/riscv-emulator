pub mod int_register_immediate;
pub mod load;
pub mod store;

pub struct CpuInstructionsOpCodes;
impl CpuInstructionsOpCodes {
    pub const INT_REG_IMMEDIATE: u8 = int_register_immediate::OP_CODE;
    pub const ADD: u8 = 0x33;
    pub const LOAD: u8 = load::OP_CODE;
    pub const STORE: u8 = store::OP_CODE;
}
