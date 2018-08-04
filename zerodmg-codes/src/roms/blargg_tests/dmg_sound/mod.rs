use crate::assembled::AssembledRom;

const DMG_SOUND: &[u8; 0x10000] = include_bytes!("./dmg_sound.gb");

/// Blargg's DMG Sound Test ROM
pub fn dmg_sound() -> AssembledRom {
    let rom = AssembledRom::from_bytes(&DMG_SOUND.to_vec());
    if cfg!(debug_assertions) {
        verify(&rom);
    }
    rom
}

/// A sanity-check/test of the result, only checked in debug mode and tests.
fn verify(assembled: &AssembledRom) {
    let known_vec = DMG_SOUND.to_vec();

    println!("=== Disassembled DMG Sound Test ROM ===");
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
    dmg_sound();
}
