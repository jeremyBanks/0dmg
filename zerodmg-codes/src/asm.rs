use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::fmt::Debug;

use self::Operation::*;
use self::U16Register::*;
use self::U8Register::*;

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
    pub fn from_bytes(_bytes: Vec<u8>) -> Self {
        unimplemented!()
    }
}

/// A single CPU operation, including any immediate arguments.
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Operation {
    /// Raw data in the ROM that has not been interpreted as instructions.
    /// This may really be raw data (graphics, text, sound), or it may just be
    /// something we don't understand how to interpret yet.
    DATA(Vec<u8>),

    /// No operation
    NOP,
    /// Increment u8 register
    INC(U8Register),
    /// Decrement u8 register
    DEC(U8Register),
    /// Conditional absolute jump, if Z flag bit is non-zero
    JP_NZ(u16),
}

impl Operation {
    /// Decodes machine code bytes from the iterator to an Operation.
    ///
    /// Returns an error if the iterator is already exhausted, or if it contains
    /// an incomplete instruction.
    pub fn from_byte_iter(_bytes: &Iterator<Item = u8>) -> Result<Self, Box<dyn Debug>> {
        unimplemented!()
    }

    /// Encodes this operation back into machine code bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            DATA(data) => data.clone(),
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
            DATA(data) => format!(
                "DATA 0x{}",
                data.iter()
                    .map(|ref b| format!("{:02X}", b))
                    .collect::<Vec<String>>()
                    .join("")
            ),
            NOP => format!("NOP"),
            INC(register) => format!("INC {:?}", register),
            DEC(register) => format!("DEC {:?}", register),
            JP_NZ(address) => format!("JP NZ 0x{:04X}", address),
        }
    }
}

/// The 8-bit registers that are available in the CPU.
///
/// The discriminant integer associated with each variant is the bit pattern
// used to identify the register the machine code.
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum U8Register {
    /// Primary accumulator register
    A = 0b111,

    /// May be paired with C as 16-bit BC register.
    B = 0b000,
    /// May be paired with B as 16-bit BC register.
    C = 0b001,

    /// May be paired with D as 16-bit BC register.
    D = 0b010,
    /// May be paired with E as 16-bit BC register.
    E = 0b011,

    /// High byte of 16-bit HL memory pointer register.
    H = 0b100,
    /// Low byte of 16-bit HL memory pointer register.
    L = 0b101,

    /// Value in memory address represented indicated by H and L registers.
    AT_HL = 0b110,
}

impl U8Register {
    /// The integer/bit pattern representing this register in the machine code.
    pub fn index(self) -> u8 {
        self as u8
    }
}

/// The 16-bit registers that are available in the CPU.
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum U16Register {
    /// Combines the accumulator register and the interal flag register.
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
