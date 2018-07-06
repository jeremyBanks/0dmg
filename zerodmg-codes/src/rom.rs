use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::fmt::Debug;

use crate::instruction::Instruction;
use crate::instruction::Instruction::*;
use crate::instruction::U16Register::*;
use crate::instruction::U8Register::*;

//^ Encoding/decoding of complete ROMs.

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use super::AssembledROM;
    pub use super::DisassembledROM;
    pub use super::ROMBlock;
    pub use super::ROMBlockContent::*;
}

use self::prelude::*;

/// A ROM in a disassembled assembly-like structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisassembledROM {
    pub blocks: Vec<ROMBlock>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ROMBlock {
    pub content: ROMBlockContent,
    pub label: Option<String>,
    pub address: Option<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ROMBlockContent {
    Code(Vec<Instruction>),
    Data(Vec<u8>),
}

/// A ROM of compiled machine code bytes, potentially with their decoded instruction values attached.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssembledROM {
    pub bytes: Vec<ROMByte>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ROMByte {
    pub byte: u8,
    pub kind: ROMByteKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ROMByteKind {
    /// This may be data, unused, or code we don't understand.
    Unknown,
    /// The initial byte of an instruction; a point at which we can begin parsing.
    InstructionStart(Instruction, IsJumpDestination),
    /// The non-initial byte of an instruction.
    InstructionRest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IsJumpDestination {
    /// We don't know whether this instruction is a potential jump destination.
    Unknown,
    /// We are confident that this is a potential jump destination in the code.
    Yes,
}

// logic
impl From<&AssembledROM> for DisassembledROM {
    fn from(_assembled: &AssembledROM) -> Self {
        unimplemented!()
    }
}

impl AssembledROM {
    pub fn trace_entry_point(&mut self, _entry_point: u16) {
        unimplemented!()
    }
}

impl From<&DisassembledROM> for AssembledROM {
    fn from(_assembled: &DisassembledROM) -> Self {
        unimplemented!()
    }
}

// glue
impl From<&Vec<u8>> for AssembledROM {
    fn from(bytes: &Vec<u8>) -> Self {
        let mut assembled = Self {
            bytes: bytes.iter().map(|byte| byte.clone().into()).collect(),
        };

        // 0x0000 isn't even an entry point for real ROMs,
        // but we're going to use this simplification for now.
        assembled.trace_entry_point(0x0000);

        assembled
    }
}

impl From<ROMBlockContent> for ROMBlock {
    fn from(content: ROMBlockContent) -> Self {
        Self {
            content,
            label: None,
            address: None,
        }
    }
}

impl From<u8> for ROMByte {
    fn from(byte: u8) -> Self {
        Self {
            byte,
            kind: ROMByteKind::Unknown,
        }
    }
}

impl From<Vec<ROMBlock>> for DisassembledROM {
    fn from(blocks: Vec<ROMBlock>) -> Self {
        DisassembledROM { blocks }
    }
}

impl From<Vec<ROMBlockContent>> for DisassembledROM {
    fn from(blocks_contents: Vec<ROMBlockContent>) -> Self {
        DisassembledROM {
            blocks: blocks_contents
                .into_iter()
                .map(|content| content.into())
                .collect(),
        }
    }
}

impl From<Vec<Instruction>> for DisassembledROM {
    fn from(instructions: Vec<Instruction>) -> Self {
        DisassembledROM {
            blocks: vec![Code(instructions).into()],
        }
    }
}

impl From<Vec<u8>> for DisassembledROM {
    fn from(bytes: Vec<u8>) -> Self {
        (&AssembledROM::from(&bytes)).into()
    }
}

impl From<Vec<ROMBlock>> for AssembledROM {
    fn from(blocks: Vec<ROMBlock>) -> Self {
        (&DisassembledROM::from(blocks)).into()
    }
}

impl From<Vec<ROMBlockContent>> for AssembledROM {
    fn from(blocks_contents: Vec<ROMBlockContent>) -> Self {
        (&DisassembledROM::from(blocks_contents)).into()
    }
}

impl From<Vec<Instruction>> for AssembledROM {
    fn from(instructions: Vec<Instruction>) -> Self {
        (&DisassembledROM::from(instructions)).into()
    }
}

impl From<&DisassembledROM> for Vec<u8> {
    fn from(disassembled: &DisassembledROM) -> Self {
        (&AssembledROM::from(disassembled)).into()
    }
}

impl From<&AssembledROM> for Vec<u8> {
    fn from(assembled: &AssembledROM) -> Self {
        assembled.bytes.iter().map(|&byte| byte.byte).collect()
    }
}
