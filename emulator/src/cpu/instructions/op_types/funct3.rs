pub struct LoadInstructionOptypes;
impl LoadInstructionOptypes {
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


pub struct StoreInstructionOptypes;
impl StoreInstructionOptypes {
    /// Store Byte
    pub const SB: u8 = 0x0;
    /// Store Half Word (16-bit)
    pub const SH: u8 = 0x1;
    /// Store Word (32-bit)
    pub const SW: u8 = 0x2;
    /// Store Double Word (64-bit)
    pub const SD: u8 = 0x3;
}
