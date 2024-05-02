use crate::memory::MemoryOpSize;

use super::{
    instructions::{op_types::funct3, IFormatInstruction, RFormatInstruction, SFormatInstruction},
    Cpu,
};

pub struct CpuInstructionExecutors;
impl CpuInstructionExecutors {
    pub fn add(cpu: &mut Cpu, instruction: RFormatInstruction) -> Result<(), ()> {
        cpu.registers[instruction.rd as usize] = cpu.registers[instruction.rs1 as usize]
            .wrapping_add(cpu.registers[instruction.rs2 as usize]);
        Ok(())
    }
    pub fn addi(cpu: &mut Cpu, instruction: IFormatInstruction) -> Result<(), ()> {
        cpu.registers[instruction.rd as usize] =
            cpu.registers[instruction.rs1 as usize].wrapping_add(instruction.imm as u64);
        Ok(())
    }

    pub fn load(cpu: &mut Cpu, instruction: IFormatInstruction) -> Result<(), ()> {
        let addr: u64 = cpu.registers[instruction.rs1 as usize].wrapping_add(instruction.imm);
        match instruction.funct3 {
            funct3::LoadInstructionOptypes::LB => match cpu.system_bus.load8(addr) {
                Ok(value) => {
                    cpu.registers[instruction.rd as usize] = value as i8 as i64 as u64;
                    Ok(())
                }
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LH => match cpu.system_bus.load16(addr) {
                Ok(value) => {
                    cpu.registers[instruction.rd as usize] = value as i16 as i64 as u64;
                    Ok(())
                }
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LW => match cpu.system_bus.load32(addr) {
                Ok(value) => {
                    cpu.registers[instruction.rd as usize] = value as i32 as i64 as u64;
                    Ok(())
                }
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LD => match cpu.system_bus.load64(addr) {
                Ok(value) => {
                    cpu.registers[instruction.rd as usize] = value;
                    Ok(())
                }
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LBU => match cpu.system_bus.load8(addr) {
                Ok(value) => Ok(cpu.registers[instruction.rd as usize] = value as u64),
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LHU => match cpu.system_bus.load16(addr) {
                Ok(value) => Ok(cpu.registers[instruction.rd as usize] = value as u64),
                Err(err) => Err(err),
            },
            funct3::LoadInstructionOptypes::LWU => match cpu.system_bus.load32(addr) {
                Ok(value) => Ok(cpu.registers[instruction.rd as usize] = value as u64),
                Err(err) => Err(err),
            },
            _ => Err(()),
        }
    }

    pub fn store(cpu: &mut Cpu, instruction: SFormatInstruction) -> Result<(), ()> {
        let addr: u64 = cpu.registers[instruction.rs1 as usize].wrapping_add(instruction.imm);
        match instruction.funct3 {
            funct3::StoreInstructionOptypes::SB => cpu.system_bus.store(
                addr,
                MemoryOpSize::B8,
                cpu.registers[instruction.rs2 as usize],
            ),
            funct3::StoreInstructionOptypes::SH => cpu.system_bus.store(
                addr,
                MemoryOpSize::B16,
                cpu.registers[instruction.rs2 as usize],
            ),
            funct3::StoreInstructionOptypes::SW => cpu.system_bus.store(
                addr,
                MemoryOpSize::B32,
                cpu.registers[instruction.rs2 as usize],
            ),
            funct3::StoreInstructionOptypes::SD => cpu.system_bus.store(
                addr,
                MemoryOpSize::B64,
                cpu.registers[instruction.rs2 as usize],
            ),
            _ => Err(()),
        }
    }
}
