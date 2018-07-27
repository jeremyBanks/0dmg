use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::fmt;
use std::fmt::Display;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use super::Instruction;
    pub use super::Instruction::*;
    pub use super::LD;
    pub use super::U16Register::*;
    pub use super::U8Register::*;
    pub use super::U8SecondaryRegister::*;
    pub use super::RSTTarget::*;
    pub use super::FlagCondition;
    pub use super::FlagCondition::*;
    pub use super::InvalidOpcode;
    pub use super::InvalidOpcode::*;
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
    /// Load immediate bytes into a 16-bit register.
    LD_16_IMMEDIATE(U16Register, u16),
    /// Loads immediates bytes into an 8-bit register.
    LD_8_IMMEDIATE(U8Register, u8),
    /// Loads the value from one 8-bit register into another.
    LD_8_INTERNAL(U8Register, U8Register),
    /// Loads the value from A into a secondary pointer/register.
    LD_8_TO_SECONDARY(U8SecondaryRegister),
    /// Loads the value from secondary pointer/register into A.
    LD_8_FROM_SECONDARY(U8SecondaryRegister),
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
    /// Unconditional absolute jump, updates PC.
    JP(u16),
    /// Uncondition relative jump, updates PC.
    JR(i8),
    /// Conditional absolute jump, may update PC.
    JP_IF(FlagCondition, u16),
    /// Conditional relative jump, may update PC.
    JR_IF(FlagCondition, i8),
    /// Unconditional call. Pushes PC on the stack, then updates it.
    CALL(i16),
    /// Conditional call. Pushes PC on the stack, then updates it.
    CALL_IF(i16, FlagCondition),
    /// Pops PC from the stack (return from call).
    RET,
    /// Pops PC from the stack and reenables interrupts (return from
    /// interrupt).
    RETI,
    /// Stops running the CPU until an interrupt occurs.
    HALT,
    /// Jumps to a hard-coded single-byte address.
    RST(RSTTarget),
    /// Halt and Catch Fire - invalid opcode used to panic in unexpected
    /// situations.
    HCF(InvalidOpcode),
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
#[allow(non_camel_case_types)]
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
    fn address(self) -> u8 {
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
}

