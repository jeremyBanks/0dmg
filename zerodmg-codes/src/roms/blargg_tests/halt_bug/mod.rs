use crate::assembled::AssembledRom;

const HALT_BUG: &[u8; 0x8000] = include_bytes!("./halt_bug.gb");

/// Blargg's Halt Bug Test ROM
pub fn halt_bug() -> AssembledRom {
    let rom = AssembledRom::from_bytes(&HALT_BUG.to_vec());
    if cfg!(debug_assertions) {
        verify(&rom);
    }
    rom
}

/// A sanity-check/test of the result, only checked in debug mode and tests.
fn verify(assembled: &AssembledRom) {
    let known_vec = HALT_BUG.to_vec();

    println!("=== Disassembled Halt Bug Test ROM ===");
    let mut assembled = assembled.clone();
    assembled.trace_standard_game_instructions();
    let disassembled = assembled.disassemble();
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    let reassembled_bytes = disassembled.assemble().to_bytes();
    assert_eq!(known_vec, reassembled_bytes);
}

#[test]
fn test_round_trip() {
    halt_bug();
}
