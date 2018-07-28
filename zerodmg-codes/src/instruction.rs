use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::fmt;
use std::fmt::Display;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use super::FlagCondition;
    pub use super::FlagCondition::*;
    pub use super::Instruction;
    pub use super::Instruction::*;
    pub use super::InvalidOpcode;
    pub use super::InvalidOpcode::*;
    pub use super::RSTTarget::*;
    pub use super::U16Register::*;
    pub use super::U8Register::*;
    pub use super::U8SecondaryRegister::*;
    pub use super::LD;
}

use self::prelude::*;

/// A single CPU instruction, including any immediate arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    // Control
    /// No instruction.
    /// Used for padding or delay.
    NOP,
    /// Stops running the CPU until an interrupt occurs.
    HALT,
    /// Invalid opcodes.
    /// Used to panic in unexpected situations.
    HCF(InvalidOpcode),

    // 8-Bit Arithmatic and Logic
    /// Increment u8 register
    INC(U8Register),
    /// Decrement u8 register
    DEC(U8Register),
    // A += x
    ADD(U8Register),
    // A += x + Z_C
    ADC(U8Register),
    // A -= x
    SUB(U8Register),
    // A -= x + Z_C
    SBC(U8Register),
    // A &= x
    AND(U8Register),
    // A ^= x
    XOR(U8Register),
    // A |= x
    OR(U8Register),
    // (A - x)
    // Updates flags, but doesn't update any other registers.
    CP(U8Register),

    // 16-Bit Arithmatic and Logic

    // 8-Bit Bitwise Operations (0xCB Opcodes)

    // 8-Bit Loads
    /// Loads the value from one 8-bit register into another.
    LD_8_INTERNAL(U8Register, U8Register),
    /// Loads the value from A into a secondary pointer/register.
    LD_8_TO_SECONDARY(U8SecondaryRegister),
    /// Loads the value from secondary pointer/register into A.
    LD_8_FROM_SECONDARY(U8SecondaryRegister),
    /// Loads immediates bytes into an 8-bit register.
    LD_8_IMMEDIATE(U8Register, u8),

    // 16-Bit Loads
    /// Load immediate bytes into a 16-bit register.
    LD_16_IMMEDIATE(U16Register, u16),

    // Jumps and Calls
    /// Unconditional absolute jump, updates PC.
    JP(u16),
    /// Conditional absolute jump, may update PC.
    JP_IF(FlagCondition, u16),
    /// Uncondition relative jump, updates PC.
    JR(i8),
    /// Conditional relative jump, may update PC.
    JR_IF(FlagCondition, i8),
    /// Unconditional call. Pushes PC on the stack, then updates it.
    CALL(u16),
    /// Conditional call. Pushes PC on the stack, then updates it.
    CALL_IF(FlagCondition, u16),
    /// Jumps to a hard-coded single-byte address.
    RST(RSTTarget),
    /// Pops PC from the stack (return from call).
    RET,
    /// Pops PC from the stack and reenables interrupts (return from
    /// interrupt).
    RETI,
}

/// Flag conditions that can be used by branching instructions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum FlagCondition {
    /// Zero flag bit is not set; last instruction had non-zero result.
    if_NZ,
    /// Zero flag bit is set; last instruction had zero result.
    if_Z,
    /// Carry flag bit is not set; last instruction did not overflow.
    if_NC,
    /// Carry flag bit is set; last instruction overflowed.
    if_C,
}

impl FlagCondition {
    fn index(self) -> u8 {
        match self {
            if_NZ => 0,
            if_Z => 1,
            if_NC => 2,
            if_C => 3,
        }
    }

    pub fn from_index(value: u8) -> Self {
        match value {
            0 => if_NZ,
            1 => if_Z,
            2 => if_NC,
            3 => if_C,
            _ => panic!("invalid FlagCondition index"),
        }
    }
}

/// The 8-bit registers that are available in the CPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum U8Register {
    /// Primary accumulator register
    A,
    /// May be paired with [C] as 16-bit [BC] register.
    B,
    /// May be paired with [B] as 16-bit [BC] register.
    C,
    /// May be paired with [D] as 16-bit [DE] register.
    D,
    /// May be paired with [E] as 16-bit [DE] register.
    E,
    /// High byte of 16-bit [HL] memory pointer register.
    H,
    /// Low byte of 16-bit [HL] memory pointer register.
    L,
    /// Value in memory address represented indicated by [H] and [L] registers.
    AT_HL,
}

