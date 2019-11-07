#![allow(non_snake_case)]

use crate::memory::Memory;

type flag_t = u8;

#[derive(Copy, Clone)]
pub enum Addressing {
    None,
    Immediate,
    Absolute,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirect,
    IndirectIndexed,
    Accumulator,
    Relative,
    Indirect,
}

type OPFunction = fn(cpu: &mut CPU, &u16, Addressing);

#[derive(Copy, Clone)]
pub struct OpCode(// Ojo es una tupla
                  OPFunction,
                  Addressing,
                  u16,
                  u16,
);

impl OpCode {
    pub fn new(OPFunction: OPFunction, Addressing: Addressing, v: u16, cycles: u16) -> OpCode {
        let mut op = OpCode(
            OPFunction,
            Addressing,
            v,
            cycles,
        );
        op
    }
}


#[derive(Copy, Clone)]
pub struct CPU {
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub S: u8,
    pub PC: u16,
    pub F: u8,

    pub EC: u16, // "Exit counter"

    pub _cycles: u16,

    pub mMemory: Memory,

    pub opCodeMap: [OpCode; 256],

}

/*
N Negative
V Overflow
-
B Break
D Decimal (use BCD for arithmetics)
I Interrupt (IRQ disable)
Z Zero
C Carry
*/


impl CPU {
    pub const NEGATIVE_FLAG: flag_t = 0b1000_0000;
    pub const OVERFLOW_FLAG: flag_t = 0b0100_0000;
    pub const IGNORED_FLAG: flag_t = 0b0010_0000;
    pub const BREAK_FLAG: flag_t = 0b0001_0000;
    pub const DECIMAL_FLAG: flag_t = 0b0000_1000;
    pub const INTERRUPT_FLAG: flag_t = 0b0000_0100;
    pub const ZERO_FLAG: flag_t = 0b0000_0010;
    pub const CARRY_FLAG: flag_t = 0b0000_0001;


    //impl CPU {

    pub fn new(memory: Memory) -> CPU {
        let opCodeMap: [OpCode; 256] = [OpCode::new(fn_no_impl, Addressing::None, 0, 0); 256]; //
        // pba

        let mut cpu = CPU {
            A: 0,
            X: 0,
            Y: 0,
            S: 0xFF,
            PC: 0,
            F: 0,

            EC: 0,

            _cycles: 0,

            mMemory: memory,
            opCodeMap: opCodeMap,
        };

        cpu.InitializeOPCodes();
        //Reset(cpu: &mut CPU); //TODO: No hace falta
        cpu
    }
    pub fn Cycles(&mut self, cycles: u16) {
        self._cycles += cycles;
    }

    pub fn GetOP(&mut self, opIndex: &u16, adr: &Addressing) -> u8 { // suma mas carry inmediato
        let ops_clone = (*opIndex).clone(); // Creo clones yo
        let x = (self.X).clone() as u16;
        let y = (self.Y).clone() as u16;
        let a = (self.A).clone();

        match adr {
            Addressing::Accumulator => a,
            Addressing::Immediate => self.mMemory.Get(opIndex),
            Addressing::Absolute => {
                println!("opIndex = {}", opIndex); // ---------------
                println!("self.mMemory.GetW(opIndex) = {}", self.mMemory.GetW(opIndex)); // ---------------
                let addr: u16 = self.mMemory.GetW(opIndex);
                println!("addr = {}", addr); // ---------------
                self.mMemory.Get(&addr)
            }
            Addressing::ZeroPage => {
                let v1 = self.mMemory.Get(opIndex) as u16;
                self.mMemory.Get(&v1)
            }
            Addressing::ZeroPageX => {
                //let x = self.X.clone() as u16;
                let v1: u16 = self.mMemory.Get(&opIndex) as u16 + &x;
                self.mMemory.Get(&(&v1 & 0xFF))
            }
            Addressing::ZeroPageY => {
                //let y = self.Y.clone() as u16;
                let v1: u16 = self.mMemory.Get(&opIndex) as u16 + &y;
                self.mMemory.Get(&(&v1 & 0xFF))
            }
            Addressing::AbsoluteX => {
                //let x = self.X.clone() as u16;
                let baseAddr: u16 = self.mMemory.GetW(opIndex);
                let finalAddr = baseAddr + &x;
                if (baseAddr >> 8) != (finalAddr >> 8) {
                    self.Cycles(1);
                }
                self.mMemory.Get(&finalAddr)
            }
            Addressing::AbsoluteY => {
                //let y = self.Y.clone() as u16;
                let baseAddr: u16 = self.mMemory.GetW(opIndex);
                let finalAddr = baseAddr + &y;
                if (baseAddr >> 8) != (finalAddr >> 8) {
                    self.Cycles(1);
                }
                self.mMemory.Get(&finalAddr)
            }
            Addressing::IndexedIndirect => {
                //let x = self.X.clone() as u16;
                let v1 = self.mMemory.Get(opIndex) as u16;
                let v2 = v1 + &x;
                let addr: u16 = self.mMemory.GetW(&v2);
                self.mMemory.Get(&addr)
            }
            Addressing::IndirectIndexed => {
                let basePointer: u16 = self.mMemory.Get(&opIndex) as u16;
                let baseAddress: u16 = self.mMemory.GetW(&basePointer); // TODO no lo tengo claro
                let finalAddress = baseAddress + y;
                if (baseAddress >> 8) != (finalAddress >> 8) {
                    self.Cycles(1);
                }
                self.mMemory.Get(&finalAddress)
            }
            _ => {
                println!("Modo de direccionamiento no soportado");
                0
            }
        }
    }

