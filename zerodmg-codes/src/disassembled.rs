#![macro_use]

use std::fmt;
use std::fmt::Display;

use self::prelude::*;
use crate::assembled::prelude::*;
use crate::instruction::prelude::*;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use super::DisassembledRom;
    pub use super::RomBlock;
    pub use super::RomBlockContent::*;
}

#[test]
fn test_assemble() {
    let disassembled = DisassembledRom::example();
    let _assembled = disassembled.assemble();
}

/// A ROM in a disassembled assembly-like structure.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct DisassembledRom {
    /// An ordered list of blocks of code or data in the ROM.
    blocks: Vec<RomBlock>,
    // TODO: use parallel arrays instead?
}

/// A contiguous block of ROM code or data, with optional metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RomBlock {
    /// The code or data in the block.
    pub content: RomBlockContent,
    /// An optional address that this block must be located at in the compiled
    /// output.
    pub address: Option<u16>,
}

/// A contiguous block of ROM code or data.
#[derive(Clone, Debug, Eq, PartialEq, From)]
pub enum RomBlockContent {
    /// A block of instructions.
    Code(Vec<Instruction>),
    /// A block of raw binary data.
    Data(Vec<u8>),
}

impl DisassembledRom {
    /// Returns some arbitrary value of this type.
    pub fn example() -> DisassembledRom {
        DisassembledRom {
            blocks: vec![RomBlock {
                address: None,
                content: Code(vec![INC(A), INC(A), INC(B), INC(C)]),
            }],
        }
    }

    /// Creates an [AssembledRom] by compiling [Code] blocks in a
    /// [DisassembledRom], concatenating them with the [Data] blocks, and
    /// inserting zero-padding to align with specified addresses.
    ///
    /// Panics if it's not possible to match a specified address because the
    /// previous block has already written that far.
    pub fn assemble(&self) -> AssembledRom {
        let mut bytes: Vec<RomByte> = vec![];
        for block in self.blocks.iter() {
            let current_length = bytes.len();
            if let Some(address) = block.address {
                let address_usize = usize::from(address);
                if address_usize < bytes.len() {
                    panic!(
                        "target address {:X} is unsatisfiable, we are already at address {:X}",
                        address, current_length
                    )
                }

                for _padding_address in current_length..address_usize {
                    bytes.push(RomByte {
                        byte: 0x00,
                        role: RomByteRole::InstructionStart {
                            instruction: NOP,
                            known_jump_destination: false,
                        },
                    })
                }
            }

            match &block.content {
                Code(instructions) => {
                    for (i, instruction) in instructions.iter().enumerate() {
                        let is_first_instruction = i == 0;
                        for (j, byte) in instruction.to_bytes().iter().enumerate() {
                            let is_first_byte = j == 0;
                            bytes.push(RomByte {
                                byte: *byte,
                                role: if is_first_byte {
                                    RomByteRole::InstructionStart {
                                        instruction: *instruction,
                                        known_jump_destination: is_first_instruction,
                                    }
                                } else {
                                    RomByteRole::InstructionRest
                                },
                            })
                        }
                    }
                }
                Data(block_bytes) => {
                    for byte in block_bytes {
                        bytes.push(RomByte {
                            byte: *byte,
                            role: RomByteRole::Unknown,
                        })
                    }
                }
            }
        }

        AssembledRom::from(bytes)
    }
}

impl From<Vec<RomBlock>> for DisassembledRom {
    fn from(blocks: Vec<RomBlock>) -> Self {
        DisassembledRom { blocks }
    }
}

impl From<Vec<RomBlockContent>> for DisassembledRom {
    fn from(blocks_contents: Vec<RomBlockContent>) -> Self {
        DisassembledRom {
            blocks: blocks_contents
                .into_iter()
                .map(|content| RomBlock {
                    content,
                    address: None,
                }).collect(),
        }
    }
}

impl From<Vec<Instruction>> for DisassembledRom {
    fn from(instructions: Vec<Instruction>) -> Self {
        vec![Code(instructions)].into()
    }
}

impl Display for DisassembledRom {
    /// Encodes this ROM as a pseudo-assembly string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in self.blocks.iter() {
            Display::fmt(&block, f)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl RomBlock {
    /// Returns the number of bytes this block would occupy in a compiled ROM.
    pub fn byte_len(&self) -> u16 {
        match self.content {
            Data(ref bytes) => bytes.len() as u16,
            Code(ref instructions) => {
                let mut len = 0;
                for instruction in instructions {
                    len += instruction.byte_len();
                }
                len
            }
        }
    }
}

impl Display for RomBlock {
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

#[macro_export]
macro_rules! code_blocks {
    (
        $(
            $(def $id:ident)*
            $(at $address:expr =>)*
            $(next $(as $section_name:ident)* =>)*
            $([$($bytes:expr$(,)*)*])*
            $({$($instructions:expr$(;)*)*})*
            $(Data($data:expr))*
            $(Code($code:expr))*

            $(,)+
        )*
    ) => {
        {
            #[allow(non_snake_case)]
            fn f() -> Vec<RomBlock> {
                $(
                    $(let $id =)* $($address)*;
                )*

                let mut LAST = 0xFFFF;
                let mut NEXT = 0x0000;
                let mut blocks = Vec::new();
                let mut SELF = 0x000;

                let _ = LAST;
                let _ = NEXT;
                let _ = SELF;

                $({
                    $(SELF = NEXT; $(let $section_name = SELF;)*)*
                    $(SELF = $address;)*

                    let block = RomBlock {
                        address: Some(SELF),
                        content: {
                            $(
                                Data(vec![$($bytes),*])
                            )*
                            $(
                                Code(Vec::<Instruction>::from(vec![$($instructions),*]))
                            )*
                            $(Data($data.to_vec()))*
                            $(Code($code.to_vec()))*
                        }
                    };

                    LAST = SELF;
                    NEXT = LAST + block.byte_len();
                    blocks.push(block);

                    let _ = LAST;
                    let _ = NEXT;
                    let _ = SELF;
                })*

                blocks
            }
            f()
        }
    };
}
