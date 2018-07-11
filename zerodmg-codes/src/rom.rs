use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Display};

use crate::instruction::Instruction;
use crate::instruction::Instruction::*;
use crate::instruction::U16Register::*;
use crate::instruction::U8Register::*;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use super::AssembledRom;
    pub use super::DisassembledRom;
    pub use super::RomBlock;
    pub use super::RomBlockContent::*;
}

use self::prelude::*;

/// A ROM in a disassembled assembly-like structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisassembledRom {
    /// An ordered list of blocks of code or data in the ROM.
    pub blocks: Vec<RomBlock>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RomBlockContent {
    /// A block of instructions.
    Code(Vec<Instruction>),
    /// A block of raw binary data.
    Data(Vec<u8>),
}

/// A ROM of compiled machine code bytes, potentially with their decoded
/// [Instruction] values attached.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssembledRom {
    /// The compiled bytes of the ROM with associated disassembly information.
    pub bytes: Vec<RomByte>,
}

/// A ROM byte and inferred information about its role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RomByte {
    /// The raw byte value.
    pub byte: u8,
    /// Current inferred information about the byte's role in the ROM.
    pub role: RomByteRole,
}

/// Potential roles a byte can have in a ROM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RomByteRole {
    /// This may be data, unused, or code we don't understand.
    Unknown,
    /// The initial byte of an instruction; a point at which we can begin
    /// parsing.
    InstructionStart(Instruction, IsJumpDestination),
    /// The non-initial byte of an instruction.
    InstructionRest,
}

/// Whether we are confident an address is used as a jump destination in the
/// program.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IsJumpDestination {
    /// We don't know whether this instruction is a potential jump destination.
    Unknown,
    /// We are confident that this is a potential jump destination in the code.
    Yes,
}

/// Internal trait used to trace static control flow from an instruction.
trait FlowsTo {
    /// Where execution may continue following this instruction.
    fn flows_to(&self) -> ControlFlowsTo;
}

/// Possible control flow that can be statically known following this
/// instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ControlFlowsTo {
    /// Whether control may flow directly to the next instruction.
    next: bool,
    /// A potential control jump following this instruction.
    jump: Option<JumpReference>,
}

/// Potential target references for a jump instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JumpReference {
    /// A jump to an absolute address in memory.
    Absolute(u16),
    /// A jump to relative to the address *following* the current instruction.
    Relative(i8),
}

impl FlowsTo for Instruction {
    fn flows_to(&self) -> ControlFlowsTo {
        match self {
            NOP => ControlFlowsTo::next(),
            INC(_) => ControlFlowsTo::next(),
            DEC(_) => ControlFlowsTo::next(),
            JP_NZ(address) => ControlFlowsTo::next_and_jump(JumpReference::Absolute(*address)),
            JP(address) => ControlFlowsTo::jump(JumpReference::Absolute(*address)),
        }
    }
}

impl ControlFlowsTo {
    /// No known control flow from here.
    pub fn none() -> Self {
        ControlFlowsTo {
            next: false,
            jump: None,
        }
    }
    /// Control can flows to the next instruction (typical case).
    pub fn next() -> Self {
        ControlFlowsTo {
            next: true,
            jump: None,
        }
    }
    /// Control can flow to a given jump reference.
    pub fn jump(jump: JumpReference) -> Self {
        ControlFlowsTo {
            next: false,
            jump: Some(jump),
        }
    }
    /// Control can flow to the next instruction or a given jump reference.
    pub fn next_and_jump(jump: JumpReference) -> Self {
        ControlFlowsTo {
            next: false,
            jump: Some(jump),
        }
    }
}