    pub fn SetOP(&mut self, opIndex: &u16, adr: Addressing, val: u8) {
        let ops_clone = (*opIndex).clone(); // Creo clones yo
        let x = (self.X).clone() as u16;
        let y = (self.Y).clone() as u16;
        //let a = (self.A).clone();

        match adr {
            Addressing::Accumulator => { self.A = val }
            Addressing::Immediate => {
                self.mMemory.Set(opIndex, val);
            }
            Addressing::Absolute => {
                let addr: u16 = self.mMemory.GetW(opIndex);
                self.mMemory.Set(&addr, val);
            }
            Addressing::ZeroPage => {
                let v1 = self.mMemory.Get(opIndex) as u16;
                self.mMemory.Set(&v1, val);
            }
            Addressing::ZeroPageX => {
                let v1: u16 = self.mMemory.Get(&opIndex) as u16 + &x;
                self.mMemory.Set(&(&v1 & 0xFF), val);
            }
            Addressing::ZeroPageY => {
                let v1: u16 = self.mMemory.Get(&opIndex) as u16 + &y;
                self.mMemory.Set(&(&v1 & 0xFF), val);
            }
            Addressing::AbsoluteX => {
                let baseAddr: u16 = self.mMemory.GetW(opIndex);
                let finalAddr: u16 = baseAddr + &x;
                if (baseAddr >> 8) != (finalAddr >> 8) {
                    self.Cycles(1);
                }
                self.mMemory.Set(&finalAddr, val);
            }
            Addressing::AbsoluteY => {
                let baseAddr: u16 = self.mMemory.GetW(opIndex);
                let finalAddr = baseAddr + &y;
                if (baseAddr >> 8) != (finalAddr >> 8) {
                    self.Cycles(1);
                }
                self.mMemory.Set(&finalAddr, val);
            }
            Addressing::IndexedIndirect => {
                //let x = self.X.clone() as u16;

                let baseAddr: u16 = (self.mMemory.Get(opIndex) as u16 + x) & 0xFF;
                let loTarget: u16 = self.mMemory.Get(&baseAddr) as u16;
                let hiTarget: u16 = self.mMemory.Get(&((&baseAddr + 1) as u16 & 0xFF)) as u16;
                let addr: u16 = loTarget + (hiTarget << 8);

                self.mMemory.Set(&addr, val);
            }
            Addressing::IndirectIndexed => {
                let basePointer: u16 = self.mMemory.Get(&opIndex) as u16;
                let loTarget: u16 = self.mMemory.Get(&basePointer) as u16;
                let hiTarget: u16 = self.mMemory.Get(&((&basePointer + 1) as u16 & 0xFF)) as u16;
                let baseAddress: u16 = loTarget + (hiTarget << 8);
                let finalAddress = baseAddress + y;
                if (baseAddress >> 8) != (finalAddress >> 8) {
                    self.Cycles(1);
                }
                self.mMemory.Set(&finalAddress, val);
            }
            _ => {
                println!("Modo de direccionamiento no soportado");
            }
        }
    }

    pub fn ClearFlag(&mut self, flag: flag_t) {
        self.F &= !flag;
    }

    pub fn IsSetFlag(&mut self, flag: flag_t) -> bool {
        &self.F & flag != 0
    }


    pub fn IsNegative(&mut self, op: &u8) -> bool {
        (op & 0b1000_0000) != 0
    }

