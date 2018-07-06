#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(dead_code, unused_imports)]

//! Experiments in decoding game boy machine code.

/// Instruction types.
pub mod instruction;

/// ROM types.
pub mod rom;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use crate::instruction::prelude::*;
    pub use crate::rom::prelude::*;
}
