use crate::assembled::prelude::*;
use crate::disassembled::prelude::*;
use crate::instruction::prelude::*;

/// My demo/test game ROM.
pub fn jeb_demo() -> DisassembledRom {

    let header_stub = vec![
        // Game ROM entry point, from which we jump to our main function.
        block(0x0100, vec![JP(0x0150)]),
    ];

    let header_stub = vec![
        // Game ROM entry point, from which we jump to our main function.
        block(0x0100, vec![JP(0x0150)]),
        // Nintendo logo, must be exactly this or boot ROM will freeze.
        block(
            0x0104,
            vec![
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
        ),
        // Game metadata.
        // Since we only require the minimal feature set we can leave this zeroed.
        block(0x0134, vec![0x00; 25]),
        // Metadata checksum, must be sum of metadata bytes + 0xE7 or boot ROM will freeze.
        block(0x014D, vec![0xE7]),
        // Global checksum of all other bytes in the ROM... but not verified, so'll neglect it.
        block(0x014E, vec![0x00, 0x00]),
    ];

    let handlers = vec![
        // One-byte RST instruction call targets.
        block(0x0000, vec![HCF(xxDD)]),
        block(0x0008, vec![HCF(xxDD)]),
        block(0x0010, vec![HCF(xxDD)]),
        block(0x0018, vec![HCF(xxDD)]),
        block(0x0020, vec![HCF(xxDD)]),
        block(0x0028, vec![HCF(xxDD)]),
        block(0x0030, vec![HCF(xxDD)]),
        block(0x0038, vec![HCF(xxDD)]),
        // Interrupt handlers:
        // V-Blank.
        block(0x0040, vec![RETI]),
        // LCD Status.
        block(0x0048, vec![HCF(xxDD)]),
        // Timer.
        block(0x0050, vec![HCF(xxDD)]),
        // Serial data.
        block(0x0058, vec![HCF(xxDD)]),
        // Button press.
        block(0x0060, vec![HCF(xxDD)]),
    ];

    let demo_body = vec![
        // Main function.
        block(
            0x0150,
            vec![
                // Set background palette
                LD(HL, 0xFF47),
                // to [black, dark gray, light gray, white]
                LD(A, 0b_00_01_10_11),
                LD(AT_HL, A),

                // Set first tile to black.
                LD(HL, 0x8000),
                LD(A, 0xFF),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),
                LD(AT_HL_Plus, A),

                // Set tiles to draw my logo in the corner.
                // We're using the second tile (0x01), which is white by default.
                LD(A, 0x01),
                LD(HL, 0x9800),

            //     LD (HL+), A
            //     and INC HL
            //     // mixed to set/skip tiles to display logo
            //     // 0x23, 0x23, 0x23, 0x22, 0x22, 0x23, 0x23, 0x23,
            //     // // LD HL, 0x9820
            //     // 0x21, 0x20, 0x98,
            //     // 0x23, 0x23, 0x22, 0x23, 0x23, 0x22, 0x23, 0x23,
            //     // 0x21, 0x40, 0x98,
            //     // 0x23, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x23,
            //     // 0x21, 0x60, 0x98,
            //     // 0x22, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x22,
            //     // 0x21, 0x80, 0x98,
            //     // 0x23, 0x22, 0x23, 0x23, 0x23, 0x23, 0x22, 0x23,
            //     // 0x21, 0xA0, 0x98,
            //     // 0x22, 0x22, 0x22, 0x23, 0x23, 0x22, 0x22, 0x22,
            //     ///
            //     // // scroll background into middle of screen
            //     // // LD A, -32
            //     // 0x3E, 0xFF - 0x32 + 1,
            //     // // LD HL, 0xFF42 ; y-scroll register
            //     // 0x21, 0x42, 0xFF,
            //     // // LD (HL), A
            //     // 0x77,
            //     // // LD HL, 0xFF43 ; x-scroll register
            //     // 0x21, 0x43, 0xFF,
            //     // // LD (HL), A
            //     // 0x77,
            //     //
            //     // // infinite loop
            //     // // JR -2
            //     // 0x18, (0xFF - 2 + 1),
            ],
        ),
        // block(0x0200, vec![
        //     // // Set background palette to [black, dark gray, light gray, white].
        //     // // LD HL, 0xFF47
        //     // 0x21, 0x47, 0xFF,
        //     // // LD A, 0b00011011
        //     // 0x3E, 0b00011011,
        //     // // LD (HL), A
        //     // 0x77,
        //     //
        //     // // Set first tile to black.
        //     // // LD HL, 0x8000
        //     // 0x21, 0x00, 0x80,
        //     // // LD A, 0xFF
        //     // 0x3E, 0xFF,
        //     // // LD (HL+), A
        //     // // repeated 16 times
        //     // 0x22, 0x22, 0x22, 0x22,
        //     // 0x22, 0x22, 0x22, 0x22,
        //     // 0x22, 0x22, 0x22, 0x22,
        //     // 0x22, 0x22, 0x22, 0x22,
        //     ///
        //     // // Set tiles to draw my logo in the corner.
        //     // // LD A, 0x01
        //     // 0x3E, 0x01,
        //     // // LD HL, 0x9800
        //     // 0x21, 0x00, 0x98,
        //     // // LD (HL+), A
        //     // // and INC HL
        //     // // mixed to set/skip tiles to display logo
        //     // 0x23, 0x23, 0x23, 0x22, 0x22, 0x23, 0x23, 0x23,
        //     // // LD HL, 0x9820
        //     // 0x21, 0x20, 0x98,
        //     // 0x23, 0x23, 0x22, 0x23, 0x23, 0x22, 0x23, 0x23,
        //     // 0x21, 0x40, 0x98,
        //     // 0x23, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x23,
        //     // 0x21, 0x60, 0x98,
        //     // 0x22, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x22,
        //     // 0x21, 0x80, 0x98,
        //     // 0x23, 0x22, 0x23, 0x23, 0x23, 0x23, 0x22, 0x23,
        //     // 0x21, 0xA0, 0x98,
        //     // 0x22, 0x22, 0x22, 0x23, 0x23, 0x22, 0x22, 0x22,
        //     ///
        //     // // scroll background into middle of screen
        //     // // LD A, -32
        //     // 0x3E, 0xFF - 0x32 + 1,
        //     // // LD HL, 0xFF42 ; y-scroll register
        //     // 0x21, 0x42, 0xFF,
        //     // // LD (HL), A
        //     // 0x77,
        //     // // LD HL, 0xFF43 ; x-scroll register
        //     // 0x21, 0x43, 0xFF,
        //     // // LD (HL), A
        //     // 0x77,
        //     //
        RomBlock {
            address: None,
            content: Code(vec![JP(0x0180)]),
        },
        block(
            0x0180,
            vec![
                // Mess with the pallet forever:
                LD(HL, 0xFF47),
                LD(A, AT_HL),
                INC(A),
                LD(AT_HL, A),
                JP(0x0180),
            ],
        ),
        RomBlock {
            address: None,
            // Loop back to main.
            content: Code(vec![JP(0x0150)]),
        },
    ];

    let rom = DisassembledRom::from(vec![handlers, header_stub, demo_body].concat());
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