    pub fn IsZero(&mut self, op: &u8) -> bool {
        op.clone() == 0
    }
    pub fn InitializeOPCodes(&mut self) {
        // ADC
        self.opCodeMap[0x69] = OpCode::new(opADC, Addressing::Immediate, 2, 2);
        self.opCodeMap[0x65] = OpCode::new(opADC, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0x75] = OpCode::new(opADC, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0x6D] = OpCode::new(opADC, Addressing::Absolute, 3, 4);
        self.opCodeMap[0x7D] = OpCode::new(opADC, Addressing::AbsoluteX, 3, 4);
        self.opCodeMap[0x79] = OpCode::new(opADC, Addressing::AbsoluteY, 3, 4);
        self.opCodeMap[0x61] = OpCode::new(opADC, Addressing::IndexedIndirect, 2, 6);
        self.opCodeMap[0x71] = OpCode::new(opADC, Addressing::IndirectIndexed, 2, 5);

        // AND
        self.opCodeMap[0x29] = OpCode::new(opAND, Addressing::Immediate, 2, 2);
        self.opCodeMap[0x25] = OpCode::new(opAND, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0x35] = OpCode::new(opAND, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0x2D] = OpCode::new(opAND, Addressing::Absolute, 3, 4);
        self.opCodeMap[0x3D] = OpCode::new(opAND, Addressing::AbsoluteX, 3, 4);
        self.opCodeMap[0x39] = OpCode::new(opAND, Addressing::AbsoluteY, 3, 4);
        self.opCodeMap[0x21] = OpCode::new(opAND, Addressing::IndexedIndirect, 2, 6);
        self.opCodeMap[0x31] = OpCode::new(opAND, Addressing::IndirectIndexed, 2, 5);

        // ASL
        self.opCodeMap[0x0A] = OpCode::new(opASL, Addressing::Accumulator, 1, 2);
        self.opCodeMap[0x06] = OpCode::new(opASL, Addressing::ZeroPage, 2, 5);
        self.opCodeMap[0x16] = OpCode::new(opASL, Addressing::ZeroPageX, 2, 6);
        self.opCodeMap[0x0E] = OpCode::new(opASL, Addressing::Absolute, 3, 6);
        self.opCodeMap[0x1E] = OpCode::new(opASL, Addressing::AbsoluteX, 3, 7);

        // branching
        self.opCodeMap[0x90] = OpCode::new(opBCC, Addressing::Relative, 2, 2);
        self.opCodeMap[0xB0] = OpCode::new(opBCS, Addressing::Relative, 2, 2);
        self.opCodeMap[0xF0] = OpCode::new(opBEQ, Addressing::Relative, 2, 2);
        self.opCodeMap[0x30] = OpCode::new(opBMI, Addressing::Relative, 2, 2);
        self.opCodeMap[0xD0] = OpCode::new(opBNE, Addressing::Relative, 2, 2);
        self.opCodeMap[0x10] = OpCode::new(opBPL, Addressing::Relative, 2, 2);
        self.opCodeMap[0x50] = OpCode::new(opBVC, Addressing::Relative, 2, 2);
        self.opCodeMap[0x70] = OpCode::new(opBVS, Addressing::Relative, 2, 2);

        // BIT
        self.opCodeMap[0x24] = OpCode::new(opBIT, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0x2C] = OpCode::new(opBIT, Addressing::Absolute, 3, 4);


        // BRK
        self.opCodeMap[0x00] = OpCode::new(opBRK, Addressing::None, 1, 7);

        // clear flags
        self.opCodeMap[0x18] = OpCode::new(opCLC, Addressing::None, 1, 2);
        self.opCodeMap[0xD8] = OpCode::new(opCLD, Addressing::None, 1, 2);
        self.opCodeMap[0x58] = OpCode::new(opCLI, Addressing::None, 1, 2);
        self.opCodeMap[0xB8] = OpCode::new(opCLV, Addressing::None, 1, 2);

        // comparisons
        self.opCodeMap[0xC9] = OpCode::new(opCMP, Addressing::Immediate, 2, 2);
        self.opCodeMap[0xC5] = OpCode::new(opCMP, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0xD5] = OpCode::new(opCMP, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0xCD] = OpCode::new(opCMP, Addressing::Absolute, 3, 4);
        self.opCodeMap[0xDD] = OpCode::new(opCMP, Addressing::AbsoluteX, 3, 4);
        self.opCodeMap[0xD9] = OpCode::new(opCMP, Addressing::AbsoluteY, 3, 4);
        self.opCodeMap[0xC1] = OpCode::new(opCMP, Addressing::IndexedIndirect, 2, 6);
        self.opCodeMap[0xD1] = OpCode::new(opCMP, Addressing::IndirectIndexed, 2, 5);
        self.opCodeMap[0xE0] = OpCode::new(opCPX, Addressing::Immediate, 2, 2);
        self.opCodeMap[0xE4] = OpCode::new(opCPX, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0xEC] = OpCode::new(opCPX, Addressing::Absolute, 3, 4);
        self.opCodeMap[0xC0] = OpCode::new(opCPY, Addressing::Immediate, 2, 2);
        self.opCodeMap[0xC4] = OpCode::new(opCPY, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0xCC] = OpCode::new(opCPY, Addressing::Absolute, 3, 4);

        // increment/decrement
        self.opCodeMap[0xE6] = OpCode::new(opINC, Addressing::ZeroPage, 2, 5);
        self.opCodeMap[0xF6] = OpCode::new(opINC, Addressing::ZeroPageX, 2, 6);
        self.opCodeMap[0xEE] = OpCode::new(opINC, Addressing::Absolute, 3, 6);
        self.opCodeMap[0xFE] = OpCode::new(opINC, Addressing::AbsoluteX, 3, 7);
        self.opCodeMap[0xE8] = OpCode::new(opINX, Addressing::None, 1, 2);
        self.opCodeMap[0xC8] = OpCode::new(opINY, Addressing::None, 1, 2);
        self.opCodeMap[0xC6] = OpCode::new(opDEC, Addressing::ZeroPage, 2, 5);
        self.opCodeMap[0xD6] = OpCode::new(opDEC, Addressing::ZeroPageX, 2, 6);
        self.opCodeMap[0xCE] = OpCode::new(opDEC, Addressing::Absolute, 3, 6);
        self.opCodeMap[0xDE] = OpCode::new(opDEC, Addressing::AbsoluteX, 3, 7);
        self.opCodeMap[0xCA] = OpCode::new(opDEX, Addressing::None, 1, 2);
        self.opCodeMap[0x88] = OpCode::new(opDEY, Addressing::None, 1, 2);

        // EOR
        self.opCodeMap[0x49] = OpCode::new(opEOR, Addressing::Immediate, 2, 2);
        self.opCodeMap[0x45] = OpCode::new(opEOR, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0x55] = OpCode::new(opEOR, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0x4D] = OpCode::new(opEOR, Addressing::Absolute, 3, 4);
        self.opCodeMap[0x5D] = OpCode::new(opEOR, Addressing::AbsoluteX, 3, 4);
        self.opCodeMap[0x59] = OpCode::new(opEOR, Addressing::AbsoluteY, 3, 4);
        self.opCodeMap[0x41] = OpCode::new(opEOR, Addressing::IndexedIndirect, 2, 6);
        self.opCodeMap[0x51] = OpCode::new(opEOR, Addressing::IndirectIndexed, 2, 5);

        // JMP
        self.opCodeMap[0x4C] = OpCode::new(opJMP, Addressing::Absolute, 3, 3);
        self.opCodeMap[0x6C] = OpCode::new(opJMP, Addressing::Indirect, 3, 5);

        // subroutine
        self.opCodeMap[0x20] = OpCode::new(opJSR, Addressing::Absolute, 3, 6);
        self.opCodeMap[0x60] = OpCode::new(opRTS, Addressing::Absolute, 1, 6);

        // load
        self.opCodeMap[0xA9] = OpCode::new(opLDA, Addressing::Immediate, 2, 2);
        self.opCodeMap[0xA5] = OpCode::new(opLDA, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0xB5] = OpCode::new(opLDA, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0xAD] = OpCode::new(opLDA, Addressing::Absolute, 3, 4);
        self.opCodeMap[0xBD] = OpCode::new(opLDA, Addressing::AbsoluteX, 3, 4);
        self.opCodeMap[0xB9] = OpCode::new(opLDA, Addressing::AbsoluteY, 3, 4);
        self.opCodeMap[0xA1] = OpCode::new(opLDA, Addressing::IndexedIndirect, 2, 6);
        self.opCodeMap[0xB1] = OpCode::new(opLDA, Addressing::IndirectIndexed, 2, 5);
        self.opCodeMap[0xA2] = OpCode::new(opLDX, Addressing::Immediate, 2, 2);
        self.opCodeMap[0xA6] = OpCode::new(opLDX, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0xB6] = OpCode::new(opLDX, Addressing::ZeroPageY, 2, 4);
        self.opCodeMap[0xAE] = OpCode::new(opLDX, Addressing::Absolute, 3, 4);
        self.opCodeMap[0xBE] = OpCode::new(opLDX, Addressing::AbsoluteY, 3, 4);
        self.opCodeMap[0xA0] = OpCode::new(opLDY, Addressing::Immediate, 2, 2);
        self.opCodeMap[0xA4] = OpCode::new(opLDY, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0xB4] = OpCode::new(opLDY, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0xAC] = OpCode::new(opLDY, Addressing::Absolute, 3, 4);
        self.opCodeMap[0xBC] = OpCode::new(opLDY, Addressing::AbsoluteX, 3, 4);

        // LSR
        self.opCodeMap[0x4A] = OpCode::new(opLSR, Addressing::Accumulator, 1, 2);
        self.opCodeMap[0x46] = OpCode::new(opLSR, Addressing::ZeroPage, 2, 5);
        self.opCodeMap[0x56] = OpCode::new(opLSR, Addressing::ZeroPageX, 2, 6);
        self.opCodeMap[0x4E] = OpCode::new(opLSR, Addressing::Absolute, 3, 6);
        self.opCodeMap[0x5E] = OpCode::new(opLSR, Addressing::AbsoluteX, 3, 7);

        // NOP
        self.opCodeMap[0xEA] = OpCode::new(opNOP, Addressing::None, 1, 2);

        // ORA
        self.opCodeMap[0x09] = OpCode::new(opORA, Addressing::Immediate, 2, 2);
        self.opCodeMap[0x05] = OpCode::new(opORA, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0x15] = OpCode::new(opORA, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0x0D] = OpCode::new(opORA, Addressing::Absolute, 3, 4);
        self.opCodeMap[0x1D] = OpCode::new(opORA, Addressing::AbsoluteX, 3, 4);
        self.opCodeMap[0x19] = OpCode::new(opORA, Addressing::AbsoluteY, 3, 4);
        self.opCodeMap[0x01] = OpCode::new(opORA, Addressing::IndexedIndirect, 2, 6);
        self.opCodeMap[0x11] = OpCode::new(opORA, Addressing::IndirectIndexed, 2, 5);

        // push/pull
        self.opCodeMap[0x48] = OpCode::new(opPHA, Addressing::IndirectIndexed, 1, 3);
        self.opCodeMap[0x08] = OpCode::new(opPHP, Addressing::IndirectIndexed, 1, 3);
        self.opCodeMap[0x68] = OpCode::new(opPLA, Addressing::IndirectIndexed, 1, 4);
        self.opCodeMap[0x28] = OpCode::new(opPLP, Addressing::IndirectIndexed, 1, 4);

        // ROL
        self.opCodeMap[0x2A] = OpCode::new(opROL, Addressing::Accumulator, 1, 2);
        self.opCodeMap[0x26] = OpCode::new(opROL, Addressing::ZeroPage, 2, 5);
        self.opCodeMap[0x36] = OpCode::new(opROL, Addressing::ZeroPageX, 2, 6);
        self.opCodeMap[0x2E] = OpCode::new(opROL, Addressing::Absolute, 3, 6);
        self.opCodeMap[0x3E] = OpCode::new(opROL, Addressing::AbsoluteX, 3, 7);

        // ROR
        self.opCodeMap[0x6A] = OpCode::new(opROR, Addressing::Accumulator, 1, 2);
        self.opCodeMap[0x66] = OpCode::new(opROR, Addressing::ZeroPage, 2, 5);
        self.opCodeMap[0x76] = OpCode::new(opROR, Addressing::ZeroPageX, 2, 6);
        self.opCodeMap[0x6E] = OpCode::new(opROR, Addressing::Absolute, 3, 6);
        self.opCodeMap[0x7E] = OpCode::new(opROR, Addressing::AbsoluteX, 3, 7);

        // RTI
        self.opCodeMap[0x40] = OpCode::new(opRTI, Addressing::None, 1, 6);

        // SBC
        self.opCodeMap[0xE9] = OpCode::new(opSBC, Addressing::Immediate, 2, 2);
        self.opCodeMap[0xE5] = OpCode::new(opSBC, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0xF5] = OpCode::new(opSBC, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0xED] = OpCode::new(opSBC, Addressing::Absolute, 3, 4);
        self.opCodeMap[0xFD] = OpCode::new(opSBC, Addressing::AbsoluteX, 3, 4);
        self.opCodeMap[0xF9] = OpCode::new(opSBC, Addressing::AbsoluteY, 3, 4);
        self.opCodeMap[0xE1] = OpCode::new(opSBC, Addressing::IndexedIndirect, 2, 6);
        self.opCodeMap[0xF1] = OpCode::new(opSBC, Addressing::IndirectIndexed, 2, 5);

        // set flags
        self.opCodeMap[0x38] = OpCode::new(opSEC, Addressing::None, 1, 2);
        self.opCodeMap[0xF8] = OpCode::new(opSED, Addressing::None, 1, 2);
        self.opCodeMap[0x78] = OpCode::new(opSEI, Addressing::None, 1, 2);

        // store
        self.opCodeMap[0x85] = OpCode::new(opSTA, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0x95] = OpCode::new(opSTA, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0x8D] = OpCode::new(opSTA, Addressing::Absolute, 3, 4);
        self.opCodeMap[0x9D] = OpCode::new(opSTA, Addressing::AbsoluteX, 3, 5);
        self.opCodeMap[0x99] = OpCode::new(opSTA, Addressing::AbsoluteY, 3, 5);
        self.opCodeMap[0x81] = OpCode::new(opSTA, Addressing::IndexedIndirect, 2, 6);
        self.opCodeMap[0x91] = OpCode::new(opSTA, Addressing::IndirectIndexed, 2, 6);
        self.opCodeMap[0x86] = OpCode::new(opSTX, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0x96] = OpCode::new(opSTX, Addressing::ZeroPageY, 2, 4);
        self.opCodeMap[0x8E] = OpCode::new(opSTX, Addressing::Absolute, 3, 4);
        self.opCodeMap[0x84] = OpCode::new(opSTY, Addressing::ZeroPage, 2, 3);
        self.opCodeMap[0x94] = OpCode::new(opSTY, Addressing::ZeroPageX, 2, 4);
        self.opCodeMap[0x8C] = OpCode::new(opSTY, Addressing::Absolute, 3, 4);

        // transfer
        self.opCodeMap[0xAA] = OpCode::new(opTAX, Addressing::None, 1, 2);
        self.opCodeMap[0xA8] = OpCode::new(opTAY, Addressing::None, 1, 2);
        self.opCodeMap[0xBA] = OpCode::new(opTSX, Addressing::None, 1, 2);
        self.opCodeMap[0x8A] = OpCode::new(opTXA, Addressing::None, 1, 2);
        self.opCodeMap[0x9A] = OpCode::new(opTXS, Addressing::None, 1, 2);
        self.opCodeMap[0x98] = OpCode::new(opTYA, Addressing::None, 1, 2);
    }
    pub fn Reset(&mut self) {
        self.A = 0;
        self.X = 0;
        self.Y = 0;
        self.S = 0xFF;
        self.PC = 0;
        self.F = 0;
    }


