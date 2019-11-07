#![allow(non_snake_case)]

use crate::cpu::CPU;
use crate::memory::Memory;

pub struct Atari {
    pub mCPU: CPU,
    //pub mMemory: Memory,
}

impl Atari {
    pub fn new() -> Atari {
        let memory = Memory::new();
        let cpu = CPU::new(memory);
        Atari {
            mCPU: cpu,
            //mMemory: memory,
        }
    }
}

pub fn add(a: i8, b: i8) -> i8 {
    a + b
}