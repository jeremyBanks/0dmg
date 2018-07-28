use crate::assembled::prelude::*;
use crate::disassembled::prelude::*;
use crate::instruction::prelude::*;

const INSTR_TIMING: &[u8; 0x8000] = include_bytes!("./instr_timing.gb");

/// Blargg's Instruction Timing Test ROM
pub fn instr_timing() -> AssembledRom {
    let rom = AssembledRom::from_bytes(&INSTR_TIMING.to_vec());
    if cfg!(debug_assertions) {
        verify(&rom);
    }
    rom
}

/// A sanity-check/test of the result, only checked in debug mode and tests.
fn verify(assembled: &AssembledRom) {
    let known_vec = INSTR_TIMING.to_vec();

    println!("=== Disassembled Instruction Timing Test ROM ===");
    let mut assembled = assembled.clone();
    assembled.trace_standard_game_instructions();
    let disassembled = assembled.disassemble();
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    let reassembled_bytes = disassembled.assemble().to_bytes();
    assert_eq!(known_vec, reassembled_bytes);
}

#[test]
#[ignore]
fn test_verified() {
    instr_timing();
}
