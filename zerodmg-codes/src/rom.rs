use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::fmt::Debug;

use crate::operation::Operation;
use crate::operation::U16Register::*;
use crate::operation::U8Register::*;

/// A complete ROM/binary executable for the processor.
#[derive(Clone, Debug)]
pub struct ROM {
    /// All of the operations and data in the ROM.
    pub operations: Vec<Operation>,
}

impl ROM {
    /// Encodes this ROM as a pseudo-assembly string.
    pub fn to_asm(&self) -> String {
        self.operations
            .iter()
            .map(|ref x| x.to_asm())
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Encodes this ROM as machine code bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.operations
            .iter()
            .map(|ref x| x.to_bytes())
            .flatten()
            .collect()
    }

    /// Decodes machine code bytes into a ROM.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let mut byte_iter = bytes.iter();
        let mut operations = Vec::<Operation>::new();
        loop {
            if let Some(next) = Operation::from_byte_iter(&mut byte_iter) {
                operations.push(next);
            } else {
                break;
            }
        }
        Self { operations }
    }
}
