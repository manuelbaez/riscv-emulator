use crate::{
    consts::DRAM_BASE_ADDR,
    error::{AppErrors, AppResult},
    system_bus::SystemBus,
};

use self::{instructions::DEFAULT_INSTRUCTION_SIZE_BYTES, side_effects::OperationSideEffect};

mod cs_registers;
mod instruction_excecutors;
mod instructions;
pub mod side_effects;

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
    #[allow(dead_code)]
    #[inline(always)]
    pub fn read_reg(&mut self, register: usize) -> AppResult<u64> {
        if register > CPU_REG_COUNT {
            return Err(AppErrors::OutOfBoundRegister);
        }
        Ok(self.registers[register])
    }
    #[inline(always)]
    pub fn write_reg(&mut self, register: usize, value: u64) -> AppResult<OperationSideEffect> {
        self.registers[register] = value;
        self.registers[0] = 0;
        Ok(OperationSideEffect::None)
    }
    /// Increase the program counter to lookup the next instruction in the next cycle
    #[inline(always)]
    pub fn increase_program_counter(&mut self) {
        self.program_counter += DEFAULT_INSTRUCTION_SIZE_BYTES as u64;
    }

    #[inline(always)]
    pub fn get_program_counter(&mut self) -> u64 {
        self.program_counter
    }

    pub fn get_registers(&mut self) -> [u64; 32] {
        self.registers
    }
}