/// Invalid instruction opcodes.
/// These should never be executed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
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

                0x01 => LD(BC, d16(bytes)),
                0x11 => LD(DE, d16(bytes)),
                0x21 => LD(HL, d16(bytes)),
                0x31 => LD(SP, d16(bytes)),

                0x02 => LD(AT_BC, A),
                0x12 => LD(AT_DE, A),
                0x22 => LD(AT_HL_Plus, A),
                0x32 => LD(AT_HL_Minus, A),

                0x0A => LD(A, AT_BC),
                0x1A => LD(A, AT_DE),
                0x2A => LD(A, AT_HL_Plus),
                0x3A => LD(A, AT_HL_Minus),

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

                0x06 => LD(B, d8(bytes)),
                0x0E => LD(C, d8(bytes)),
                0x16 => LD(D, d8(bytes)),
                0x1E => LD(E, d8(bytes)),
                0x26 => LD(H, d8(bytes)),
                0x2E => LD(L, d8(bytes)),
                0x36 => LD(AT_HL, d8(bytes)),
                0x3E => LD(A, d8(bytes)),

                0x40 => LD(B, B),
                0x41 => LD(B, C),
                0x42 => LD(B, D),
                0x43 => LD(B, E),
                0x44 => LD(B, H),
                0x45 => LD(B, L),
                0x46 => LD(B, AT_HL),
                0x47 => LD(B, A),
                0x48 => LD(C, B),
                0x49 => LD(C, C),
                0x4A => LD(C, D),
                0x4B => LD(C, E),
                0x4C => LD(C, H),
                0x4D => LD(C, L),
                0x4E => LD(C, AT_HL),
                0x4F => LD(C, A),
                0x50 => LD(D, B),
                0x51 => LD(D, C),
                0x52 => LD(D, D),
                0x53 => LD(D, E),
                0x54 => LD(D, H),
                0x55 => LD(D, L),
                0x56 => LD(D, AT_HL),
                0x57 => LD(D, A),
                0x58 => LD(E, B),
                0x59 => LD(E, C),
                0x5A => LD(E, D),
                0x5B => LD(E, E),
                0x5C => LD(E, H),
                0x5D => LD(E, L),
                0x5E => LD(E, AT_HL),
                0x5F => LD(E, A),
                0x60 => LD(H, B),
                0x61 => LD(H, C),
                0x62 => LD(H, D),
                0x63 => LD(H, E),
                0x64 => LD(H, H),
                0x65 => LD(H, L),
                0x66 => LD(H, AT_HL),
                0x67 => LD(H, A),
                0x68 => LD(L, B),
                0x69 => LD(L, C),
                0x6A => LD(L, D),
                0x6B => LD(L, E),
                0x6C => LD(L, H),
                0x6D => LD(L, L),
                0x6E => LD(L, AT_HL),
                0x6F => LD(L, A),
                0x70 => LD(AT_HL, B),
                0x71 => LD(AT_HL, C),
                0x72 => LD(AT_HL, D),
                0x73 => LD(AT_HL, E),
                0x74 => LD(AT_HL, H),
                0x75 => LD(AT_HL, L),
                0x77 => LD(AT_HL, A),
                0x78 => LD(A, B),
                0x79 => LD(A, C),
                0x7A => LD(A, D),
                0x7B => LD(A, E),
                0x7C => LD(A, H),
                0x7D => LD(A, L),
                0x7E => LD(A, AT_HL),
                0x7F => LD(A, A),

                0x76 => HALT,

                0x80 => ADD(B),
                0x81 => ADD(C),
                0x82 => ADD(D),
                0x83 => ADD(E),
                0x84 => ADD(H),
                0x85 => ADD(L),
                0x86 => ADD(AT_HL),
                0x87 => ADD(A),

                0x88 => ADC(B),
                0x89 => ADC(C),
                0x8A => ADC(D),
                0x8B => ADC(E),
                0x8C => ADC(H),
                0x8D => ADC(L),
                0x8E => ADC(AT_HL),
                0x8F => ADC(A),

                0x90 => SUB(B),
                0x91 => SUB(C),
                0x92 => SUB(D),
                0x93 => SUB(E),
                0x94 => SUB(H),
                0x95 => SUB(L),
                0x96 => SUB(AT_HL),
                0x97 => SUB(A),

                0x98 => SBC(B),
                0x99 => SBC(C),
                0x9A => SBC(D),
                0x9B => SBC(E),
                0x9C => SBC(H),
                0x9D => SBC(L),
                0x9E => SBC(AT_HL),
                0x9F => SBC(A),

                0xA0 => AND(B),
                0xA1 => AND(C),
                0xA2 => AND(D),
                0xA3 => AND(E),
                0xA4 => AND(H),
                0xA5 => AND(L),
                0xA6 => AND(AT_HL),
                0xA7 => AND(A),

                0xA8 => XOR(B),
                0xA9 => XOR(C),
                0xAA => XOR(D),
                0xAB => XOR(E),
                0xAC => XOR(H),
                0xAD => XOR(L),
                0xAE => XOR(AT_HL),
                0xAF => XOR(A),

                0xB0 => OR(B),
                0xB1 => OR(C),
                0xB2 => OR(D),
                0xB3 => OR(E),
                0xB4 => OR(H),
                0xB5 => OR(L),
                0xB6 => OR(AT_HL),
                0xB7 => OR(A),

                0xB8 => CP(B),
                0xB9 => CP(C),
                0xBA => CP(D),
                0xBB => CP(E),
                0xBC => CP(H),
                0xBD => CP(L),
                0xBE => CP(AT_HL),
                0xBF => CP(A),

                0xC2 => JP_IF(if_NZ, d16(bytes)),
                0xC3 => JP(d16(bytes)),
                0xC9 => RET,
                0xD9 => RETI,

                0xD3 => HCF(xxD3),
                0xDB => HCF(xxDB),
                0xDD => HCF(xxDD),
                0xE3 => HCF(xxE3),
                0xE4 => HCF(xxE4),
                0xEB => HCF(xxEB),
                0xEC => HCF(xxEC),
                0xED => HCF(xxED),
                0xF4 => HCF(xxF4),
                0xFC => HCF(xxFC),
                0xFD => HCF(xxFD),

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
            LD_16_IMMEDIATE(_, _) => 3,
            LD_8_IMMEDIATE(_, _) => 2,
            LD_8_INTERNAL(_, _) => 1,
            LD_8_FROM_SECONDARY(_) => 1,
            LD_8_TO_SECONDARY(_) => 1,
            INC(_) => 1,
            DEC(_) => 1,
            ADD(_) => 1,
            SUB(_) => 1,
            ADC(_) => 1,
            SBC(_) => 1,
            AND(_) => 1,
            OR(_) => 1,
            XOR(_) => 1,
            CP(_) => 1,
            JP_IF(_, _) => 3,
            JR_IF(_, _) => 2,
            JP(_) => 3,
            JR(_) => 2,
            HALT => 1,
            HCF(_) => 1,
            RET => 1,
            RETI => 1,
            RST(_) => 1,
        }
    }

    /// Encodes this instruction back into machine code bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = match self {
            NOP => vec![0x00],
            INC(register) => vec![0x04 + 0b00_001_000 * register.index()],
            DEC(register) => vec![0x05 + 0b00_001_000 * register.index()],
            JP_IF(condition, address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0xC2 + condition.index() << 4, low, high]
            }
            JP(address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0xC3, low, high]
            }
            JR(offset) => vec![0x18, *offset as u8],
            LD_16_IMMEDIATE(register, value) => {
                let (low, high) = u16_to_u8s(*value);
                let opcode = 0x01 + 0b00_01_0000 * register.index();
                vec![opcode, low, high]
            }
            LD_8_IMMEDIATE(register, value) => {
                let opcode = 0x06 + 0b00_001_000 * register.index();
                vec![opcode, *value]
            }
            LD_8_INTERNAL(dest, source) => {
                vec![0x40 + 0b00_001_000 * dest.index() + source.index()]
            }
            HALT => vec![0x76],
            RET => vec![0xC9],
            RETI => vec![0xD9],
            HCF(variant) => vec![variant.opcode()],
            RST(target) => vec![0xC7 + target.address()],
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
            JP_IF(condition, address) => write!(f, "JP {:?} 0x{:04X}", condition, address),
            JR(offset) => write!(f, "JR 0x{:02X}", offset),
            JR_IF(condition, address) => write!(f, "JR {:?} 0x{:04X}", condition, address),
            LD_16_IMMEDIATE(register, value) => write!(f, "LD {:?} 0x{:04X}", register, value),
            LD_8_IMMEDIATE(register, value) => write!(f, "LD {:?} 0x{:02X}", register, value),
            LD_8_INTERNAL(dest, source) => write!(f, "LD {:?} {:?}", dest, source),
            LD_8_TO_SECONDARY(dest) => write!(f, "LD {:?} A", dest),
            LD_8_FROM_SECONDARY(source) => write!(f, "LD A {:?}", source),
            HALT => write!(f, "HALT"),
            RET => write!(f, "RET"),
            RETI => write!(f, "RETI"),
            HCF(variant) => write!(f, "HCF {:?} !!!", variant),
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
}

pub fn LD<A : Ld<B>, B>(a: A, b: B) -> Instruction {
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
            _ => panic!("You can only LD a U8SecondaryRegister to A, not {:?}.", destination)
        }
    }
}

impl Ld<U8Register> for U8SecondaryRegister {
    fn LD(destination: U8SecondaryRegister, source: U8Register) -> Instruction {
        match source {
            A => LD_8_TO_SECONDARY(destination), 
            _ => panic!("You can only LD a U8SecondaryRegister from A, not {:?}.", destination)
        }
    }
}
