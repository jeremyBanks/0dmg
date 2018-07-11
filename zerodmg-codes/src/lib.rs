#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![feature(try_from)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(dead_code, unused_imports)]

//! Experiments in decoding Game Boy machine code.

/// Encoding/decoding individual CPU instructions.
pub mod instruction;

/// Encoding/decoding of complete ROMs.
///
/// Apparent logic errors in ROM data are currently "handled" by panicking.
pub mod rom;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use crate::instruction::prelude::*;
    pub use crate::rom::prelude::*;
}

use self::prelude::*;

/// An example/experimental use of this crate.
pub fn main() -> Result<(), Box<std::any::Any + Send>> {
    let disassembled = DisassembledRom::from({
        let main_addr = 0x0000;
        let init_addr = 0x0030;
        vec![
            RomBlock {
                address: Some(main_addr),
                content: Code(vec![INC(A), INC(A), INC(A)]),
            },
            RomBlock {
                address: None,
                content: Code(vec![DEC(A)]),
            },
            RomBlock {
                address: None,
                content: Code(vec![DEC(A), JP(init_addr)]),
            },
            RomBlock {
                address: None,
                content: Data(vec![
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                    0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x02,
                    0x03, 0x04, 0x05, 0x06, 0x07, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                ]),
            },
            RomBlock {
                address: Some(init_addr),
                content: Code(vec![DEC(A)]),
            },
        ]
    });

    println!("=== Disassembled ===");
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    println!("=== Assembled ===");
    let assembled = disassembled.assemble();
    println!("{:?}\n", Vec::<u8>::from(&assembled));

    println!("=== Redisassembled ===");
    let redisassembled = assembled.disassemble();
    println!("{:?}\n", redisassembled);
    println!("{}\n", redisassembled);

    Ok(())
}

#[test]
fn test_main() -> Result<(), Box<std::any::Any + Send>> {
    main()
}
