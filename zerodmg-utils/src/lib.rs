#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![feature(try_from)]
#![warn(missing_docs, missing_debug_implementations)]

//! Assorted utility modules for zerodmg.

/// Functions for working with little-endian binary data.
/// Argument lists and return tuples are least-significant-first.
///
/// Don't forget that Rust's hex integer literals are big-endian.
pub mod little_endian;
