use crate::assembled::AssembledRom;

const MEM_TIMING_2: &[u8; 0x10000] = include_bytes!("./mem_timing_2.gb");

/// Blargg's Memory Timing Test ROM 2
pub fn mem_timing_2() -> AssembledRom {
    let rom = AssembledRom::from_bytes(&MEM_TIMING_2.to_vec());
    if cfg!(debug_assertions) {
        verify(&rom);
    }
    rom
}

/// A sanity-check/test of the result, only checked in debug mode and tests.
fn verify(assembled: &AssembledRom) {
    let known_vec = MEM_TIMING_2.to_vec();

    println!("=== Disassembled Memory Tming Test ROM ===");
    let mut assembled = assembled.clone();
    assembled.trace_standard_game_instructions();
    let disassembled = assembled.disassemble();
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    let reassembled_bytes = disassembled.assemble().to_bytes();
    assert_eq!(known_vec, reassembled_bytes);
}

#[test]
#[ignore(known_failure)]
fn test_round_trip() {
    mem_timing_2();
}