/// The 16-bit registers that are available in the CPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum U16Register {
    /// Combines the accumulator register and the internal flag register.
    ///
    /// Combines the [B] and [C] 8-bit registers.
    BC,
    /// Combines the [D] and [E] 8-bit registers.
    DE,
    /// Memory pointer register, combining the [H] and [L] 8-bit registers.
    HL,
    /// Stack pointer register
    SP,
}

/// The 16-bit registers that are available in the CPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum U8SecondaryRegister {
    /// Value in memory address represented indicated by [BC] register.
    AT_BC,
    /// Value in memory address represented indicated by [BC] register.
    AT_DE,
    /// Value in memory address represented indicated by [HL] register,
    /// which is then incremented.
    AT_HL_Plus,
    /// Value in memory address represented indicated by [HL] register,
    /// which is then decremented.
    AT_HL_Minus,
}

/// Addresses that can be called by single-byte RST instructions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types, missing_docs)]
pub enum RSTTarget {
    to00,
    to08,
    to10,
    to18,
    to20,
    to28,
    to30,
    to38,
}

impl RSTTarget {
    pub fn address(self) -> u8 {
        match self {
            to00 => 0x00,
            to08 => 0x08,
            to10 => 0x10,
            to18 => 0x08,
            to20 => 0x20,
            to28 => 0x28,
            to30 => 0x30,
            to38 => 0x38,
        }
    }

    pub fn from_address(value: u8) -> Self {
        match value {
            0x00 => to00,
            0x08 => to08,
            0x10 => to10,
            0x08 => to18,
            0x20 => to20,
            0x28 => to28,
            0x30 => to30,
            0x38 => to38,
            _ => panic!("invalid RST target"),
        }
    }
}

/// Invalid instruction opcodes.
/// These should never be executed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types, missing_docs)]
pub enum InvalidOpcode {
    xxD3,
    xxDB,
    xxDD,
    xxE3,
    xxE4,
    xxEB,
    xxEC,
    xxED,
    xxF4,
    xxFC,
    xxFD,
}

impl InvalidOpcode {
    fn opcode(self) -> u8 {
        match self {
            xxD3 => 0xD3,
            xxDB => 0xDB,
            xxDD => 0xDD,
            xxE3 => 0xE3,
            xxE4 => 0xE4,
            xxEB => 0xEB,
            xxEC => 0xEC,
            xxED => 0xED,
            xxF4 => 0xF4,
            xxFC => 0xFC,
            xxFD => 0xFD,
        }
    }

    pub fn from_opcode(value: u8) -> Self {
        match value {
            0xD3 => xxD3,
            0xDB => xxDB,
            0xDD => xxDD,
            0xE3 => xxE3,
            0xE4 => xxE4,
            0xEB => xxEB,
            0xEC => xxEC,
            0xED => xxED,
            0xF4 => xxF4,
            0xFC => xxFC,
            0xFD => xxFD,
            _ => panic!("invalid invalid opcode"),
        }
    }
}

