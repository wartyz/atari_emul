// https://www.youtube.com/watch?v=pcqKVQ_bK9A&list=PLJ0INvSnPQjPqw4SiiSVMCDAb3dllSs_M&index=1
// Video 1: acabado
// Video 2: acabado
// Video 3: 1:22:18
#![allow(non_snake_case)]

//use main::cpu::*;
use main::atari::Atari;
//use main::debugger::Debugger;
use main::tests::*;
use main::memory::Memory;
use main::debugger::Debugger;

fn main() {
    let mut atari = Atari::new();
    atari.mCPU.mMemory.Load("ehbasic.bin".to_string(), 0xC000);
    atari.mCPU.EntryPoint(0xC000, 0xFFFF);
    while atari.mCPU.CyclesDevuelve() < 10000_0000 {
        Debugger::DumpCPU(&mut atari.mCPU);
        atari.mCPU.Execute();
    }

    //Tests::CPUFlagsTest(&mut atari.mCPU);
    //let mut debugger = Debugger::DumpCPU(&atari.mCPU);
    //Tests::ADC_iTest(&mut atari.mCPU);
    //Tests::AddrTest(&mut atari.mCPU);
    //Tests::OPTest(&mut atari.mCPU);

    println!("Done!");
}

#[cfg(test)]
mod tests {
    use main::atari::Atari;

    #[test]
    fn prueba() {
        assert_eq!(2 + 2, 4);
    }
}



