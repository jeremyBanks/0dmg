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
