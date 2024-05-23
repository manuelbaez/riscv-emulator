pub mod conditional_branches;
pub mod control_transfer;
pub mod int_register_immediate;
pub mod int_register_immediate_rv64i;
pub mod int_registers;
pub mod int_registers_rv64i;
pub mod load;
pub mod memory_ordering;
pub mod store;
pub mod syscalls;
pub mod zicsr;

pub struct SubFunctions;

pub struct CpuInstructionsOpCodes;
impl CpuInstructionsOpCodes {
    pub const INT_REG_IMMEDIATE: u8 = 0x13;
    pub const INT_REG_IMMEDIATE_RV64I: u8 = 0b0011011;
    pub const INT_REG_IMMEDIATE_LUI: u8 = 0x37;
    pub const INT_REG_IMMEDIATE_AUIPC: u8 = 0x17;
    pub const INT_REG_REG_RV32I: u8 = 0x33;
    pub const INT_REG_REG_RV64I: u8 = 0b0111011;
    pub const MEM_ORDERING: u8 = 0b0001111;
    pub const CONDITIONAL_BRANCHES: u8 = 0b1100011;
    pub const CONTROL_JAL: u8 = 0b1101111;
    pub const CONTROL_JALR: u8 = 0b1100111;
    pub const LOAD: u8 = 0x03;
    pub const STORE: u8 = 0x23;
    pub const SYSCALLS_CSR: u8 = 0b1110011;
}
