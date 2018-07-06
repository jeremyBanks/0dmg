#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(dead_code, unused_imports)]

//! Experiments in decoding Game Boy machine code.

/// Encoding/decoding individual CPU instructions.
pub mod instruction;

/// Encoding/decoding of complete ROMs.
///
/// Apparent logic errors in ROM data are currently handled by panicking.
pub mod rom;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use crate::instruction::prelude::*;
    pub use crate::rom::prelude::*;
}
