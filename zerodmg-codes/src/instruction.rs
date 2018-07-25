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
    /// Loads immediates bytes into an 8-bit register.
    LD_8_IMMEDIATE(U8Register, u8),
    /// Loads a value from one 8-bit register into another.
    LD_8_INTERNAL(U8Register, U8Register),
    /// Pops PC+AF from the stack (return from call).
    RET,
    /// Pops PC+AF from the stack and reenables interrupts (return from
    /// interrupt).
    RETI,
    /// Stops running the CPU until an interrupt occurs.
    HALT,
    /// Halt and Catch Fire - invalid opcode used to panic in unexpected
    /// situations.
    HCF,
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

            Some(match first {
                0x00 => NOP,

                0x01 => LD_16_IMMEDIATE(BC, d16(bytes)),
                0x11 => LD_16_IMMEDIATE(DE, d16(bytes)),
                0x21 => LD_16_IMMEDIATE(HL, d16(bytes)),
                0x31 => LD_16_IMMEDIATE(SP, d16(bytes)),

                0x04 => INC(B),
                0x0C => INC(C),
                0x14 => INC(D),
                0x18 => JR(r8(bytes)),
                0x1C => INC(E),
                0x24 => INC(H),
                0x2C => INC(L),
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

                0x06 => LD_8_IMMEDIATE(B, d8(bytes)),
                0x0E => LD_8_IMMEDIATE(C, d8(bytes)),
                0x16 => LD_8_IMMEDIATE(D, d8(bytes)),
                0x1E => LD_8_IMMEDIATE(E, d8(bytes)),
                0x26 => LD_8_IMMEDIATE(H, d8(bytes)),
                0x2E => LD_8_IMMEDIATE(L, d8(bytes)),
                0x36 => LD_8_IMMEDIATE(AT_HL, d8(bytes)),
                0x3E => LD_8_IMMEDIATE(A, d8(bytes)),

                0x40 => LD_8_INTERNAL(B, B),
                0x41 => LD_8_INTERNAL(B, C),
                0x42 => LD_8_INTERNAL(B, D),
                0x43 => LD_8_INTERNAL(B, E),
                0x44 => LD_8_INTERNAL(B, H),
                0x45 => LD_8_INTERNAL(B, L),
                0x46 => LD_8_INTERNAL(B, AT_HL),
                0x47 => LD_8_INTERNAL(B, A),
                0x48 => LD_8_INTERNAL(C, B),
                0x49 => LD_8_INTERNAL(C, C),
                0x4A => LD_8_INTERNAL(C, D),
                0x4B => LD_8_INTERNAL(C, E),
                0x4C => LD_8_INTERNAL(C, H),
                0x4D => LD_8_INTERNAL(C, L),
                0x4E => LD_8_INTERNAL(C, AT_HL),
                0x4F => LD_8_INTERNAL(C, A),
                0x50 => LD_8_INTERNAL(D, B),
                0x51 => LD_8_INTERNAL(D, C),
                0x52 => LD_8_INTERNAL(D, D),
                0x53 => LD_8_INTERNAL(D, E),
                0x54 => LD_8_INTERNAL(D, H),
                0x55 => LD_8_INTERNAL(D, L),
                0x56 => LD_8_INTERNAL(D, AT_HL),
                0x57 => LD_8_INTERNAL(D, A),
                0x58 => LD_8_INTERNAL(E, B),
                0x59 => LD_8_INTERNAL(E, C),
                0x5A => LD_8_INTERNAL(E, D),
                0x5B => LD_8_INTERNAL(E, E),
                0x5C => LD_8_INTERNAL(E, H),
                0x5D => LD_8_INTERNAL(E, L),
                0x5E => LD_8_INTERNAL(E, AT_HL),
                0x5F => LD_8_INTERNAL(E, A),
                0x60 => LD_8_INTERNAL(H, B),
                0x61 => LD_8_INTERNAL(H, C),
                0x62 => LD_8_INTERNAL(H, D),
                0x63 => LD_8_INTERNAL(H, E),
                0x64 => LD_8_INTERNAL(H, H),
                0x65 => LD_8_INTERNAL(H, L),
                0x66 => LD_8_INTERNAL(H, AT_HL),
                0x67 => LD_8_INTERNAL(H, A),
                0x68 => LD_8_INTERNAL(L, B),
                0x69 => LD_8_INTERNAL(L, C),
                0x6A => LD_8_INTERNAL(L, D),
                0x6B => LD_8_INTERNAL(L, E),
                0x6C => LD_8_INTERNAL(L, H),
                0x6D => LD_8_INTERNAL(L, L),
                0x6E => LD_8_INTERNAL(L, AT_HL),
                0x6F => LD_8_INTERNAL(L, A),
                0x70 => LD_8_INTERNAL(AT_HL, B),
                0x71 => LD_8_INTERNAL(AT_HL, C),
                0x72 => LD_8_INTERNAL(AT_HL, D),
                0x73 => LD_8_INTERNAL(AT_HL, E),
                0x74 => LD_8_INTERNAL(AT_HL, H),
                0x75 => LD_8_INTERNAL(AT_HL, L),
                0x77 => LD_8_INTERNAL(AT_HL, A),
                0x78 => LD_8_INTERNAL(A, B),
                0x79 => LD_8_INTERNAL(A, C),
                0x7A => LD_8_INTERNAL(A, D),
                0x7B => LD_8_INTERNAL(A, E),
                0x7C => LD_8_INTERNAL(A, H),
                0x7D => LD_8_INTERNAL(A, L),
                0x7E => LD_8_INTERNAL(A, AT_HL),
                0x7F => LD_8_INTERNAL(A, A),

                0x76 => HALT,

                0xC2 => JP_NZ(d16(bytes)),
                0xC3 => JP(d16(bytes)),
                0xC9 => RET,
                0xD9 => RETI,

                0xDD => HCF,

                _ => unimplemented!("unsupported instruction code 0x{:02X}", first),
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
            LD_8_IMMEDIATE(_, _) => 2,
            LD_8_INTERNAL(_, _) => 1,
            HALT => 1,
            HCF => 1,
            RET => 1,
            RETI => 1,
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
            }
            JP(address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0xC3, low, high]
            }
            JR(offset) => vec![0x18, *offset as u8],
            LD_16_IMMEDIATE(register, value) => {
                let (low, high) = u16_to_u8s(*value);
                let instruction = 0x01 + 0b00_01_0000 * register.index();
                vec![instruction, low, high]
            }
            LD_8_IMMEDIATE(register, value) => {
                let instruction = 0x06 + 0b00_001_000 * register.index();
                vec![instruction, *value]
            }
            LD_8_INTERNAL(dest, source) => {
                vec![0x40 + 0b00_001_000 * dest.index() + source.index()]
            }
            HALT => vec![0x76],
            RET => vec![0xC9],
            RETI => vec![0xD9],
            HCF => vec![0xDD],
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
            LD_8_IMMEDIATE(register, value) => write!(f, "LD {:?} 0x{:02X}", register, value),
            LD_8_INTERNAL(dest, source) => write!(f, "LD {:?} {:?}", dest, source),
            HALT => write!(f, "HALT"),
            RET => write!(f, "RET"),
            RETI => write!(f, "RETI"),
            HCF => write!(f, "HCF!"),
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
