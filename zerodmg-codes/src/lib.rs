#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![feature(try_from)]
#![warn(missing_docs, missing_debug_implementations)]

//! Experiments in decoding Game Boy machine code.

#[macro_use]
extern crate derive_more;

/// Encoding/decoding individual CPU instructions.
pub mod instruction;

/// Decoding and disassembling assembled ROMs binaries.
pub mod assembled;

/// Assembling and manipulating disassembled ROM programs.
pub mod disassembled;

/// ROM data constants/factories.
pub mod roms;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use crate::assembled::prelude::*;
    pub use crate::disassembled::prelude::*;
    pub use crate::instruction::prelude::*;
}
