use crate::cpu::instruction_excecutors::InstructionsExecutor;

use super::SubFunctions;

impl SubFunctions {
    ///funct3 only
    pub const CSRRW: u8 = 0b001;
}

impl InstructionsExecutor{

    // pub fn csrrw()
}