    pub fn SetFlag1(&mut self, flag: flag_t) {
        self.F |= flag;
    }

    pub fn SetFlag2(&mut self, flag: flag_t, isSet: bool) {
        if isSet {
            self.SetFlag1(flag);
        } else {
            self.ClearFlag(flag);
        }
    }

    pub fn StackPush(&mut self, val: u8) {
        // El stack en el 6502 siempre está en la página 0x100-0x1FF
        let S_copia_u16 = 0x100u16 + self.S.clone() as u16;
        self.mMemory.Set(&S_copia_u16, val);
        self.S = &self.S - 1;

        /*	if (S == 0) // wasteful??
        {
            throw std::runtime_error("Stack overflow!");
        }*/
    }
    pub fn StackPull(&mut self) -> u8 {
        if self.S < 0xFF {
            let S_copia_u16 = 0x100u16 + self.S.clone() as u16;
            let val = self.mMemory.Get(&S_copia_u16);
            self.S = &self.S + 1;
            val
        } else {
            panic!("Stack vacio!");
        }
    }
    pub fn EntryPoint(&mut self, startAddr: u16, endAddr: u16) {
        self.PC = startAddr;
        self.EC = endAddr;
    }
    pub fn Execute(&mut self) {
        println!("En Execute -------------"); // -----------------------------------
        let code: usize = self.mMemory.Get(&self.PC) as usize;
        let opCode = self.opCodeMap[code];
        if opCode.2 > 0 { // si número de bytes de la instrucción > 0

            let adr = opCode.1; // modo de direccionamiento
            let func = opCode.0; // función a ejecutar
            let opIndex = &self.PC + 1;

            // PC already points to next instruction
            self.PC += opCode.2;
            func(self, &opIndex, adr); // Ejecutar instrucción

            if self.PC == (opIndex - 1) {
                // jump to self: trap
                //throw std::runtime_error("Trap!");
            }
            if self.PC == self.EC { // Llegó al final
                panic!("¡Final incorrecto!");
            }
            self.Cycles(opCode.3); // aumenta valor de cycles
        } else {
            panic!("OPcode no soportado");
        }
    }
}
/***********************************************************/
/************* Funciones llamadas por puntero **************/
/***********************************************************/
pub fn fn_no_impl(cpu: &mut CPU, valor: &u16, Addressing: Addressing) {}