impl Instruction {
    /// Encodes this instruction back into machine code bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = match self {
            // Control
            NOP => vec![0x00],
            HALT => vec![0x76],
            HCF(variant) => vec![variant.opcode()],
            // 8-Bit Arithmatic and Logic
            INC(register) => vec![0x04 + (register.index() << 3)],
            DEC(register) => vec![0x05 + (register.index() << 3)],
            ADD(register) => vec![0x80 + register.index()],
            ADC(register) => vec![0x88 + register.index()],
            SUB(register) => vec![0x90 + register.index()],
            SBC(register) => vec![0x98 + register.index()],
            AND(register) => vec![0xA0 + register.index()],
            XOR(register) => vec![0xA8 + register.index()],
            OR(register) => vec![0xB0 + register.index()],
            CP(register) => vec![0xB8 + register.index()],
            // 16-Bit Arithmatic and Logic
            // 8-Bit Bitwise Operations (0xCB Opcodes)
            // 8-Bit Loads
            LD_8_INTERNAL(dest, source) => vec![0x40 + (dest.index() << 3) + source.index()],
            LD_8_IMMEDIATE(register, value) => vec![0x06 + (register.index() << 3), *value],
            LD_8_TO_SECONDARY(register) => vec![0x02 + (register.index() << 4)],
            LD_8_FROM_SECONDARY(register) => vec![0x0A + (register.index() << 4)],
            // 16-Bit Loads
            LD_16_IMMEDIATE(register, value) => {
                let (low, high) = u16_to_u8s(*value);
                let opcode = 0x01 + (register.index() << 4);
                vec![opcode, low, high]
            }
            // Jumps and Calls
            JP(address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0xC3, low, high]
            }
            JP_IF(condition, address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0xC2 + (condition.index() << 3), low, high]
            }
            JR(offset) => vec![0x18, *offset as u8],
            JR_IF(condition, offset) => vec![0x20 + (condition.index() << 3), *offset as u8],
            CALL(address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0xCD, low, high]
            }
            CALL_IF(condition, address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0xC4 + (condition.index() << 3), low, high]
            }
            RST(target) => vec![0xC7 + target.address()],
            RET => vec![0xC9],
            RETI => vec![0xD9],
        };

        assert_eq!(bytes.len(), self.byte_len().into());

        bytes
    }

    /// Decodes machine code bytes from the iterator to an Instruction.
    ///
    /// Returns [None] if the iterator is exhausted.
    pub fn from_byte_iter(bytes: &mut Iterator<Item = u8>) -> Option<Self> {
        if let Some(opcode) = bytes.next() {
            fn d8(bytes: &mut Iterator<Item = u8>) -> u8 {
                bytes.next().expect("unexpected end of ROM byte iterator")
            };
            fn d16(bytes: &mut Iterator<Item = u8>) -> u16 {
                let low = d8(bytes);
                let high = d8(bytes);
                u8s_to_u16(low, high)
            };
            fn r8(bytes: &mut Iterator<Item = u8>) -> i8 {
                d8(bytes) as i8
            };

            Some(match opcode {
                // Control
                0x00 => NOP,
                0x76 => HALT,
                0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                    HCF(InvalidOpcode::from_opcode(opcode))
                }
                // 8-Bit Arithmatic and Logic
                0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                    INC(U8Register::from_index(0b111 & (opcode >> 3)))
                }
                0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
                    DEC(U8Register::from_index(0b111 & (opcode >> 3)))
                }
                0x80..=0x87 => ADD(U8Register::from_index(0b111 & opcode)),
                0x88..=0x8F => ADC(U8Register::from_index(0b111 & opcode)),
                0x90..=0x97 => SUB(U8Register::from_index(0b111 & opcode)),
                0x98..=0x9F => SBC(U8Register::from_index(0b111 & opcode)),
                0xA0..=0xA7 => AND(U8Register::from_index(0b111 & opcode)),
                0xA8..=0xAF => XOR(U8Register::from_index(0b111 & opcode)),
                0xB0..=0xB7 => OR(U8Register::from_index(0b111 & opcode)),
                0xB8..=0xBF => CP(U8Register::from_index(0b111 & opcode)),
                // 16-Bit Arithmatic and Logic
                // 8-Bit Bitwise Operations (0xCB Opcodes)
                0xCB => Self::cb_instruction(d8(bytes)),
                // 8-Bit Loads
                0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                    let register = U8Register::from_index(0b111 & (opcode >> 3));
                    LD_8_IMMEDIATE(register, d8(bytes))
                }
                0x02 | 0x12 | 0x22 | 0x32 => {
                    LD_8_TO_SECONDARY(U8SecondaryRegister::from_index(0b11 & (opcode >> 4)))
                }
                0x0A | 0x1A | 0x2A | 0x3A => {
                    LD_8_FROM_SECONDARY(U8SecondaryRegister::from_index(0b11 & (opcode >> 4)))
                }
                0x40..=0x75 | 0x77..=0x7F => {
                    let dest = U8Register::from_index(0b111 & (opcode >> 3));
                    let source = U8Register::from_index(0b111 & opcode);
                    LD_8_INTERNAL(dest, source)
                }
                // 16-Bit Loads
                0x01 | 0x11 | 0x21 | 0x31 => {
                    let register = U16Register::from_index(0b11 & (opcode >> 4));
                    LD(register, d16(bytes))
                }
                // Jumps and Calls
                0xC3 => JP(d16(bytes)),
                0xC2 | 0xCA | 0xD2 | 0xDA => {
                    let condition = FlagCondition::from_index(0b11 & (opcode >> 3));
                    JP_IF(condition, d16(bytes))
                }
                0x18 => JR(r8(bytes)),
                0x20 | 0x28 | 0x30 | 0x38 => {
                    let condition = FlagCondition::from_index(0b11 & (opcode >> 3));
                    JR_IF(condition, r8(bytes))
                }
                0xCD => CALL(d16(bytes)),
                0xC4 | 0xCC | 0xD4 | 0xDC => {
                    let condition = FlagCondition::from_index(0b11 & (opcode >> 3));
                    CALL_IF(condition, d16(bytes))
                }
                0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                    let target = RSTTarget::from_address(opcode - 0xC7);
                    RST(target)
                }
                0xC9 => RET,
                0xD9 => RETI,
                // TODO: implement everything
                _ => unimplemented!("unsupported instruction code 0x{:02X}", opcode),
            })
        } else {
            None
        }
    }

    fn cb_instruction(second_byte: u8) -> Instruction {
        match second_byte {
            _ => unimplemented!(
                "8-bit bitwise opcode 0xCB 0x{:02X} not implemented",
                second_byte
            ),
        }
    }

    /// The number of bytes this instruction will occupy in the ROM.
    pub fn byte_len(&self) -> u16 {
        match self {
            // Control
            NOP => 1,
            HALT => 1,
            HCF(_) => 1,
            // 8-Bit Arithmatic and Logic
            INC(_) => 1,
            DEC(_) => 1,
            ADD(_) => 1,
            ADC(_) => 1,
            SUB(_) => 1,
            SBC(_) => 1,
            AND(_) => 1,
            XOR(_) => 1,
            OR(_) => 1,
            CP(_) => 1,
            // 16-Bit Arithmatic and Logic
            // 8-Bit Bitwise Operations (0xCB Opcodes)
            // 8-Bit Loads
            LD_8_INTERNAL(_, _) => 1,
            LD_8_IMMEDIATE(_, _) => 2,
            LD_8_TO_SECONDARY(_) => 1,
            LD_8_FROM_SECONDARY(_) => 1,
            // 16-Bit Loads
            LD_16_IMMEDIATE(_, _) => 3,
            // Jumps and Calls
            JP_IF(_, _) => 3,
            JP(_) => 3,
            JR_IF(_, _) => 2,
            JR(_) => 2,
            CALL_IF(_, _) => 3,
            CALL(_) => 3,
            RST(_) => 1,
            RET => 1,
            RETI => 1,
        }
    }
}

