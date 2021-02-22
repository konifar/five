use crate::emulator::memory::Memory;

pub enum Xlen {
    Byte = 1,
    Halfword = 2,
    Word = 4,
}

#[derive(Default)]
pub struct SystemBus {
    pub memory: Memory,
}

impl SystemBus {
    pub fn load(&self, address: u32, xlen: Xlen) -> u32 {
        self.memory.load(address, xlen)
    }

    pub fn load8(&self, address: u32) -> u8 {
        self.load(address, Xlen::Byte) as u8
    }

    pub fn load16(&self, address: u32) -> u16 {
        self.load(address, Xlen::Halfword) as u16
    }

    pub fn load32(&self, address: u32) -> u32 {
        self.load(address, Xlen::Word) as u32
    }

    pub fn store(&mut self, address: u32, value: u32, xlen: Xlen) {
        self.memory.store(address, value, xlen);
    }

    pub fn store8(&mut self, address: u32, value: u8) {
        self.store(address, value as u32, Xlen::Byte)
    }

    pub fn store16(&mut self, address: u32, value: u16) {
        self.store(address, value as u32, Xlen::Halfword)
    }

    pub fn store32(&mut self, address: u32, value: u32) {
        self.store(address, value as u32, Xlen::Word)
    }
}
