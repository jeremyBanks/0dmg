use crate::assembled::prelude::*;
use crate::disassembled::prelude::*;
use crate::instruction::prelude::*;

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

/// My demo/test game ROM.
pub fn jeb_demo() -> DisassembledRom {
    let code = code_blocks![
        // One-byte RST instruction call targets.
        at 0x0000 => {HCF(xxDD)},
        at 0x0008 => {HCF(xxDD)},
        at 0x0010 => {HCF(xxDD)},
        at 0x0018 => {HCF(xxDD)},
        at 0x0020 => {HCF(xxDD)},
        at 0x0028 => {HCF(xxDD)},
        at 0x0030 => {HCF(xxDD)},
        at 0x0038 => {HCF(xxDD)},
        // Interrupt handlers:
        // V-Blank.
        at 0x0040 => {RETI},
        // LCD Status.
        at 0x0048 => {HCF(xxDD)},
        // Timer.
        at 0x0050 => {HCF(xxDD)},
        // Serial data.
        at 0x0058 => {HCF(xxDD)},
        // Button press.
        at 0x0060 => {HCF(xxDD)},

        // Game ROM entry point, from which we jump to our main function.
        at 0x0100 => {JP(main)},
        // Nintendo logo, required for boot ROM copyright check.
        at 0x0104 => Data(NINTENDO_LOGO),
        // Game metadata.
        // Since we only require the minimal feature set we can leave this zeroed.
        at 0x0134 => Data([0x00; 25]),
        // Metadata checksum, must be sum of metadata bytes + 0xE7 or boot ROM will freeze.
        at 0x014D => [0xE7],
        // Global checksum of all other bytes in the ROM... but not verified, so we can ignore it.
        at 0x014E => [0x00, 0x00],

        def main at 0x0150 => {
            // Set background palette
            LD(HL, 0xFF47);
            // to [black, dark gray, light gray, white]
            LD(A, 0b_00_01_10_11);
            LD(AT_HL, A);
        },
        // Set first tile to white.
        next => {
            LD(HL, 0x8000);
            LD(A, 0x10);
        },
        next as tile_loop => {
            LD(AT_HL, 0xFF);
            INC(L); // should be INC(HL);
            DEC(A);
            JP_IF(if_NZ, tile_loop);
        },
        // Set second tile to black.
        next => {
            LD(HL, 0x8010);
            LD(A, 0x10);
        },
        next as tile_loop => {
            LD(AT_HL, 0x00);
            INC(L); // should be INC(HL);
            DEC(A);
            JP_IF(if_NZ, tile_loop);
        },
        next => {
            // Set tiles to draw my logo in the corner.
            // We're using the second tile (0x01), which is white by default.
            LD(A, 0x01);
            LD(HL, 0x9800);
            /*     LD (HL+), A
                *     and INC HL
                *     // mixed to set/skip tiles to display logo
                *     // 0x23, 0x23, 0x23, 0x22, 0x22, 0x23, 0x23, 0x23,
                *     // // LD HL, 0x9820
                *     // 0x21, 0x20, 0x98,
                *     // 0x23, 0x23, 0x22, 0x23, 0x23, 0x22, 0x23, 0x23,
                *     // 0x21, 0x40, 0x98,
                *     // 0x23, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x23,
                *     // 0x21, 0x60, 0x98,
                *     // 0x22, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x22,
                *     // 0x21, 0x80, 0x98,
                *     // 0x23, 0x22, 0x23, 0x23, 0x23, 0x23, 0x22, 0x23,
                *     // 0x21, 0xA0, 0x98,
                *     // 0x22, 0x22, 0x22, 0x23, 0x23, 0x22, 0x22, 0x22,
                *     ///
                *     // // scroll background into middle of screen
                *     // // LD A, -32
                *     // 0x3E, 0xFF - 0x32 + 1,
                *     // // LD HL, 0xFF42 ; y-scroll register
                *     // 0x21, 0x42, 0xFF,
                *     // // LD (HL), A
                *     // 0x77,
                *     // // LD HL, 0xFF43 ; x-scroll register
                *     // 0x21, 0x43, 0xFF,
                *     // // LD (HL), A
                *     // 0x77,
                *     //
                *     // // infinite loop
                *     // // JR -2
                *     // 0x18, (0xFF - 2 + 1), */
        },

        def pallet_loop at 0x2000 => {
            // Mess with the pallet forever:
            LD(HL, 0xFF47);
            LD(A, AT_HL);
            INC(A);
            LD(AT_HL, A);
            JP(pallet_loop);
        },
    ];

    let rom = DisassembledRom::from(code);
    if cfg!(debug_assertions) {
        verify(&rom);
    }
    rom
}

/// A sanity-check/test of the result, only checked in debug mode and tests.
fn verify(rom: &DisassembledRom) {
    println!("=== Demo ROM ===");
    println!("{:?}\n", rom);
    println!("{}\n", rom);

    println!("=== Assembled Demo ROM ===");
    let assembled_bytes = rom.assemble().to_bytes();
    println!("{:?}\n", assembled_bytes);

    println!("=== Disassembled Demo ROM ===");
    let mut assembled = AssembledRom::from_bytes(&assembled_bytes);
    assembled.trace_standard_game_instructions();
    let disassembled = assembled.disassemble();
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    let reassembled_bytes = disassembled.assemble().to_bytes();
    assert_eq!(reassembled_bytes, assembled_bytes);
}

#[test]
fn test_verified() {
    jeb_demo();
}
