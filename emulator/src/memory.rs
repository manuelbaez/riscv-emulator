use std::cmp::Ordering;

pub struct SystemMemory {
    pub data: Vec<u8>,
}

pub enum MemoryOpSize {
    B8,
    B16,
    B32,
    B64,
}

impl SystemMemory {
    pub fn new(size_bytes: u64, initial_progam_code: Vec<u8>) -> Self {
        let mut data = vec![0; size_bytes as usize];
        data.splice(
            ..initial_progam_code.len(),
            initial_progam_code.iter().cloned(),
        );
        Self { data }
    }

    pub fn load(&self, addr: u64, size: MemoryOpSize) -> Result<u64, ()> {
        self.validate_mem_address(addr)
            .expect("Invalid memory address");
        match size {
            MemoryOpSize::B8 => self.load8(addr).map(|r| r as u64),
            MemoryOpSize::B16 => self.load16(addr).map(|r| r as u64),
            MemoryOpSize::B32 => self.load32(addr).map(|r| r as u64),
            MemoryOpSize::B64 => self.load64(addr),
        }
    }

    pub fn store(&mut self, addr: u64, size: MemoryOpSize, value: u64) -> Result<(), ()> {
        self.validate_mem_address(addr)
            .expect("Invalid memory address");
        match size {
            MemoryOpSize::B8 => Ok(self.store8(addr, value)),
            MemoryOpSize::B16 => Ok(self.store16(addr, value)),
            MemoryOpSize::B32 => Ok(self.store32(addr, value)),
            MemoryOpSize::B64 => Ok(self.store64(addr, value)),
        }
    }

    fn validate_mem_address(&self, addr: u64) -> Result<(), ()> {
        match (addr as usize).cmp(&self.data.len()) {
            Ordering::Less => Ok(()),
            _ => Err(()),
        }
    }

    pub fn load8(&self, addr: u64) -> Result<u8, ()> {
        match self.validate_mem_address(addr) {
            Ok(()) => {
                let index = addr as usize;
                Ok(self.data[index] as u8)
            }
            Err(err) => Err(err),
        }
    }

    pub fn load16(&self, addr: u64) -> Result<u16, ()> {
        match self.validate_mem_address(addr) {
            Ok(()) => {
                let index = addr as usize;
                Ok((self.data[index] as u16) | ((self.data[index + 1] as u16) << 8))
            }
            Err(err) => Err(err),
        }
    }

    pub fn load32(&self, addr: u64) -> Result<u32, ()> {
        match self.validate_mem_address(addr) {
            Ok(()) => {
                let index = addr as usize;
                Ok((self.data[index] as u32)
                    | ((self.data[index + 1] as u32) << 8)
                    | ((self.data[index + 2] as u32) << 16)
                    | ((self.data[index + 3] as u32) << 24))
            }
            Err(err) => Err(err),
        }
    }

    pub fn load64(&self, addr: u64) -> Result<u64, ()> {
        match self.validate_mem_address(addr) {
            Ok(()) => {
                let index = addr as usize;
                Ok((self.data[index] as u64)
                    | ((self.data[index + 1] as u64) << 8)
                    | ((self.data[index + 2] as u64) << 16)
                    | ((self.data[index + 3] as u64) << 24)
                    | ((self.data[index + 4] as u64) << 32)
                    | ((self.data[index + 5] as u64) << 40)
                    | ((self.data[index + 6] as u64) << 48)
                    | ((self.data[index + 7] as u64) << 56))
            }
            Err(err) => Err(err),
        }
    }

    fn store8(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.data[index] = (value & 0xff) as u8;
    }

    fn store16(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.data[index] = (value & 0xff) as u8;
        self.data[index + 1] = ((value >> 8) & 0xff) as u8;
    }

    fn store32(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.data[index] = (value & 0xff) as u8;
        self.data[index + 1] = ((value >> 8) & 0xff) as u8;
        self.data[index + 2] = ((value >> 16) & 0xff) as u8;
        self.data[index + 3] = ((value >> 24) & 0xff) as u8;
    }

    fn store64(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.data[index] = (value & 0xff) as u8;
        self.data[index + 1] = ((value >> 8) & 0xff) as u8;
        self.data[index + 2] = ((value >> 16) & 0xff) as u8;
        self.data[index + 3] = ((value >> 24) & 0xff) as u8;
        self.data[index + 4] = ((value >> 32) & 0xff) as u8;
        self.data[index + 5] = ((value >> 40) & 0xff) as u8;
        self.data[index + 6] = ((value >> 48) & 0xff) as u8;
        self.data[index + 7] = ((value >> 56) & 0xff) as u8;
    }
}
