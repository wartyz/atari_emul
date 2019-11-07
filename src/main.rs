// https://www.youtube.com/watch?v=pcqKVQ_bK9A&list=PLJ0INvSnPQjPqw4SiiSVMCDAb3dllSs_M&index=1
// 6:11:45
#![allow(non_snake_case)]

use main::cpu::*;
use main::atari::Atari;
use main::debugger::Debugger;
use main::tests::*;

fn main() {
    let mut atari = Atari::new();


    Tests::CPUFlagsTest(&mut atari.mCPU);
    //let mut debugger = Debugger::DumpCPU(&atari.mCPU);
    Tests::ADC_iTest(&mut atari.mCPU);
    Tests::AddrTest(&mut atari.mCPU);
    Tests::OPTest(&mut atari.mCPU);

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



