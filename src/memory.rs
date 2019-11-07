#![allow(non_snake_case)]

pub const MEM_SIZE: usize = 65_536;

#[derive(Copy, Clone)]
pub struct Memory {
    _bytes: [u8; MEM_SIZE],
}


impl Memory {
    pub fn new() -> Memory {
        Memory {
            _bytes: [0; MEM_SIZE], //0x0000 to 0xFFFF}
        }
    }
    pub fn Get(&self, addr: &u16) -> u8 {
        let addr1 = addr.clone() as usize;
        self._bytes[addr1]
    }
    pub fn Set(&mut self, addr: &u16, val: u8) {
        let addr1 = addr.clone() as usize;
        self._bytes[addr1] = val;
    }

    pub fn GetW(&self, addr: &u16) -> u16 {
        let addr1 = addr.clone() as usize;
        //println!("addr1 = {}", addr1);
        //println!("self._bytes[addr1] = {}", self._bytes[addr1]);
        // don't assume running on little-endian
        ((self._bytes[addr1 + 1] as u16) << 8) + self._bytes[addr1] as u16
    }
    pub fn SetW(&mut self, addr: &u16, val: u16) {
        let addr1 = addr.clone() as usize;
        self._bytes[addr1] = (val as u8) & 0xFF;
        self._bytes[&addr1 + 1] = (val >> 8) as u8;
    }
}