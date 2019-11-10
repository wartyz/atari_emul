#![allow(non_snake_case)]

use crate::cpu::*;

pub struct Debugger {}

impl Debugger {
    pub fn DumpCPU(cpu: &mut CPU) {
        //print!("Dumping CPU state: ");
        print!("A = {:02X}, ({:03}) ", cpu.A, cpu.A);
        print!("X = {:02X}, ({:03}) ", cpu.X, cpu.X);
        print!("Y = {:02X}, ({:03}) ", cpu.Y, cpu.Y);
        print!("PC = {:04X}, ({:05}) ", cpu.PC, cpu.PC);
        print!("S = {:02X}, ({:03}) ", cpu.S, cpu.S);

        print!("Flags = ");
        if cpu.IsSetFlag(CPU::NEGATIVE_FLAG) { print!("{}", "N"); } else { print!("{}", "n"); }
        if cpu.IsSetFlag(CPU::OVERFLOW_FLAG) { print!("{}", "O"); } else { print!("{}", "o"); }
        if cpu.IsSetFlag(CPU::IGNORED_FLAG) { print!("{}", "X"); } else { print!("{}", "x"); }
        if cpu.IsSetFlag(CPU::BREAK_FLAG) { print!("{}", "B"); } else { print!("{}", "b"); }
        if cpu.IsSetFlag(CPU::DECIMAL_FLAG) { print!("{}", "D"); } else { print!("{}", "d"); }
        if cpu.IsSetFlag(CPU::INTERRUPT_FLAG) { print!("{}", "I"); } else { print!("{}", "i"); }
        if cpu.IsSetFlag(CPU::ZERO_FLAG) { print!("{}", "Z"); } else { print!("{}", "z"); }
        if cpu.IsSetFlag(CPU::CARRY_FLAG) { print!("{}", "C"); } else { print!("{}", "c"); }

        println!(" {:08b}", cpu.F);
    }
}

/*
pub const NEGATIVE_FLAG: flag_t = 0b1000_0000;
pub const OVERFLOW_FLAG: flag_t = 0b0100_0000;
pub const IGNORED_FLAG: flag_t = 0b0010_0000;
pub const BREAK_FLAG: flag_t = 0b0001_0000;
pub const DECIMAL_FLAG: flag_t = 0b0000_1000;
pub const INTERRUPT_FLAG: flag_t = 0b0000_0100;
pub const ZERO_FLAG: flag_t = 0b0000_0010;
pub const CARRY_FLAG: flag_t = 0b0000_0001;
*/