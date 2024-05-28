pub mod b32;

use crate::error::AppResult;

#[inline(always)]
pub fn get_op_code(instruction: u32) -> u8 {
    (instruction & 0x7f) as u8
}
#[inline(always)]
pub fn get_instruction_size(opcode: u8) -> AppResult<InstructionSize> {
    match opcode {
        _ if opcode & 0b11 == 0b11 => Ok(InstructionSize::B32),
        _ if (opcode & 0b11 == 0b00) || (opcode & 0b11 == 0b01) || (opcode & 0b11 == 0b10) => {
            Ok(InstructionSize::B16)
        }
        _ => Err(crate::error::AppErrors::InstructionSizeNotSupported),
    }
}

pub enum InstructionSize {
    B16 = 2,
    B32 = 4,
}

pub trait InstructionRawGetter {
    fn get_raw_instruction(&self) -> u32;
}
