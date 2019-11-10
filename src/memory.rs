#![allow(non_snake_case)]

use std::fs::File;
use std::io::Read;

pub const MEM_SIZE: usize = 65_536;

#[derive(Copy, Clone)]
pub struct Memory {
    bytes: [u8; MEM_SIZE],
}


impl Memory {
    pub fn new() -> Memory {
        Memory {
            bytes: [0; MEM_SIZE], //0x0000 to 0xFFFF}
        }
    }
    pub fn Get(&mut self, addr: &u16) -> u8 {
        let addr_copia1 = addr.clone() as usize;
        let addr_copia2 = addr.clone() as usize;

        let v = self.bytes[addr_copia1];
        if addr == &0xF004u16 {
            self.bytes[addr_copia2] = 0;
        }
        v
        //let addr1 = addr.clone() as usize;
        //println!("Get->addr1 {:#X} = {:#X}", addr1, self.bytes[addr1]);
        //self.bytes[addr1]
    }
    pub fn Set(&mut self, addr: &u16, val: u8) {
        let addr1 = addr.clone() as usize;
        if addr1 == 0xF001 {
            println!("val = {:#2X}", val);
        }
        self.bytes[addr1] = val;
    }

    pub fn GetW(&self, addr: &u16) -> u16 {
        let addr1 = addr.clone() as usize;
        //println!("addr1 = {}", addr1);
        //println!("self._bytes[addr1] = {}", self._bytes[addr1]);
        // don't assume running on little-endian
        ((self.bytes[addr1 + 1] as u16) << 8) + self.bytes[addr1] as u16
    }
    pub fn SetW(&mut self, addr: &u16, val: u16) {
//        let addr1 = addr.clone() as usize;
//        self.bytes[addr1] = (val as u8) & 0xFF;
//        self.bytes[&addr1 + 1] = (val >> 8) as u8;
        self.Set(addr, (val & 0xFF) as u8);
        self.Set(&(addr + 1), (val >> 8) as u8);
    }
    pub fn Load(&mut self, fileName: String, startAddr: u16) {
        // Lee el fichero ROM
        let mut f = File::open(fileName).expect("Fichero no encontrado");
        let mut fichero = Vec::<u8>::new(); // crea un Vec de u8
        f.read_to_end(&mut fichero).unwrap();

        // Pone el fichero en la memoria RAM
        //let mut mem = MEM::new(procesador);
        self.rellena_mem_desde_fichero(&fichero, startAddr);
        //mem.cierra_rom();
    }
    pub fn rellena_mem_desde_fichero(&mut self, file: &[u8], startAddr: u16) {
        //let bytes = &rom_file[..rom_file.len()];
        let mut i: u16 = startAddr;
        for &byte in file.iter() {
            //println!("{:#X} -> {:#X}", i, byte);
            self.escribe_byte_en_mem(i, byte);

            i = i.wrapping_add(1); // Incrementa 1 sin overflow
        }
    }

    pub fn escribe_byte_en_mem(&mut self, address: u16, valor: u8) {
        self.bytes[address as usize] = valor;
    }
}