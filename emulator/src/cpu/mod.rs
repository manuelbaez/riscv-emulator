use crate::{
    consts::DRAM_BASE_ADDR,
    error::{AppErrors, AppResult},
    system_bus::SystemBus,
};

use self::instructions::{
    decoder::InstructionsDecoder, implementations::CpuInstructionsOpCodes, DEFAULT_ISNTRUCTION_SIZE_BYTES,
};

mod cs_registers;
mod instruction_excecutors;
mod instructions;

const CPU_REG_COUNT: usize = 32;

pub struct Cpu {
    registers: [u64; CPU_REG_COUNT],
    program_counter: u64,
    pub system_bus: SystemBus,
    cs_registers: [u64; 4096],
}

impl Cpu {
    pub fn new(memory_size: u64, init_code: Vec<u8>) -> Self {
        let mut cpu = Self {
            registers: [0_u64; 32],
            program_counter: DRAM_BASE_ADDR,
            system_bus: SystemBus::new(memory_size, init_code),
            cs_registers: [0_u64; 4096],
        };
        cpu.registers[0x02] = DRAM_BASE_ADDR + memory_size - 1;
        cpu
    }

    pub fn fetch_next_instruction(&mut self) -> AppResult<u32> {
        self.system_bus.load32(self.program_counter)
    }

    pub fn read_reg(&mut self, register: usize) -> AppResult<u64> {
        if register > CPU_REG_COUNT {
            return Err(AppErrors::OutOfBoundRegister);
        }
        Ok(self.registers[register])
    }

    pub fn write_reg(&mut self, register: usize, value: u64) -> AppResult<()> {
        if register == 0x0 {
            return Err(AppErrors::RegisterWriteProhibited);
        }
        Ok(self.registers[register] = value)
    }

    pub fn execute(&mut self, instruction: u32) -> AppResult<()> {
        // Increase the program counter to lookup the next instruction in the next cycle
        self.program_counter += DEFAULT_ISNTRUCTION_SIZE_BYTES as u64;

        match InstructionsDecoder::get_op_code(instruction) {
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE => {
                let decoded = InstructionsDecoder::decode_i_format_instruction(instruction);
                self.int_reg_immediate(decoded)
            }
            CpuInstructionsOpCodes::ADD => {
                let decoded = InstructionsDecoder::decode_r_format_instruction(instruction);
                self.add(decoded)
            }
            CpuInstructionsOpCodes::LOAD => {
                let decoded = InstructionsDecoder::decode_i_format_instruction(instruction);
                self.load(decoded)
            }
            CpuInstructionsOpCodes::STORE => {
                let decoded = InstructionsDecoder::decode_s_format_instruction(instruction);
                self.store(decoded)
            }
            _ => {
                dbg!("instruction not implemented");
                dbg!(instruction);
                Err(AppErrors::InstructionNotImplemented)
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
                    "x{:02}({})={:>#18x}\tx{:02}({})={:>#18x}\tx{:02}({})={:>#18x}\tx{:02}({})={:>#18x}",
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