// ----------------------------------------------------------------------
// ADC Add Memory to Accumulator with Carry
// A + M + C -> A, C                N Z C I D V
//                                  + + + - - +
// ----------------------------------------------------------------------
pub fn ADC(cpu: &mut CPU, op: u8) { // suma
    if cpu.IsSetFlag(CPU::DECIMAL_FLAG) {
        let lhr = (cpu.A & 15) + (cpu.A >> 4) * 10;
        let rhr = (op & 15) + (op >> 4) * 10;
        let mut res = lhr + rhr;
        cpu.SetFlag2(CPU::OVERFLOW_FLAG, res > 99);
        res %= 100; // ???
        cpu.A = res;
        let v1: bool = cpu.IsNegative(&res);
        cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

        let v2: bool = cpu.IsZero(&res);
        cpu.SetFlag2(CPU::ZERO_FLAG, v2);
    } else {
        let mut result: i16 = cpu.A.clone() as i16;
        result += op.clone() as i16;

        if cpu.IsSetFlag(CPU::CARRY_FLAG) {
            result += 1;
        }
        let res: u8 = result as u8 & 0xFF;
        cpu.SetFlag2(CPU::CARRY_FLAG, result > 255);

        let v: bool = cpu.IsNegative(&res);
        cpu.SetFlag2(CPU::NEGATIVE_FLAG, v);

        let v: bool = cpu.IsZero(&res);
        cpu.SetFlag2(CPU::ZERO_FLAG, v);

        let v0 = cpu.A.clone();
        let v1: bool = cpu.IsNegative(&v0);
        let v2: bool = cpu.IsNegative(&res);
        cpu.SetFlag2(CPU::OVERFLOW_FLAG, v1 != v2);

        cpu.A = res;
    }
}

pub fn opADC(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let v1: u8 = cpu.GetOP(opIndex, &adr);
    ADC(cpu, v1);
}

// ----------------------------------------------------------------------
// AND  AND Memory with Accumulator
// A AND M -> A                     N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn AND(cpu: &mut CPU, op: u8) {
    let mut a = cpu.A.clone() & op; // operación AND

    let v1: bool = cpu.IsNegative(&a);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&a);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);
}

pub fn opAND(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let v1 = cpu.GetOP(opIndex, &adr);
    AND(cpu, v1);
}

// ----------------------------------------------------------------------
// ASL shift Left One Bit (Memoria o Acumulador)
// C <- [76543210] <- 0             N Z C I D V
//                                  + + + - - -
// ----------------------------------------------------------------------
pub fn ASL(cpu: &mut CPU, op1: u8) -> u8 {
    let res: u16 = (op1 << 1) as u16; // creo que debe ser u16
    cpu.SetFlag2(CPU::CARRY_FLAG, (res & 0x100) != 0);
    let op2 = (res & 0xFF) as u8;

    let v1: bool = cpu.IsNegative(&op2);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&op2);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);

    op2
}

pub fn opASL(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let adr1 = adr.clone();
    let v1 = cpu.GetOP(&opIndex, &adr1);
    let v2 = ASL(cpu, v1);
    cpu.SetOP(&opIndex, adr, v2);
}

// ----------------------------------------------------------------------
// Funcion auxiliar para saltos
// ----------------------------------------------------------------------
pub fn Branch(cpu: &mut CPU, opIndex: &u16, condition: bool) {
    if condition {
        let salto = cpu.mMemory.Get(&opIndex) as i16;
        let pci = (cpu.PC).clone() as i16;
        let pc = (pci + salto) as u16;
        cpu.PC = pc; // Creo que sbyte_t es byte con signo
    }
}

// ----------------------------------------------------------------------
// BCC Branch on Carry Clear
// salto si C = 0                  N Z C I D V
//                                 - - - - - -
// ----------------------------------------------------------------------

pub fn opBCC(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let condicion: bool = cpu.IsSetFlag(CPU::CARRY_FLAG);
    Branch(cpu, opIndex, !condicion);
}

// ----------------------------------------------------------------------
// BCS Branch on Carry Set
// salto si C = 1                  N Z C I D V
//                                 - - - - - -
// ----------------------------------------------------------------------
pub fn opBCS(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let condicion: bool = cpu.IsSetFlag(CPU::CARRY_FLAG);
    Branch(cpu, opIndex, condicion);
}

// ----------------------------------------------------------------------
// BEQ Branch on Result Zero
// salto si Z = 1                  N Z C I D V
//                                 - - - - - -
// ----------------------------------------------------------------------
pub fn opBEQ(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let condicion: bool = cpu.IsSetFlag(CPU::ZERO_FLAG);
    Branch(cpu, opIndex, condicion);
}

// ----------------------------------------------------------------------
// BIT Test Bits in Memory with Accumulator
// bits 7 y 6 del operando son transferidos a los bits 7 y 6 de SR (N,V);
//     the zeroflag is set to the result of operand AND accumulator.
//
//     A AND M, M7 -> N, M6 -> V       N  Z C I D V
//                                     M7 + - - - M6
// ----------------------------------------------------------------------
pub fn opBIT(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let op: u8 = cpu.GetOP(&opIndex, &adr);
    let v1: bool = cpu.IsNegative(&op);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);
    cpu.SetFlag2(CPU::OVERFLOW_FLAG, (op & 0b01000000) != 0);
    let res: u8 = (cpu.A & op);
    let v2: bool = cpu.IsZero(&res);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);
}

// ----------------------------------------------------------------------
// BMI Branch on Result Minus
// salto si N = 1                  N Z C I D V
//                                 - - - - - -
// ----------------------------------------------------------------------
pub fn opBMI(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let condicion: bool = cpu.IsSetFlag(CPU::NEGATIVE_FLAG);
    Branch(cpu, opIndex, condicion);
}

// ----------------------------------------------------------------------
// BNE Branch on Result not Zero
// salto si Z = 0                  N Z C I D V
//                                 - - - - - -
// ----------------------------------------------------------------------
pub fn opBNE(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let condicion: bool = cpu.IsSetFlag(CPU::ZERO_FLAG);
    Branch(cpu, opIndex, !condicion);
}

