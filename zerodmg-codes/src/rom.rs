use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::fmt;
use std::fmt::{Debug, Display};

use crate::instruction::Instruction;
use crate::instruction::Instruction::*;
use crate::instruction::U16Register::*;
use crate::instruction::U8Register::*;

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
    /// An ordered list of blocks of code or data in the ROM.
    pub blocks: Vec<ROMBlock>,
}

/// A contiguous block of ROM code or data, with optional metadata.
///
/// If this is a Code block, the first instruction will be a known jump destination,
/// and all of the other instructions will not be known jump destinations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ROMBlock {
    /// The code or data in the block.
    pub content: ROMBlockContent,
    /// An optional address that this block must be located at in the compiled output.
    pub address: Option<u16>,
}

/// A contiguous block of ROM code or data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ROMBlockContent {
    /// A block of instructions.
    Code(Vec<Instruction>),
    /// A block of raw binary data.
    Data(Vec<u8>),
}

/// A ROM of compiled machine code bytes, potentially with their decoded instruction values attached.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssembledROM {
    /// The compiled bytes of the ROM with associated disassembly information.
    pub bytes: Vec<ROMByte>,
}

/// A ROM byte and inferred information about its role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ROMByte {
    /// The raw byte value.
    pub byte: u8,
    /// Current inferred information about the byte's role in the ROM.
    pub role: ROMByteRole,
}

/// Potential roles a byte can have in a ROM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ROMByteRole {
    /// This may be data, unused, or code we don't understand.
    Unknown,
    /// The initial byte of an instruction; a point at which we can begin parsing.
    InstructionStart(Instruction, IsJumpDestination),
    /// The non-initial byte of an instruction.
    InstructionRest,
}

/// Whether we are confident an address is used as a jump destination in the program.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IsJumpDestination {
    /// We don't know whether this instruction is a potential jump destination.
    Unknown,
    /// We are confident that this is a potential jump destination in the code.
    Yes,
}

impl Display for DisassembledROM {
    /// Encodes this ROM as a pseudo-assembly string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in self.blocks.iter() {
            Display::fmt(&block, f)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Display for ROMBlock {
    /// Encodes this block as a pseudo-assembly string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(address) = self.address {
            write!(f, "0x{:04X}:\n", address)?;
        } else {
            write!(f, "0x____:\n")?;
        }

        match self.content {
            Data(ref bytes) => {
                let mut n = 0;
                for byte in bytes.iter() {
                    if n == 0 {
                        write!(f, "    DATA 0x")?;
                        n += 6;
                    }

                    write!(f, "{:02X}", byte)?;
                    n += 2;

                    if n >= 61 {
                        write!(f, "\n")?;
                        n = 0;
                    }
                }
                write!(f, "\n")?;
            }
            Code(ref instructions) => {
                // TODO: exclude trailing padding NOPs
                for instruction in instructions.iter() {
                    write!(f, "    {}\n", instruction)?;
                }
            }
        }
        Ok(())
    }
}

impl From<&AssembledROM> for DisassembledROM {
    fn from(_assembled: &AssembledROM) -> Self {
        unimplemented!()
    }
}

impl AssembledROM {
    /// Creates a new `AssembledROM` of the given raw bytes, with their roles
    /// inferred where possible from constant known instruction addresses.
    pub fn new(bytes: &Vec<u8>) -> Self {
        let mut assembled = Self::from(bytes);

        // For now, we're pretending that 0x0000 is the only known constant instruction address.
        assembled.add_known_instruction_address(0x0000);
        // In reality, 0x0000 is a constant instruction address for the boot ROM, but for games
        // it's not, and the actual constant instruction addresses are the entry point at 0x0100 and
        // the interrupt handlers at 0x0040, 0x0048, 0x0050, and 0x0048.

        assembled
    }

    /// Updates byte role information give that the byte at entry_point is the beginning
    /// of an instruction.
    pub fn add_known_instruction_address(&mut self, _address: u16) {
        unimplemented!()
    }

    /// Returns the instruction starting at the specified address, which may need to be newly decoded.
    ///
    /// If this instruction was not previously decoded, this will also decode the roles of following
    /// bytes that can now be decoded.
    pub fn get_instruction(&mut self, address: u16) -> Instruction {
        self.add_known_instruction_address(address);
        if let ROMByteRole::InstructionStart(instruction, IsJumpDestination::Yes) =
            self.bytes[usize::from(address)].role
        {
            instruction
        } else {
            unreachable!();
        }
    }
}

impl From<&DisassembledROM> for AssembledROM {
    fn from(_assembled: &DisassembledROM) -> Self {
        unimplemented!()
    }
}

impl From<&Vec<u8>> for AssembledROM {
    fn from(bytes: &Vec<u8>) -> Self {
        Self {
            bytes: bytes.iter().map(|byte| byte.clone().into()).collect(),
        }
    }
}

impl From<ROMBlockContent> for ROMBlock {
    fn from(content: ROMBlockContent) -> Self {
        Self {
            content,
            address: None,
        }
    }
}

impl From<u8> for ROMByte {
    fn from(byte: u8) -> Self {
        Self {
            byte,
            role: ROMByteRole::Unknown,
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
