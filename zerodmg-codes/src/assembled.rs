use std::convert::TryFrom;

use crate::disassembled::prelude::*;
use crate::instruction::prelude::*;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use super::AssembledRom;
    pub use super::IsJumpDestination;
    pub use super::RomByte;
    pub use super::RomByteRole;
}

// to considers:
// - switch is_jump_destination to a named boolean instead of an anonymous enum
// - use parallel arrays instead of RomByte structure

#[test]
fn test_disassemble() {
    let assembled = AssembledRom::example();
    let _disassembled = assembled.disassemble();
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

#[test]
fn test_bytes_from_assembled() {
    let assembled = AssembledRom::example();
    let _bytes = Vec::<u8>::from(&assembled);
}

/// A ROM of compiled machine code bytes, potentially with their decoded
/// [Instruction] values attached.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssembledRom {
    /// The compiled bytes of the ROM with associated disassembly information.
    bytes: Vec<RomByte>,
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

                self.bytes[usize::from(address)].role =
                    RomByteRole::InstructionStart(instruction, IsJumpDestination::Unknown);
                for i in (address + 1)..(address + instruction.byte_length()) {
                    self.bytes[usize::from(i)].role = RomByteRole::InstructionRest;
                }

                // TODO: trace anything, recursively is okay for now.
                match instruction.flows_to() {
                    _ => {}
                }

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

        enum BlockChange {
            None,
            New(RomBlock),
            End,
        }

        for (address, byte) in self.bytes.iter().enumerate() {
            let address = Some(u16::try_from(address).unwrap());

            let block_change = match byte.role {
                RomByteRole::InstructionStart(instruction, is_jump_destination) => {
                    // Each jump destination starts a new Code block.
                    if is_jump_destination == IsJumpDestination::Yes {
                        BlockChange::New(RomBlock {
                            address,
                            content: Code(vec![instruction]),
                        })
                    } else {
                        match current_block {
                            Some(ref mut block) => match block.content {
                                Code(ref mut vec) => {
                                    // If we're already in a Code block, append this instruction.
                                    vec.push(instruction);
                                    BlockChange::None
                                }
                                Data(_) => {
                                    if instruction != NOP {
                                        // If we're in a Data block, and this instruction isn't NOP,
                                        // start a new Code block.
                                        BlockChange::New(RomBlock {
                                            address,
                                            content: Code(vec![instruction]),
                                        })
                                    } else {
                                        // If we're in a Data block and the instruction is NOP,
                                        // end the block but ignore the NOP as padding.
                                        BlockChange::End
                                    }
                                }
                            },
                            None => {
                                if instruction != NOP {
                                    // If we're not in a block, and this instruction isn't NOP,
                                    // start a new Code block.
                                    BlockChange::New(RomBlock {
                                        address,
                                        content: Code(vec![instruction]),
                                    })
                                } else {
                                    // If we're not in a block and the instruction is NOP,
                                    // ignore it as padding.
                                    BlockChange::End
                                }
                            }
                        }
                    }
                }
                RomByteRole::InstructionRest => {
                    // Do nothing; this instruction was already handled at the InstructionStart.
                    BlockChange::None
                }
                RomByteRole::Unknown => {
                    // This byte is unknown or data role.
                    match current_block {
                        Some(ref mut block) => match block.content {
                            Data(ref mut vec) => {
                                // If we're in a Data block, append this byte.
                                vec.push(byte.byte);
                                BlockChange::None
                            }
                            // If we're in a Code block, start a new Data block.
                            Code(_) => BlockChange::New(RomBlock {
                                address,
                                content: Data(vec![byte.byte]),
                            }),
                        },
                        // If we aren't in anything, start a new Data block.
                        None => BlockChange::New(RomBlock {
                            address,
                            content: Data(vec![byte.byte]),
                        }),
                    }
                }
            };

            match block_change {
                BlockChange::None => {}
                BlockChange::New(new_block) => {
                    if let Some(ref last_block) = current_block {
                        blocks.push(last_block.clone());
                    }
                    current_block = Some(new_block);
                }
                BlockChange::End => {
                    if let Some(ref last_block) = current_block {
                        blocks.push(last_block.clone());
                    }
                    current_block = None
                }
            }
        }

        if let Some(last_block) = current_block {
            blocks.push(last_block);
        }

        for ref mut block in blocks.iter_mut() {
            match block.content {
                Code(ref mut vec) => {
                    // Strip trailing NOPs from Code blocks.
                    for i in (1..vec.len()).rev() {
                        if vec[i] == NOP {
                            vec.pop();
                        } else {
                            break;
                        }
                    }
                }
                Data(_) => {}
            }
        }

        DisassembledRom::from(blocks)
    }
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

impl From<Vec<RomByte>> for AssembledRom {
    fn from(bytes: Vec<RomByte>) -> Self {
        Self { bytes }
    }
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

impl From<u8> for RomByte {
    fn from(byte: u8) -> Self {
        Self {
            byte,
            role: RomByteRole::Unknown,
        }
    }
}