// ----------------------------------------------------------------------
// BPL Branch on Result Plus
// salto si N = 0                  N Z C I D V
//                                 - - - - - -
// ----------------------------------------------------------------------
pub fn opBPL(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let condicion: bool = cpu.IsSetFlag(CPU::NEGATIVE_FLAG);
    Branch(cpu, opIndex, !condicion);
}

// ----------------------------------------------------------------------
// BRK  Force Break
// interrupt,                       N Z C I D V
// push PC+2, push SR               - - - 1 - -
// ----------------------------------------------------------------------
pub fn opBRK(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.PC += 1;
    let v1 = (&cpu.PC >> 8) as u8;
    cpu.StackPush(v1);
    cpu.StackPush((&cpu.PC & 0xFF) as u8);
    let mut f = (cpu.F).clone();
    f |= CPU::BREAK_FLAG;
    cpu.StackPush(f);
    cpu.SetFlag1(CPU::INTERRUPT_FLAG);
    cpu.PC = cpu.mMemory.GetW(&0xFFFEu16); // jump to vector
}

// ----------------------------------------------------------------------
// BVC Branch on Overflow Clear
// salto si V = 0                  N Z C I D V
//                                 - - - - - -
// ----------------------------------------------------------------------
pub fn opBVC(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let condicion: bool = cpu.IsSetFlag(CPU::OVERFLOW_FLAG);
    Branch(cpu, opIndex, !condicion);
}

// ----------------------------------------------------------------------
// BVS Branch on Overflow Set
// salto si V = 1                  N Z C I D V
//                                 - - - - - -
// ----------------------------------------------------------------------
pub fn opBVS(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let condicion: bool = cpu.IsSetFlag(CPU::OVERFLOW_FLAG);
    Branch(cpu, opIndex, condicion);
}
// ----------------------------------------------------------------------
// CLC Clear Carry Flag
//     0 -> C                           N Z C I D V
//                                      - - 0 - - -
// ----------------------------------------------------------------------

pub fn opCLC(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.ClearFlag(CPU::CARRY_FLAG);
}

// ----------------------------------------------------------------------
// CLD Clear Decimal Mode
//     0 -> D                           N Z C I D V
//                                      - - - - 0 -
// ----------------------------------------------------------------------
pub fn opCLD(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.ClearFlag(CPU::DECIMAL_FLAG);
}

// ----------------------------------------------------------------------
// CLI Clear Interrupt Disable Bit
//     0 -> I                           N Z C I D V
//                                      - - - 0 - -
// ----------------------------------------------------------------------
pub fn opCLI(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.ClearFlag(CPU::INTERRUPT_FLAG);
}

// ----------------------------------------------------------------------
// CLV  Clear Overflow Flag
//     0 -> V                           N Z C I D V
//                                      - - - - - 0
// ----------------------------------------------------------------------
pub fn opCLV(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.ClearFlag(CPU::OVERFLOW_FLAG);
}

// ----------------------------------------------------------------------
// Función auxiliar para comparaciones
// ----------------------------------------------------------------------
pub fn Compare(cpu: &mut CPU, r: &u8, op: u8) {
    let r_copia = r.clone();
    let res = ((r_copia - op) & 0xFF) as u8; // TODO: no entiendo esta conversion

    let v1: bool = cpu.IsZero(&res);
    cpu.SetFlag2(CPU::ZERO_FLAG, v1);

    let v2: bool = cpu.IsNegative(&res);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
    cpu.SetFlag2(CPU::CARRY_FLAG, op <= *r);
}

// ----------------------------------------------------------------------
// CMP Compara memoria con A
// A - M                            N Z C I D V
//                                  + + + - - -
// ----------------------------------------------------------------------
pub fn opCMP(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let A_copia = cpu.A.clone();
    let op = cpu.GetOP(opIndex, &adr);
    Compare(cpu, &A_copia, op);
}

// ----------------------------------------------------------------------
// CPX Compara memoria con X
// X - M                            N Z C I D V
//                                  + + + - - -
// ----------------------------------------------------------------------
pub fn opCPX(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let X_copia = cpu.X.clone();
    let op = cpu.GetOP(opIndex, &adr);
    Compare(cpu, &X_copia, op);
}

// ----------------------------------------------------------------------
// CPY Compara memoria con Y
// Y - M                            N Z C I D V
//                                  + + + - - -
// ----------------------------------------------------------------------
pub fn opCPY(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let Y_copia = cpu.Y.clone();
    let op = cpu.GetOP(opIndex, &adr);
    Compare(cpu, &Y_copia, op);
}

// ----------------------------------------------------------------------
// DEC Decrementa memoria en 1
// M - 1 -> M                       N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opDEC(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let adr_copy = adr.clone();
    let res = cpu.GetOP(opIndex, &adr_copy) - 1;
    let v1: bool = cpu.IsZero(&res);
    cpu.SetFlag2(CPU::ZERO_FLAG, v1);

    let v2: bool = cpu.IsNegative(&res);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
    cpu.SetOP(opIndex, adr, res);
}

// ----------------------------------------------------------------------
// DEX Decrementa X en 1
// X - 1 -> X                       N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opDEX(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.X = &cpu.X - 1;
    let copia_X = cpu.X.clone();
    let v1: bool = cpu.IsZero(&copia_X);
    cpu.SetFlag2(CPU::ZERO_FLAG, v1);

    let v2: bool = cpu.IsNegative(&copia_X);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
}

// ----------------------------------------------------------------------
// DEY Decrementa Y en 1   Y - 1 -> Y
// Y - 1 -> Y                       N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opDEY(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.Y = &cpu.Y - 1;
    let copia_Y = cpu.Y.clone();
    let v1: bool = cpu.IsZero(&copia_Y);
    cpu.SetFlag2(CPU::ZERO_FLAG, v1);

    let v2: bool = cpu.IsNegative(&copia_Y);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
}

// ----------------------------------------------------------------------
// EOR Acumulador OR exclusiva con Memoria
// A EOR M -> A                     N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn EOR(cpu: &mut CPU, op: u8) {
    cpu.A ^= op;
    let copia_A = cpu.A.clone();

    let v1: bool = cpu.IsNegative(&copia_A);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&copia_A);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);
}

pub fn opEOR(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let v: u8 = cpu.GetOP(opIndex, &adr);
    EOR(cpu, v);
}

// ----------------------------------------------------------------------
// INC Incrementa memoria en 1
// M + 1 -> M                       N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opINC(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let res = cpu.GetOP(opIndex, &adr) + 1;
    //res++;
    let v1: bool = cpu.IsZero(&res);
    cpu.SetFlag2(CPU::ZERO_FLAG, v1);

    let v2: bool = cpu.IsNegative(&res);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
    cpu.SetOP(opIndex, adr, res);
}

