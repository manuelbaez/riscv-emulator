use crate::memory::MemoryOpSize;

use super::{
    instructions::{op_types::funct3, IFormatInstruction, RFormatInstruction, SFormatInstruction},
    Cpu,
};

impl Cpu {
    pub fn add(&mut self, instruction: RFormatInstruction) -> Result<(), ()> {
        self.registers[instruction.rd as usize] = self.registers[instruction.rs1 as usize]
            .wrapping_add(self.registers[instruction.rs2 as usize]);
        Ok(())
    }
    pub fn addi(&mut self, instruction: IFormatInstruction) -> Result<(), ()> {
        self.registers[instruction.rd as usize] =
            self.registers[instruction.rs1 as usize].wrapping_add(instruction.imm as u64);
        Ok(())
    }
    pub fn load(&mut self, instruction: IFormatInstruction) -> Result<(), ()> {
        let addr: u64 = self.registers[instruction.rs1 as usize].wrapping_add(instruction.imm);
        match instruction.funct3 {
            funct3::LoadInstructionOptypes::LB => match self.system_bus.load8(addr) {
                Ok(value) => {
                    self.registers[instruction.rd as usize] = value as i8 as i64 as u64;
                    Ok(())
                }
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LH => match self.system_bus.load16(addr) {
                Ok(value) => {
                    self.registers[instruction.rd as usize] = value as i16 as i64 as u64;
                    Ok(())
                }
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LW => match self.system_bus.load32(addr) {
                Ok(value) => {
                    self.registers[instruction.rd as usize] = value as i32 as i64 as u64;
                    Ok(())
                }
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LD => match self.system_bus.load64(addr) {
                Ok(value) => {
                    self.registers[instruction.rd as usize] = value;
                    Ok(())
                }
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LBU => match self.system_bus.load8(addr) {
                Ok(value) => Ok(self.registers[instruction.rd as usize] = value as u64),
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LHU => match self.system_bus.load16(addr) {
                Ok(value) => Ok(self.registers[instruction.rd as usize] = value as u64),
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LWU => match self.system_bus.load32(addr) {
                Ok(value) => Ok(self.registers[instruction.rd as usize] = value as u64),
                Err(err) => Err(err),
            },
            _ => Err(()),
        }
    }
    pub fn store(&mut self, instruction: SFormatInstruction) -> Result<(), ()> {
        let addr: u64 = self.registers[instruction.rs1 as usize].wrapping_add(instruction.imm);
        match instruction.funct3 {
            funct3::StoreInstructionOptypes::SB => self.system_bus.store(
                addr,
                MemoryOpSize::B8,
                self.registers[instruction.rs2 as usize],
            ),
            funct3::StoreInstructionOptypes::SH => self.system_bus.store(
                addr,
                MemoryOpSize::B16,
                self.registers[instruction.rs2 as usize],
            ),
            funct3::StoreInstructionOptypes::SW => self.system_bus.store(
                addr,
                MemoryOpSize::B32,
                self.registers[instruction.rs2 as usize],
            ),
            funct3::StoreInstructionOptypes::SD => self.system_bus.store(
                addr,
                MemoryOpSize::B64,
                self.registers[instruction.rs2 as usize],
            ),
            _ => Err(()),
        }
    }
}
