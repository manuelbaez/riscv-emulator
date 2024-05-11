use super::SubFunctions;

///Funct3 field Sub-instructions
impl SubFunctions {
    /// Store Byte
    pub const SB: u8 = 0b000;
    /// Store Half Word (16-bit)
    pub const SH: u8 = 0b001;
    /// Store Word (32-bit)
    pub const SW: u8 = 0b010;
    /// Store Double Word (64-bit)
    pub const SD: u8 = 0b011;
}
