use crate::{consts::DRAM_BASE_ADDR, system_bus::SystemBus};

use self::{
    excecutors::CpuInstructionExecutors,
    instructions::{CpuInstructionsOpCodes, InstructionsDecoder, DEFAULT_ISNTRUCTION_SIZE_BYTES},
};

mod excecutors;
mod instructions;

pub struct Cpu {
    pub registers: [u64; 32],
    pub program_counter: u64,
    pub system_bus: SystemBus,
}

impl Cpu {
    pub fn new(memory_size: u64, init_code: Vec<u8>) -> Self {
        let mut cpu = Self {
            registers: [0_u64; 32],
            program_counter: DRAM_BASE_ADDR,
            system_bus: SystemBus::new(memory_size, init_code),
        };
        cpu.registers[0x02] = DRAM_BASE_ADDR + memory_size - 1;
        cpu
    }

    pub fn fetch_next_instruction(&mut self) -> Result<u32, ()> {
        self.system_bus.load32(self.program_counter)
    }

    pub fn execute(&mut self, instruction: u32) -> Result<(), ()> {
        // Increase the program counter to lookup the next instruction in the next cycle
        self.program_counter += DEFAULT_ISNTRUCTION_SIZE_BYTES as u64;

        match InstructionsDecoder::get_op_code(instruction) {
            CpuInstructionsOpCodes::ADDI => {
                let decoded = InstructionsDecoder::decode_i_format_instruction(instruction);
                CpuInstructionExecutors::addi(self, decoded)
            }
            CpuInstructionsOpCodes::ADD => {
                let decoded = InstructionsDecoder::decode_r_format_instruction(instruction);
                CpuInstructionExecutors::add(self, decoded)
            }
            CpuInstructionsOpCodes::LOAD => {
                let decoded = InstructionsDecoder::decode_i_format_instruction(instruction);
                CpuInstructionExecutors::load(self, decoded)
            }
            CpuInstructionsOpCodes::STORE => {
                let decoded = InstructionsDecoder::decode_s_format_instruction(instruction);
                CpuInstructionExecutors::store(self, decoded)
            }
            _ => {
                dbg!("instruction not implemented");
                dbg!(instruction);
                Err(())
            }
        }
    }

    pub fn dump_registers(&self) {
        let mut output = String::from("");
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                    i,
                    abi[i],
                    self.registers[i],
                    i + 1,
                    abi[i + 1],
                    self.registers[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.registers[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.registers[i + 3],
                )
            );
        }
        println!("{}", output);
    }
}
