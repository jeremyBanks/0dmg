use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::fmt::Debug;

use self::Operation::*;
use self::U16Register::*;
use self::U8Register::*;

/// A single CPU operation, including any immediate arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Operation {
    /// No operation
    NOP,
    /// Increment u8 register
    INC(U8Register),
    /// Decrement u8 register
    DEC(U8Register),
    /// Conditional absolute jump, if Z flag bit is non-zero
    JP_NZ(u16),
}

/// The 8-bit registers that are available in the CPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum U8Register {
    /// Primary accumulator register
    A,
    /// May be paired with C as 16-bit BC register.
    B,
    /// May be paired with B as 16-bit BC register.
    C,
    /// May be paired with D as 16-bit BC register.
    D,
    /// May be paired with E as 16-bit BC register.
    E,
    /// High byte of 16-bit HL memory pointer register.
    H,
    /// Low byte of 16-bit HL memory pointer register.
    L,
    /// Value in memory address represented indicated by H and L registers.
    AT_HL,
}

/// The 16-bit registers that are available in the CPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum U16Register {
    /// Combines the accumulator register and the internal flag register.
    AF,
    /// Combines the B and C 8-bit registers.
    BC,
    /// Combines the D and E 8-bit registers.
    DE,
    /// Memory pointer register, combining the H and L 8-bit registers.
    HL,
    /// Stack pointer register
    SP,
    /// Program counter register
    PC,
}

impl Operation {
    /// Decodes machine code bytes from the iterator to an Operation.
    ///
    /// Returns None if the iterator is exhausted.
    pub fn from_byte_iter(bytes: &mut Iterator<Item = &u8>) -> Option<Self> {
        if let Some(first) = bytes.next() {
            Some(match first {
                0b0000_0000 => NOP,
                0b0000_0100 => INC(B),
                0b0000_1100 => INC(C),
                0b0001_0100 => INC(D),
                0b0001_1100 => INC(E),
                0b0010_0100 => INC(H),
                0b0010_1100 => INC(L),
                0b0011_0100 => INC(AT_HL),
                0b0011_1100 => INC(A),
                0b0000_0101 => DEC(B),
                0b0000_1101 => DEC(C),
                0b0001_0101 => DEC(D),
                0b0001_1101 => DEC(E),
                0b0010_0101 => DEC(H),
                0b0010_1101 => DEC(L),
                0b0011_0101 => DEC(AT_HL),
                0b0011_1101 => DEC(A),
                0b1100_0010 => {
                    let low = bytes.next().expect("TODO handle this gracefully");
                    let high = bytes.next().expect("TODO handle this gracefully");
                    let address = u8s_to_u16(*low, *high);
                    JP_NZ(address)
                }
                first => {
                    // Just grab all of the remaining bytes as DATA.
                    let mut data = vec![*first];
                    data.extend(bytes);
                    DATA(data)
                }
            })
        } else {
            None
        }
    }

    /// Encodes this operation back into machine code bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            NOP => vec![0b0000_0000],
            INC(register) => vec![0b00_000_100 + 0b00_001_000 * register.index()],
            DEC(register) => vec![0b00_000_101 + 0b00_001_000 * register.index()],
            JP_NZ(address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0b1100_0010, low, high]
            }
        }
    }

    /// Encodes this instruction as a pseudo-assembly string.
    pub fn to_asm(&self) -> String {
        match self {
            NOP => format!("NOP"),
            INC(register) => format!("INC {:?}", register),
            DEC(register) => format!("DEC {:?}", register),
            JP_NZ(address) => format!("JP NZ 0x{:04X}", address),
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
