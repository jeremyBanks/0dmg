#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![feature(try_from)]
#![warn(missing_docs, missing_debug_implementations)]

//! Experiments in decoding Game Boy machine code.

/// Encoding/decoding individual CPU instructions.
pub mod instruction;

/// Decoding and disassembling assembled ROMs binaries.
pub mod assembled;

/// Assembling and manipulating disassembled ROM programs.
pub mod disassembled;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use crate::assembled::prelude::*;
    pub use crate::disassembled::prelude::*;
    pub use crate::instruction::prelude::*;
}

use self::prelude::*;

/// Returns a DisassembledRom with our demo program.
pub fn demo() -> DisassembledRom {
    DisassembledRom::from({
        vec![
            RomBlock {
                // Main entry point, jumping to main function.
                address: Some(0x0100),
                content: Code(vec![
                    JP(0x0150),
                ]),
            },
            
            RomBlock {
                // Nintendo Logo (for copyright check).
                address: Some(0x0104),
                content: Data(vec![
                    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
                    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
                    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
                    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
                    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
                    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
                ]),
            },

            RomBlock {
                // Metadata.
                address: Some(0x0134),
                content: Data(vec![
                    0x00, 0x00, 0x00, 0x00, 0x00, 
                    0x00, 0x00, 0x00, 0x00, 0x00, 
                    0x00, 0x00, 0x00, 0x00, 0x00, 
                    0x00, 0x00, 0x00, 0x00, 0x00, 
                    0x00, 0x00, 0x00, 0x00, 0x00,
                ]),
            },

            RomBlock {
                // Metadata checksum (sum of metadata bytes + 0xE7).
                address: Some(0x014D),
                content: Data(vec![0xE7]),
            },
            
            RomBlock {
                // Main function.
                address: Some(0x0150),
                content: Code(vec![INC(A), INC(A), INC(A), JP(0x0150)]),
            },
        ]
    })
}

#[test]
pub fn test() -> Result<(), Box<std::any::Any + Send>> {
    let disassembled = demo();

    println!("=== Input ===");
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    println!("=== Assembled ===");
    let assembled = disassembled.assemble();
    println!("{:?}\n", assembled.to_bytes());

    println!("=== Redisassembled (using metadata) ===");
    let redisassembled = assembled.disassemble();
    println!("{:?}\n", redisassembled);
    println!("{}\n", redisassembled);

    println!("=== Redisassembled (just from the bytes) ===");
    let really_disassembled = AssembledRom::new(assembled.to_bytes()).disassemble();
    println!("{:?}\n", really_disassembled);
    println!("{}\n", really_disassembled);

    Ok(())
}
