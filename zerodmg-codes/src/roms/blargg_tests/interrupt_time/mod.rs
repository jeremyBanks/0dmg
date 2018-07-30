use crate::assembled::AssembledRom;

const INTERRUPT_TIME: &[u8; 0x8000] = include_bytes!("./interrupt_time.gb");

/// Blargg's Interrupt Timing Test ROM
pub fn interrupt_time() -> AssembledRom {
    let rom = AssembledRom::from_bytes(&INTERRUPT_TIME.to_vec());
    if cfg!(debug_assertions) {
        verify(&rom);
    }
    rom
}

/// A sanity-check/test of the result, only checked in debug mode and tests.
fn verify(assembled: &AssembledRom) {
    let known_vec = INTERRUPT_TIME.to_vec();

    println!("=== Disassembled Interrupt Timing Test ROM ===");
    let mut assembled = assembled.clone();
    assembled.trace_standard_game_instructions();
    let disassembled = assembled.disassemble();
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    let reassembled_bytes = disassembled.assemble().to_bytes();
    assert_eq!(known_vec, reassembled_bytes);
}

#[test]
#[ignore(requires_multiple_rom_bank_support)]
fn test_verified() {
    interrupt_time();
}
