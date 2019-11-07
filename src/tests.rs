/*
#[cfg(test)]
pub mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
//use super::*;
    #[test]
    fn test_add() {
        assert_eq!(super::atari::add(1, 2), 3);
    }

//    #[test]
//    fn test_bad_add() {
//        // This assert would fire and test will fail.
//        // Please note, that private functions can be tested too!
//        assert_eq!(bad_add(1, 2), 3);
//    }
}*/

#![allow(non_snake_case)]

use crate::cpu::*;
use crate::debugger::*;

pub struct Tests {}

impl Tests {
    pub fn Assert(mustBeTrue: bool) {
        if !mustBeTrue {
            println!("Fallo en Assert");
        } else {
            println!("Pasado Assert");
        }
    }
    pub fn CPUFlagsTest(cpu: &mut CPU) {
        cpu.SetFlag1(CPU::NEGATIVE_FLAG);
        println!("{}", cpu.IsSetFlag(CPU::CARRY_FLAG));
        cpu.SetFlag1(CPU::CARRY_FLAG);
        println!("{}", cpu.IsSetFlag(CPU::CARRY_FLAG));
        cpu.ClearFlag(CPU::CARRY_FLAG);
        println!("{}", cpu.IsSetFlag(CPU::CARRY_FLAG));
        println!("{}", cpu.IsSetFlag(CPU::NEGATIVE_FLAG));
    }
    pub fn ADC_iTest(cpu: &mut CPU) {
        println!("ADC_iTest");

        cpu.A = 2;
        cpu.F = 0;
        ADC(cpu, 3u8);
        Debugger::DumpCPU(cpu);

        cpu.A = 2;
        cpu.F = CPU::CARRY_FLAG;
        ADC(cpu, 3u8);
        Debugger::DumpCPU(cpu);

        cpu.A = 2;
        cpu.F = 0;
        ADC(cpu, 254u8);
        Debugger::DumpCPU(cpu);

        cpu.A = 2;
        cpu.F = 0;
        ADC(cpu, 253u8);
        Debugger::DumpCPU(cpu);

        cpu.A = 253;
        cpu.F = CPU::NEGATIVE_FLAG;
        ADC(cpu, 6u8);
        Debugger::DumpCPU(cpu);

        cpu.A = 125;
        cpu.F = CPU::CARRY_FLAG;
        ADC(cpu, 2u8);
        Debugger::DumpCPU(cpu);
    }
    pub fn OPTest(cpu: &mut CPU) {
        cpu.mMemory.Set(&0u16, 0x69);
        cpu.mMemory.Set(&1u16, 7);
        cpu.A = 1;
        cpu.SetFlag1(CPU::CARRY_FLAG);
        cpu.Execute();
        Debugger::DumpCPU(cpu);
    }
    pub fn AddrTest(cpu: &mut CPU) {
        cpu.Reset();
        cpu.mMemory.Set(&0u16, 1);
        println!("1");
        Tests::Assert(cpu.GetOP(&0u16, &Addressing::Immediate) == 1);
        println!("2");
        cpu.mMemory.SetW(&0u16, 0x2123);
        println!("3");
        cpu.mMemory.Set(&0x2123u16, 2);
        println!("4");
        Tests::Assert(cpu.GetOP(&0u16, &Addressing::Absolute) == 2);
        println!("5");
        cpu.mMemory.Set(&0u16, 0x30);
        println!("6");
        cpu.mMemory.Set(&0x30u16, 3);
        println!("7");
        Tests::Assert(cpu.GetOP(&0u16, &Addressing::ZeroPage) == 3);

        cpu.X = 2;
        cpu.mMemory.Set(&0u16, 0x40);
        cpu.mMemory.Set(&0x42u16, 4);
        println!("8");
        Tests::Assert(cpu.GetOP(&0u16, &Addressing::ZeroPageX) == 4);

        // wraparound
        cpu.X = 0xFF;
        cpu.mMemory.Set(&0u16, 0x80);
        cpu.mMemory.Set(&0x7Fu16, 5);
        println!("9");
        Tests::Assert(cpu.GetOP(&0u16, &Addressing::ZeroPageX) == 5);

        cpu.X = 0x10;
        cpu.mMemory.SetW(&0u16, 0x60F0);
        cpu.mMemory.Set(&0x6100u16, 6);
        println!("10");
        Tests::Assert(cpu.GetOP(&0u16, &Addressing::AbsoluteX) == 6);

        cpu.Y = 0x11;
        cpu.mMemory.SetW(&0u16, 0x60F1);
        cpu.mMemory.Set(&0x6102u16, 7);
        println!("11");
        Tests::Assert(cpu.GetOP(&0u16, &Addressing::AbsoluteY) == 7);

        cpu.Y = 6;
        cpu.mMemory.Set(&0u16, 0x43);
        cpu.mMemory.Set(&0x43u16, 0x53);
        cpu.mMemory.Set(&0x44u16, 0xE4);
        println!("12");
        Tests::Assert(cpu.mMemory.GetW(&0x43u16) == 0xE453);
        cpu.mMemory.Set(&0xE459u16, 8);
        println!("13");
        Tests::Assert(cpu.GetOP(&0u16, &Addressing::IndirectIndexed) == 8);

        cpu.X = 4;
        cpu.mMemory.Set(&0u16, 0x43);
        cpu.mMemory.SetW(&0x47u16, 0xE453);
        cpu.mMemory.Set(&0xE453u16, 9);
        println!("14");
        Tests::Assert(cpu.GetOP(&0u16, &Addressing::IndexedIndirect) == 9);
    }
}