impl Display for Instruction {
    /// Encodes this instruction as a pseudo-assembly string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Control
            NOP => write!(f, "NOP"),
            HALT => write!(f, "HALT"),
            HCF(variant) => write!(f, "HCF {:?} !!!", variant),
            // 8-Bit Arithmatic and Logic
            INC(register) => write!(f, "INC {:?}", register),
            DEC(register) => write!(f, "DEC {:?}", register),
            ADD(register) => write!(f, "ADD {:?}", register),
            ADC(register) => write!(f, "ADC {:?}", register),
            SUB(register) => write!(f, "SUB {:?}", register),
            SBC(register) => write!(f, "SBC {:?}", register),
            AND(register) => write!(f, "AND {:?}", register),
            XOR(register) => write!(f, "XOR {:?}", register),
            OR(register) => write!(f, "OR {:?}", register),
            CP(register) => write!(f, "CP {:?}", register),
            // 16-Bit Arithmatic and Logic
            // 8-Bit Bitwise Operations (0xCB Opcodes)
            // 8-Bit Loads
            LD_8_INTERNAL(dest, source) => write!(f, "LD {:?} {:?}", dest, source),
            LD_8_TO_SECONDARY(dest) => write!(f, "LD {:?} A", dest),
            LD_8_FROM_SECONDARY(source) => write!(f, "LD A {:?}", source),
            LD_8_IMMEDIATE(register, value) => write!(f, "LD {:?} 0x{:02X}", register, value),
            // 16-Bit Loads
            LD_16_IMMEDIATE(register, value) => write!(f, "LD {:?} 0x{:04X}", register, value),
            // Jumps and Calls
            JP(address) => write!(f, "JP 0x{:04X}", address),
            JP_IF(condition, address) => write!(f, "JP {:?} 0x{:04X}", condition, address),
            JR(offset) => write!(f, "JR 0x{:02X}", offset),
            JR_IF(condition, address) => write!(f, "JR {:?} 0x{:04X}", condition, address),
            CALL(address) => write!(f, "CALL 0x{:04X}", address),
            CALL_IF(condition, address) => write!(f, "CALL {:?} 0x{:04X}", condition, address),
            RST(target) => write!(f, "RST 0x{:02X}", target.address()),
            RET => write!(f, "RET"),
            RETI => write!(f, "RETI"),
        }
    }
}

