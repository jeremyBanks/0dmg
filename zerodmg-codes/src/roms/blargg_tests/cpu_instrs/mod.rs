use crate::assembled::AssembledRom;

const CPU_INSTRS: &[u8; 0x10000] = include_bytes!("./cpu_instrs.gb");

/// Blargg's CPU Instructions Test ROM
pub fn cpu_instrs() -> AssembledRom {
    let rom = AssembledRom::from_bytes(&CPU_INSTRS.to_vec());
    if cfg!(debug_assertions) {
        verify(&rom);
    }
    rom
}

/// A sanity-check/test of the result, only checked in debug mode and tests.
fn verify(assembled: &AssembledRom) {
    let known_vec = CPU_INSTRS.to_vec();

    println!("=== Disassembled CPU Instructions Test ROM ===");
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
    cpu_instrs();
}
