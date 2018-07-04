#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![feature(iterator_flatten)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(dead_code, unused_imports)]

//! Experiments in decoding game boy machine code.

/// Instruction-level stuff.
pub mod operation;
/// ROM-level stuff.
pub mod rom;

use self::operation::Operation::*;
use self::operation::U16Register::*;
use self::operation::U8Register::*;
use self::rom::ROM;

/// Runs whatever I'm working on!
pub fn main() -> Result<(), String> {
    let instructions = ROM {
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

    println!("input data:\n{:?}\n", instructions.operations);

    println!("as ASM:\n{}\n", instructions.to_asm());
    println!("as machine code:\n{:?}\n", instructions.to_bytes());

    println!(
        "re-parsed from machine code:\n{}\n",
        ROM::from_bytes(instructions.to_bytes()).to_asm()
    );

    Ok(())
}