impl U8Register {
    /// The integer/bit pattern representing this register in the machine code.
    pub fn index(self) -> u8 {
        match self {
            A => 0b111,
            B => 0b000,
            C => 0b001,
            D => 0b010,
            E => 0b011,
            H => 0b100,
            L => 0b101,
            AT_HL => 0b110,
        }
    }

    pub fn from_index(value: u8) -> Self {
        match value {
            0b111 => A,
            0b000 => B,
            0b001 => C,
            0b010 => D,
            0b011 => E,
            0b100 => H,
            0b101 => L,
            0b110 => AT_HL,
            _ => panic!("invalid U8Register index"),
        }
    }
}

impl U16Register {
    /// The integer/bit pattern representing this register in the machine code.
    pub fn index(self) -> u8 {
        match self {
            BC => 0b00,
            DE => 0b01,
            HL => 0b10,
            SP => 0b11,
        }
    }

    pub fn from_index(value: u8) -> Self {
        match value {
            0b00 => BC,
            0b01 => DE,
            0b10 => HL,
            0b11 => SP,
            _ => panic!("invalid U16Register index"),
        }
    }
}
impl U8SecondaryRegister {
    /// The integer/bit pattern representing this register in the machine code.
    pub fn index(self) -> u8 {
        match self {
            AT_BC => 0b00,
            AT_DE => 0b01,
            AT_HL_Plus => 0b10,
            AT_HL_Minus => 0b11,
        }
    }

    pub fn from_index(value: u8) -> Self {
        match value {
            0b00 => AT_BC,
            0b01 => AT_DE,
            0b10 => AT_HL_Plus,
            0b11 => AT_HL_Minus,
            _ => panic!("invalid U8SecondaryRegister index"),
        }
    }
}

pub fn LD<A: Ld<B>, B>(a: A, b: B) -> Instruction {
    Ld::LD(a, b)
}

pub trait Ld<Source> {
    fn LD(destination: Self, source: Source) -> Instruction;
}

impl Ld<u8> for U8Register {
    fn LD(destination: U8Register, value: u8) -> Instruction {
        LD_8_IMMEDIATE(destination, value)
    }
}

impl Ld<u16> for U16Register {
    fn LD(destination: U16Register, value: u16) -> Instruction {
        LD_16_IMMEDIATE(destination, value)
    }
}

impl Ld<U8Register> for U8Register {
    fn LD(destination: U8Register, source: U8Register) -> Instruction {
        LD_8_INTERNAL(destination, source)
    }
}

impl Ld<U8SecondaryRegister> for U8Register {
    fn LD(destination: U8Register, source: U8SecondaryRegister) -> Instruction {
        match destination {
            A => LD_8_FROM_SECONDARY(source),
            _ => panic!(
                "You can only LD a U8SecondaryRegister to A, not {:?}.",
                destination
            ),
        }
    }
}

impl Ld<U8Register> for U8SecondaryRegister {
    fn LD(destination: U8SecondaryRegister, source: U8Register) -> Instruction {
        match source {
            A => LD_8_TO_SECONDARY(destination),
            _ => panic!(
                "You can only LD a U8SecondaryRegister from A, not {:?}.",
                destination
            ),
        }
    }
}
