use crate::assembled::AssembledRom;

const MEM_TIMING: &[u8; 0x10000] = include_bytes!("./mem_timing.gb");

/// Blargg's Memory Timing Test ROM
pub fn mem_timing() -> AssembledRom {
    let rom = AssembledRom::from_bytes(&MEM_TIMING.to_vec());
    if cfg!(debug_assertions) {
        verify(&rom);
    }
    rom
}

/// A sanity-check/test of the result, only checked in debug mode and tests.
fn verify(assembled: &AssembledRom) {
    let known_vec = MEM_TIMING.to_vec();

    println!("=== Disassembled Memory Timing Test ROM ===");
    let mut assembled = assembled.clone();
    assembled.trace_standard_game_instructions();
    let disassembled = assembled.disassemble();
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    let reassembled_bytes = disassembled.assemble().to_bytes();
    assert_eq!(known_vec, reassembled_bytes);
}

#[test]
fn test_verified() {
    mem_timing();
}