// ----------------------------------------------------------------------
// INX Incrementa X en 1
// X + 1 -> X                       N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opINX(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.X = &cpu.X + 1;
    let copia_X = cpu.X.clone();

    let v1: bool = cpu.IsZero(&copia_X);
    cpu.SetFlag2(CPU::ZERO_FLAG, v1);

    let v2: bool = cpu.IsNegative(&copia_X);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
}

// ----------------------------------------------------------------------
// INY Incrementa Y en 1   Y + 1 -> Y
//  Y + 1 -> Y                       N Z C I D V
//                                   + + - - - -
// ----------------------------------------------------------------------
pub fn opINY(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.Y = &cpu.Y + 1;
    let copia_Y = cpu.Y.clone();

    let v1: bool = cpu.IsZero(&copia_Y);
    cpu.SetFlag2(CPU::ZERO_FLAG, v1);

    let v2: bool = cpu.IsNegative(&copia_Y);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
}


// ----------------------------------------------------------------------
// JMP Salto a una nueva localización
// (PC+1) -> PCL                    N Z C I D V
// (PC+2) -> PCH                    - - - - - -
// ----------------------------------------------------------------------
pub fn opJMP(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    match adr {
        Addressing::Absolute => {
            cpu.PC = cpu.mMemory.GetW(opIndex);
        }
        Addressing::Indirect => {
            let addr = cpu.mMemory.GetW(opIndex);
            cpu.PC = cpu.mMemory.GetW(&addr);
        }
        _ => panic!("Modo de direccionamiento en JMP no soportado"),
    }
}

// ----------------------------------------------------------------------
// JSR Salto a subrutina guardando el retorno en el stack
// push (PC+2),                     N Z C I D V
// (PC+1) -> PCL                    - - - - - -
// (PC+2) -> PCH
// ----------------------------------------------------------------------
pub fn opJSR(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let retAddress = &cpu.PC - 1; // next instruction - 1
    cpu.StackPush((retAddress >> 8) as u8);
    cpu.StackPush((retAddress & 0xFF) as u8);
    cpu.PC = cpu.mMemory.GetW(opIndex);
}


// ----------------------------------------------------------------------
// LDA Load acumulador con memoria
// M -> A                           N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opLDA(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.A = cpu.GetOP(opIndex, &adr);
    let A_copia = cpu.A.clone();
    let v1: bool = cpu.IsNegative(&A_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&A_copia);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);
}

// ----------------------------------------------------------------------
// LDX Load X con memoria
// M -> X                           N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opLDX(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.X = cpu.GetOP(opIndex, &adr);
    let X_copia = cpu.X.clone();
    let v1: bool = cpu.IsNegative(&X_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&X_copia);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);
}

// ----------------------------------------------------------------------
// LDY Load Y con memoria
// M -> Y                           N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opLDY(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.Y = cpu.GetOP(opIndex, &adr);
    let Y_copia = cpu.Y.clone();
    let v1: bool = cpu.IsNegative(&Y_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&Y_copia);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);
}

// ----------------------------------------------------------------------
// LSR Desplazamiento un Bit hacia la derecha (Memoria o Acumulador)
// 0 -> [76543210] -> C             N Z C I D V
//                                  0 + + - - -
// ----------------------------------------------------------------------
pub fn LSR(cpu: &mut CPU, op: u8) -> u8 {
    cpu.SetFlag2(CPU::CARRY_FLAG, (&op & 0x01) != 0);
    let op_copia = op.clone() >> 1;

    cpu.ClearFlag(CPU::NEGATIVE_FLAG);
    let v: bool = cpu.IsZero(&op_copia);

    cpu.SetFlag2(CPU::ZERO_FLAG, v);
    op_copia
}

pub fn opLSR(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let v1: u8 = cpu.GetOP(opIndex, &adr);
    let v2: u8 = LSR(cpu, v1);
    cpu.SetOP(opIndex, adr, v2);
}

// ----------------------------------------------------------------------
// NOP   No Operación
// ---                              N Z C I D V
//                                  - - - - - -
// ----------------------------------------------------------------------
pub fn opNOP(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {}

// ----------------------------------------------------------------------
// ORA OR de memoria con acumulador
// A OR M -> A                      N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn ORA(cpu: &mut CPU, op: u8) {
    cpu.A |= op;
    let A_copia = cpu.A.clone();

    let v1: bool = cpu.IsNegative(&A_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&A_copia);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);
}

pub fn opORA(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let v: u8 = cpu.GetOP(opIndex, &adr);
    ORA(cpu, v);
}

// ----------------------------------------------------------------------
// PHA Push acumulador en el stack
// push A                           N Z C I D V
//                                  - - - - - -
// ----------------------------------------------------------------------
pub fn opPHA(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let A_copia = cpu.A.clone();
    cpu.StackPush(A_copia);
}

// ----------------------------------------------------------------------
// PHP Push status del procesador en el stack
// push SR                          N Z C I D V
//                                  - - - - - -
// ----------------------------------------------------------------------
pub fn opPHP(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let F_copia = cpu.F.clone();
    cpu.StackPush(F_copia);
}

// ----------------------------------------------------------------------
// PLA Pull acumulador del stack
// pull A                           N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opPLA(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.A = cpu.StackPull();
}

// ----------------------------------------------------------------------
// PLP Pull status del procesador del stack
// pull SR                          N Z C I D V
//                                  del stack
// ----------------------------------------------------------------------
pub fn opPLP(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.F = cpu.StackPull();
}

// ----------------------------------------------------------------------
// ROL Rotar un bit a la izquierda (memoria o acumulador)
// C <- [76543210] <- C             N Z C I D V
//                                  + + + - - -
// ----------------------------------------------------------------------
pub fn ROL(cpu: &mut CPU, op: u8) -> u8 {
    let hadCarry = cpu.IsSetFlag(CPU::CARRY_FLAG);

    cpu.SetFlag2(CPU::CARRY_FLAG, (&op & 0x80) != 0);
    let mut op_copia = op << 1;
    //op_copia <<= 1;
    if hadCarry {
        op_copia = &op_copia + 1;
    }
    let v1: bool = cpu.IsNegative(&op_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&op_copia);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);
    op_copia
}

pub fn opROL(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let v1: u8 = cpu.GetOP(opIndex, &adr);
    let v2: u8 = ROL(cpu, v1);
    cpu.SetOP(opIndex, adr, v2);
}

// ----------------------------------------------------------------------
// ROR Rotar un bit a la derecha (memoria o acumulador)
// C -> [76543210] -> C             N Z C I D V
//                                  + + + - - -
// ----------------------------------------------------------------------
pub fn ROR(cpu: &mut CPU, op: u8) -> u8 {
    let hadCarry = cpu.IsSetFlag(CPU::CARRY_FLAG);
    cpu.SetFlag2(CPU::CARRY_FLAG, (&op & 0x01) != 0);
    let mut op_copia = op >> 1;
    //op >> = 1;
    if hadCarry {
        op_copia = &op_copia + 0x80;
    }
    let v1: bool = cpu.IsNegative(&op_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&op_copia);
    cpu.SetFlag2(CPU::ZERO_FLAG, v2);
    op_copia
}

