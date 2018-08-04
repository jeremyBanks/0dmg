use crate::assembled::AssembledRom;

const OAM_BUG: &[u8; 0x10000] = include_bytes!("./oam_bug.gb");

/// Blargg's OAM Bug Test ROM
pub fn oam_bug() -> AssembledRom {
    let rom = AssembledRom::from_bytes(&OAM_BUG.to_vec());
    if cfg!(debug_assertions) {
        verify(&rom);
    }
    rom
}

/// A sanity-check/test of the result, only checked in debug mode and tests.
fn verify(assembled: &AssembledRom) {
    let known_vec = OAM_BUG.to_vec();

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
    oam_bug();
}
