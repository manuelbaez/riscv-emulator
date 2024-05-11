use std::cmp::Ordering;

use crate::{
    consts::DRAM_BASE_ADDR,
    error::{AppErrors, AppResult},
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

    #[allow(dead_code)]
    pub fn load(&self, addr: u64, size: BusOpSize) -> AppResult<u64> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(AppErrors::AddressNotFound),
            _ => self.system_memory.load(addr - DRAM_BASE_ADDR, size),
        }
    }

    #[inline(always)]
    pub fn load8(&self, addr: u64) -> AppResult<u8> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(AppErrors::AddressNotFound),
            _ => self.system_memory.load8(addr - DRAM_BASE_ADDR),
        }
    }

    #[inline(always)]
    pub fn load16(&self, addr: u64) -> AppResult<u16> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(AppErrors::AddressNotFound),
            _ => self.system_memory.load16(addr - DRAM_BASE_ADDR),
        }
    }

    #[inline(always)]
    pub fn load32(&self, addr: u64) -> AppResult<u32> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(AppErrors::AddressNotFound),
            _ => self.system_memory.load32(addr - DRAM_BASE_ADDR),
        }
    }

    #[inline(always)]
    pub fn load64(&self, addr: u64) -> AppResult<u64> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(AppErrors::AddressNotFound),
            _ => self.system_memory.load64(addr - DRAM_BASE_ADDR),
        }
    }

    #[inline(always)]
    pub fn store(&mut self, addr: u64, size: BusOpSize, value: u64) -> AppResult<()> {
        match addr.cmp(&DRAM_BASE_ADDR) {
            Ordering::Less => Err(AppErrors::AddressNotFound),
            _ => self.system_memory.store(addr - DRAM_BASE_ADDR, size, value),
        }
    }
}
