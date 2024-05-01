use std::cmp::Ordering;

use crate::{
    consts::DRAM_BASE_ADDR,
    memory::{MemoryOpSize, SystemMemory},
};

pub type BusOpSize = MemoryOpSize;

pub struct SystemBus {
    system_memory: SystemMemory,
}

impl SystemBus {
    pub fn new(memory_size: u64, init_code: Vec<u8>) -> Self {
        Self {
            system_memory: SystemMemory::new(memory_size, init_code),
        }
    }

    // pub fn load(&self, addr: u64, size: BusOpSize) -> Result<u64, ()> {
    //     match addr.cmp(&DRAM_BASE_ADDR) {
    //         Ordering::Less => Err(()),
    //         _ => self.system_memory.load(addr - DRAM_BASE_ADDR, size),
    //     }
    // }
    
    pub fn load8(&self, addr: u64) -> Result<u8, ()> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(()),
            _ => Ok(self.system_memory.load8(addr - DRAM_BASE_ADDR)),
        }
    }

    pub fn load16(&self, addr: u64) -> Result<u16, ()> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(()),
            _ => Ok(self.system_memory.load16(addr - DRAM_BASE_ADDR)),
        }
    }
    
    pub fn load32(&self, addr: u64) -> Result<u32, ()> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(()),
            _ => self.system_memory.load32(addr - DRAM_BASE_ADDR),
        }
    }

    pub fn store(&mut self, addr: u64, size: BusOpSize, value: u64) -> Result<(), ()> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(()),
            _ => self.system_memory.store(addr - DRAM_BASE_ADDR, size, value),
        }
    }
}
