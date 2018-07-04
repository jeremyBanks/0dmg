#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![feature(iterator_flatten)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(dead_code, unused_imports)]

//! Experiments in decoding game boy machine code.

/// It's all in here.
pub mod asm;

use self::asm::Operation::*;
use self::asm::U16Register::*;
use self::asm::U8Register::*;

/// Runs whatever I'm working on!
pub fn main() -> Result<(), String> {
    let instructions = asm::ROM {
        operations: vec![
            INC(A),
            JP_NZ(0x0009),
            DATA(vec![0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]),
            INC(A),
            INC(A),
            INC(A),
            NOP,
            DEC(A),
            JP_NZ(0x000E),
        ],
    };

    println!("{:#?}\n", instructions.operations);
    println!("{}\n", instructions.to_asm());
    println!("{:#?}", instructions.to_bytes());

    Ok(())
}
