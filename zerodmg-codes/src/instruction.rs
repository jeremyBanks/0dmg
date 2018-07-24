use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::fmt;
use std::fmt::Display;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use super::Instruction;
    pub use super::Instruction::*;
    pub use super::U16Register::*;
    pub use super::U8Register::*;
}

use self::prelude::*;

/// A single CPU instruction, including any immediate arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    /// No instruction
    NOP,
    /// Increment u8 register
    INC(U8Register),
    /// Decrement u8 register
    DEC(U8Register),
    /// Conditional absolute jump, if Z flag bit is non-zero
    JP_NZ(u16),
    /// Unconditional absolute jump
    JP(u16),
    /// Uncondition relative jump
    JR(i8),
    /// Load immediate bytes into a 16-bit register.
    LD_16_IMMEDIATE(U16Register, u16),
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

impl Instruction {
    /// Decodes machine code bytes from the iterator to an Instruction.
    ///
    /// Returns [None] if the iterator is exhausted.
    pub fn from_byte_iter(bytes: &mut Iterator<Item = u8>) -> Option<Self> {
        if let Some(first) = bytes.next() {
            Some(match first {
                0x00 => NOP,
                0x01 => {
                    let low = bytes.next().unwrap();
                    let high = bytes.next().unwrap();
                    let address = u8s_to_u16(low, high);
                    LD_16_IMMEDIATE(BC, address)
                }
                0x04 => INC(B),
                0x0C => INC(C),
                0x11 => {
                    let low = bytes.next().unwrap();
                    let high = bytes.next().unwrap();
                    let address = u8s_to_u16(low, high);
                    LD_16_IMMEDIATE(DE, address)
                }
                0x14 => INC(D),
                0x18 => {
                    let offset = bytes.next().unwrap() as i8;
                    JR(offset)
                }
                0x1C => INC(E),
                0x21 => {
                    let low = bytes.next().unwrap();
                    let high = bytes.next().unwrap();
                    let address = u8s_to_u16(low, high);
                    LD_16_IMMEDIATE(HL, address)
                }
                0x24 => INC(H),
                0x2C => INC(L),
                0x31 => {
                    let low = bytes.next().unwrap();
                    let high = bytes.next().unwrap();
                    let address = u8s_to_u16(low, high);
                    LD_16_IMMEDIATE(SP, address)
                }
                0x34 => INC(AT_HL),
                0x3C => INC(A),
                0x05 => DEC(B),
                0x0D => DEC(C),
                0x15 => DEC(D),
                0x1D => DEC(E),
                0x25 => DEC(H),
                0x2D => DEC(L),
                0x35 => DEC(AT_HL),
                0x3D => DEC(A),
                0xC2 => {
                    let low = bytes.next().unwrap();
                    let high = bytes.next().unwrap();
                    let address = u8s_to_u16(low, high);
                    JP_NZ(address)
                }
                0xC3 => {
                    let low = bytes.next().unwrap();
                    let high = bytes.next().unwrap();
                    let address = u8s_to_u16(low, high);
                    JP(address)
                }
                _ => unimplemented!(),
            })
        } else {
            None
        }
    }

    /// The number of bytes this instruction will occupy in the ROM.
    pub fn byte_length(&self) -> u16 {
        match self {
            NOP => 1,
            INC(_) => 1,
            DEC(_) => 1,
            JP_NZ(_) => 3,
            JP(_) => 3,
            JR(_) => 2,
            LD_16_IMMEDIATE(_, _) => 3,
        }
    }

    /// Encodes this instruction back into machine code bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = match self {
            NOP => vec![0x00],
            INC(register) => vec![0x04 + 0b00_001_000 * register.index()],
            DEC(register) => vec![0x05 + 0b00_001_000 * register.index()],
            JP_NZ(address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0xC2, low, high]
            },
            JP(address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0xC3, low, high]
            },
            JR(offset) => vec![0x18, *offset as u8],
            LD_16_IMMEDIATE(register, value) => {
                let (low, high) = u16_to_u8s(*value);
                let instruction = 0x01 + 0b0001_0000 * register.index();
                vec![instruction, low, high]
            },
        };

        assert_eq!(bytes.len(), self.byte_length().into());

        bytes
    }
}

impl Display for Instruction {
    /// Encodes this instruction as a pseudo-assembly string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NOP => write!(f, "NOP"),
            INC(register) => write!(f, "INC {:?}", register),
            DEC(register) => write!(f, "DEC {:?}", register),
            JP(address) => write!(f, "JP 0x{:04X}", address),
            JP_NZ(address) => write!(f, "JP NZ 0x{:04X}", address),
            JR(offset) => write!(f, "JR 0x{:02X}", offset),
            LD_16_IMMEDIATE(register, value) => write!(f, "LD {:?} 0x{:04X}", register, value),
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
}
