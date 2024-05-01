use super::{
    instructions::{IFormatInstruction, RFormatInstruction},
    Cpu,
};

pub struct CpuInstructionExecutors;
impl CpuInstructionExecutors {
    pub fn add(cpu: &mut Cpu, instruction: RFormatInstruction) {
        cpu.registers[instruction.rd as usize] = cpu.registers[instruction.rs1 as usize]
            .wrapping_add(cpu.registers[instruction.rs2 as usize]);
    }
    pub fn addi(cpu: &mut Cpu, instruction: IFormatInstruction) {
        cpu.registers[instruction.rd as usize] =
            cpu.registers[instruction.rs1 as usize].wrapping_add(instruction.imm as u64);
    }
}
