use std::usize;

use super::Cpu;

pub struct MachineLevelCSRegisters;
impl MachineLevelCSRegisters {
    /// Machine status register.
    pub const MSTATUS: usize = 0x300;
    /// Machine exception delefation register.
    pub const MEDELEG: usize = 0x302;
    /// Machine interrupt delefation register.
    pub const MIDELEG: usize = 0x303;
    /// Machine interrupt-enable register.
    pub const MIE: usize = 0x304;
    /// Machine trap-handler base address.
    pub const MTVEC: usize = 0x305;
    /// Machine exception program counter.
    pub const MEPC: usize = 0x341;
    /// Machine trap cause.
    pub const MCAUSE: usize = 0x342;
    /// Machine bad address or instruction.
    pub const MTVAL: usize = 0x343;
    /// Machine interrupt pending.
    pub const MIP: usize = 0x344;
}

pub struct SupervisorLevelCSRegisters;
impl SupervisorLevelCSRegisters {
    /// Supervisor status register.
    pub const SSTATUS: usize = 0x100;
    /// Supervisor interrupt-enable register.
    pub const SIE: usize = 0x104;
    /// Supervisor trap handler base address.
    pub const STVEC: usize = 0x105;
    /// Supervisor exception program counter.
    pub const SEPC: usize = 0x141;
    /// Supervisor trap cause.
    pub const SCAUSE: usize = 0x142;
    /// Supervisor bad address or instruction.
    pub const STVAL: usize = 0x143;
    /// Supervisor interrupt pending.
    pub const SIP: usize = 0x144;
    /// Supervisor address translation and protection.
    pub const SATP: usize = 0x180;
}

impl Cpu {
    fn load_csr(&self, addr: usize) -> u64 {
        match addr {
            SupervisorLevelCSRegisters::SIE => {
                self.cs_registers[MachineLevelCSRegisters::MIE]
                    & self.cs_registers[MachineLevelCSRegisters::MIDELEG]
            }
            _ => self.cs_registers[addr],
        }
    }

    fn store_csr(&mut self, addr: usize, value: u64) {
        match addr {
            SupervisorLevelCSRegisters::SIE => {
                self.cs_registers[MachineLevelCSRegisters::MIE] = (self.cs_registers
                    [MachineLevelCSRegisters::MIE]
                    & !self.cs_registers[MachineLevelCSRegisters::MIDELEG])
                    | (value & self.cs_registers[MachineLevelCSRegisters::MIDELEG]);
            }
            _ => self.cs_registers[addr] = value,
        }
    }
}