pub fn opROR(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let v1: u8 = cpu.GetOP(opIndex, &adr);
    let v2: u8 = ROR(cpu, v1);
    cpu.SetOP(opIndex, adr, v2);
}

// ----------------------------------------------------------------------
// RTI Return de Interrupción
// pull SR, pull PC                 N Z C I D V
//                                   del stack
// ----------------------------------------------------------------------
pub fn opRTI(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.F = cpu.StackPull();
    let mut retAddress = cpu.StackPull() as u16;
    retAddress += ((cpu.StackPull() as u16) << 8);
    cpu.PC = retAddress;
}

// ----------------------------------------------------------------------
// RTS Return de subrutina sacando el retorno del stack
// pull PC, PC+1 -> PC              N Z C I D V
//                                  - - - - - -
// ----------------------------------------------------------------------
pub fn opRTS(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let mut retAddress = cpu.StackPull() as u16;
    retAddress = retAddress + ((cpu.StackPull() as u16) << 8);
    let retAddress1 = retAddress + 1; // we pushed next instruction - 1
    cpu.PC = retAddress1;
}

// ----------------------------------------------------------------------
// SBC Resta memoria del acumulador con acarreo
// A - M - C -> A                   N Z C I D V
//                                  + + + - - +
// ----------------------------------------------------------------------
pub fn SBC(cpu: &mut CPU, op: u8) { // suma
    if cpu.IsSetFlag(CPU::DECIMAL_FLAG) {
        let lhr = (cpu.A & 15) + (cpu.A >> 4) * 10;
        let rhr = (op & 15) + (op >> 4) * 10;
        let mut res = lhr - rhr;
        cpu.SetFlag2(CPU::OVERFLOW_FLAG, res < 0);
        res %= 100; // ???
        cpu.A = res;
        let v1: bool = cpu.IsNegative(&res);
        cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

        let v2: bool = cpu.IsZero(&res);
        cpu.SetFlag2(CPU::ZERO_FLAG, v2);
    } else {
        let mut result: i16 = cpu.A.clone() as i16;


        if cpu.IsSetFlag(CPU::CARRY_FLAG) && op > cpu.A {
            result += 0x100;
            cpu.ClearFlag(CPU::CARRY_FLAG);
        }
        result -= op.clone() as i16;


        cpu.SetFlag2(CPU::OVERFLOW_FLAG, result < -127 || result > 127); // validate this
        let res = (&result & 0xFF) as u8;

        let v: bool = cpu.IsNegative(&res);
        cpu.SetFlag2(CPU::NEGATIVE_FLAG, v);

        let v: bool = cpu.IsZero(&res);
        cpu.SetFlag2(CPU::ZERO_FLAG, v);


        cpu.A = res;
    }
}

pub fn opSBC(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let v: u8 = cpu.GetOP(opIndex, &adr);
    SBC(cpu, v);
}

// ----------------------------------------------------------------------
// SEC Set carry flag
// 1 -> C                           N Z C I D V
//                                  - - 1 - - -
// ----------------------------------------------------------------------
pub fn opSEC(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.SetFlag1(CPU::CARRY_FLAG);
}

// ----------------------------------------------------------------------
// SED Set Decimal Flag
// 1 -> D                           N Z C I D V
//                                  - - - - 1 -
// ----------------------------------------------------------------------
pub fn opSED(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.SetFlag1(CPU::DECIMAL_FLAG);
}

// ----------------------------------------------------------------------
// SEI Set Interrupt Disable Status
// 1 -> I                           N Z C I D V
//                                  - - - 1 - -
// ----------------------------------------------------------------------
pub fn opSEI(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.SetFlag1(CPU::INTERRUPT_FLAG);
}

// ----------------------------------------------------------------------
// STA Store Acumulador en Memoria
// A -> M                           N Z C I D V
//                                  - - - - - -
// ----------------------------------------------------------------------
pub fn opSTA(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let A_copia = cpu.A.clone();
    cpu.SetOP(opIndex, adr, A_copia);
}

// ----------------------------------------------------------------------
// STX Store X en Memoria
// X -> M                           N Z C I D V
//                                  - - - - - -
// ----------------------------------------------------------------------
pub fn opSTX(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let X_copia = cpu.X.clone();
    cpu.SetOP(opIndex, adr, X_copia);
}

// ----------------------------------------------------------------------
// STY Store Y en Memoria
// Y -> M                           N Z C I D V
//                                  - - - - - -
// ----------------------------------------------------------------------
pub fn opSTY(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    let Y_copia = cpu.Y.clone();
    cpu.SetOP(opIndex, adr, Y_copia);
}

// ----------------------------------------------------------------------
// TAX Transfer Acumulador a X
// A -> X                           N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opTAX(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.X = cpu.A.clone();
    let X_copia = cpu.X.clone();

    let v1: bool = cpu.IsNegative(&X_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&X_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
}

// ----------------------------------------------------------------------
// TAY Transfer Acumulador a Y
// A -> Y                           N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opTAY(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.Y = cpu.A.clone();
    let Y_copia = cpu.Y.clone();

    let v1: bool = cpu.IsNegative(&Y_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&Y_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
}

// ----------------------------------------------------------------------
// TSX Transfer Stack pointer a X
// SP -> X                          N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opTSX(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.X = cpu.S.clone();
    let X_copia = cpu.X.clone();

    let v1: bool = cpu.IsNegative(&X_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&X_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
}

// ----------------------------------------------------------------------
// TXA Transfer X a Acumulador
// X -> A                           N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opTXA(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.A = cpu.X.clone();
    let A_copia = cpu.A.clone();

    let v1: bool = cpu.IsNegative(&A_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&A_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
}

// ----------------------------------------------------------------------
// TXS Transfer X a Stack
// X -> SP                          N Z C I D V
//                                  - - - - - -
// ----------------------------------------------------------------------
pub fn opTXS(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.S = cpu.X.clone();
}

// ----------------------------------------------------------------------
// TYA Transfer Y a Acumulador
// Y -> A                           N Z C I D V
//                                  + + - - - -
// ----------------------------------------------------------------------
pub fn opTYA(cpu: &mut CPU, opIndex: &u16, adr: Addressing) {
    cpu.A = cpu.Y.clone();
    let A_copia = cpu.A.clone();

    let v1: bool = cpu.IsNegative(&A_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v1);

    let v2: bool = cpu.IsZero(&A_copia);
    cpu.SetFlag2(CPU::NEGATIVE_FLAG, v2);
}