#[test]
fn test_assemble() {
    let disassembled = DisassembledRom::example();
    let _assembled = disassembled.assemble();
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
    ///
    /// **This conversion is lossy** because for new or modified ROMs we may be
    /// unable to decode instructions back if the program structure isn't
    /// simple enough for our analysis, all flexible block addresses will
    /// become specified, and implied padding will become explicit as zeroed
    /// [Data] blocks.
    ///
    /// TODO: well, we shouldn't lose instructions: RomBlocks can preserve
    /// those. Padding should be converted to NOPs.
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
                        role: RomByteRole::InstructionStart(NOP, IsJumpDestination::Unknown),
                    })
                }
            }

            match &block.content {
                Code(instructions) => {
                    for instruction in instructions.iter() {
                        for (i, byte) in instruction.to_bytes().iter().enumerate() {
                            bytes.push(RomByte {
                                byte: *byte,
                                role: if i == 0 {
                                    RomByteRole::InstructionStart(
                                        *instruction,
                                        IsJumpDestination::Unknown,
                                    )
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

        AssembledRom { bytes }
    }
}

#[test]
fn test_disassemble() {
    let assembled = AssembledRom::example();
    let _disassembled = assembled.disassemble();
}

impl AssembledRom {
    /// Creates a new [AssembledRom] of the given raw bytes, with their roles
    /// inferred where possible from constant known instruction addresses.
    pub fn new(bytes: Vec<u8>) -> Self {
        let mut assembled = Self::from(&bytes);

        // For now, we're pretending that 0x0000 is the only known constant instruction
        // address.
        assembled.get_known_instruction(0x0000);
        // In reality, 0x0000 is a constant instruction address for the boot ROM, but
        // for games it's not, and the actual constant instruction addresses
        // are the entry point at 0x0100 and the interrupt handlers at 0x0040,
        // 0x0048, 0x0050, and 0x0048.

        assembled
    }

    /// Returns the instruction starting at the specified address, which may
    /// need to be newly decoded.
    ///
    /// If this instruction was not previously decoded, this will trace the
    /// control flow and decode the roles of following instruction bytes that
    /// can now be decoded.
    pub fn get_known_instruction(&mut self, address: u16) -> Instruction {
        let byte = self.bytes[usize::from(address)];

        match byte.role {
            RomByteRole::InstructionStart(instruction, _is_jump_destination) => instruction,

            RomByteRole::InstructionRest => panic!(
                "requested instruction address mis-aligned with previously-decoded instructions"
            ),

            RomByteRole::Unknown => {
                let instruction = {
                    let mut byte_iter = self
                        .bytes
                        .iter()
                        .skip(usize::from(address))
                        .map(|ref b| b.byte);
                    Instruction::from_byte_iter(&mut byte_iter).unwrap()
                };
                // TODO: not this
                let len = u16::try_from(instruction.to_bytes().len()).unwrap();

                self.bytes[usize::from(address)].role =
                    RomByteRole::InstructionStart(instruction, IsJumpDestination::Unknown);
                for i in (address + 1)..(address + len) {
                    self.bytes[usize::from(i)].role = RomByteRole::InstructionRest;
                }
                // TODO: also trace anything

                instruction
            }
        }
    }

    /// Returns some arbitrary value of this type.
    pub fn example() -> AssembledRom {
        AssembledRom {
            bytes: vec![
                RomByte {
                    role: RomByteRole::InstructionStart(INC(A), IsJumpDestination::Yes),
                    byte: 0x3C,
                },
                RomByte {
                    role: RomByteRole::InstructionStart(INC(A), IsJumpDestination::Unknown),
                    byte: 0x3C,
                },
                RomByte {
                    role: RomByteRole::InstructionStart(INC(B), IsJumpDestination::Unknown),
                    byte: 0x04,
                },
                RomByte {
                    role: RomByteRole::InstructionStart(INC(C), IsJumpDestination::Unknown),
                    byte: 0x0C,
                },
            ],
        }
    }

    /// Constructs a [DisassembledRom] from the bytes and current role
    /// information in an [AssembledRom]. You probably want to make sure you've
    /// added as many known instruction addresses as possible (with
    /// [AssembledRom::get_known_instruction()]) before calling this.
    ///
    /// Each byte which [IsJumpDestination::Yes] starts a new [Code] block, and
    /// contiguous [RomByteRole::Unknown] bytes are grouped into [Data] blocks.
    pub fn disassemble(&self) -> DisassembledRom {
        let mut blocks = Vec::<RomBlock>::new();
        let mut current_block: Option<RomBlock> = None;

        for (address, byte) in self.bytes.iter().enumerate() {
            let address = Some(u16::try_from(address).unwrap());
            let mut new_block: Option<RomBlock> = None;

            match byte.role {
                RomByteRole::Unknown => {
                    if let Some(ref mut block) = current_block {
                        match block.content {
                            RomBlockContent::Data(ref mut vec) => {
                                vec.push(byte.byte);
                            }
                            RomBlockContent::Code(_) => {
                                new_block = Some(RomBlock {
                                    address,
                                    content: RomBlockContent::Data(vec![byte.byte]),
                                });
                            }
                        }
                    } else {
                        new_block = Some(RomBlock {
                            address,
                            content: RomBlockContent::Data(vec![byte.byte]),
                        });
                    }
                }
                RomByteRole::InstructionStart(instruction, _is_jump_destination) => {
                    // TODO: strip trailing nops from a code block?

                    if let Some(ref mut block) = current_block {
                        match block.content {
                            RomBlockContent::Code(ref mut vec) => {
                                vec.push(instruction);
                            }
                            RomBlockContent::Data(_) => {
                                new_block = Some(RomBlock {
                                    address,
                                    content: RomBlockContent::Code(vec![instruction]),
                                });
                            }
                        }
                    } else {
                        new_block = Some(RomBlock {
                            address,
                            content: RomBlockContent::Code(vec![instruction]),
                        });
                    }
                }
                RomByteRole::InstructionRest => {
                    // do nothing; this instruction was already handled at the InstructionStart.
                }
            }

            if let Some(new_block) = new_block {
                if let Some(current_block) = current_block {
                    blocks.push(current_block);
                }
                current_block = Some(new_block);
            }
        }

        if let Some(last_block) = current_block {
            blocks.push(last_block);
        }

        DisassembledRom { blocks }
    }
}

#[test]
fn test_assembled_from_bytes_then_get_instructions_trivial() {
    let bytes = vec![0u8, 0x3C, 0x04, 0x0C];
    let mut assembled = AssembledRom::from(&bytes);
    assert_eq!(NOP, assembled.get_known_instruction(0x0000));
    assert_eq!(INC(A), assembled.get_known_instruction(0x0001));
    assert_eq!(INC(B), assembled.get_known_instruction(0x0002));
    assert_eq!(INC(C), assembled.get_known_instruction(0x0003));
    assert_eq!(NOP, assembled.get_known_instruction(0x0000));
}

impl From<&Vec<u8>> for AssembledRom {
    /// Copies bytes into a new [AssembledRom] and marks them as as
    /// [RomByteRole::Unknown].
    ///
    /// Used to load binary ROMs.
    fn from(bytes: &Vec<u8>) -> Self {
        Self {
            bytes: bytes.iter().map(|byte| byte.clone().into()).collect(),
        }
    }
}

#[test]
fn test_bytes_from_assembled() {
    let assembled = AssembledRom::example();
    let _bytes = Vec::<u8>::from(&assembled);
}
impl From<&AssembledRom> for Vec<u8> {
    /// Copies the bytes from [AssembledRom] into a new byte vector.
    ///
    /// Used to save binary ROMs.
    ///
    /// **This️ conversion is lossy** because byte role information is not
    /// preserved.
    fn from(assembled: &AssembledRom) -> Self {
        assembled.bytes.iter().map(|&byte| byte.byte).collect()
    }
}

impl From<RomBlockContent> for RomBlock {
    fn from(content: RomBlockContent) -> Self {
        Self {
            content,
            address: None,
        }
    }
}

impl From<u8> for RomByte {
    fn from(byte: u8) -> Self {
        Self {
            byte,
            role: RomByteRole::Unknown,
        }
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
                .map(|content| content.into())
                .collect(),
        }
    }
}

impl From<Vec<Instruction>> for DisassembledRom {
    fn from(instructions: Vec<Instruction>) -> Self {
        DisassembledRom {
            blocks: vec![Code(instructions).into()],
        }
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
