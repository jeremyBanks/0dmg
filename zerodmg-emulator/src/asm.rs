#![warn(missing_docs, missing_debug_implementations)]
#![allow(dead_code)]

use zerodmg_utils::little_endian::{u16_to_u8s, u8s_to_u16};

use std::fmt;

use self::Operation::*;
use self::OneByteRegister::*;

// TODO:
// - ROM::from_bytes and Operation::from_byte_iter
// - 

pub fn main() -> Result<(), String> {
    let instructions = vec![
        INC(A),
        JP_NZ(0x0009),
        Data(vec![0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]),
        INC(A),
        INC(A),
        INC(A),
        NOP,
        DEC(A),
        JP_NZ(0x000E),
    ];

    println!("{:?}", instructions.to_bytes());
    println!("{:?}", instructions.to_asm());
    println!("{:?}", instructions);

    // TODO:
    // declare code as vec
    // compile to bytes
    // parse back to data structure
    // print control flow graph?

    Ok(())
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
enum OneByteRegister {
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    AT_HL = 0b110,
    A = 0b111,
}

impl OneByteRegister {
    pub fn index(&self) -> u8 {
        self.clone() as u8
    }
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
enum Operation {
    Data(Vec<u8>),
    NOP,
    INC(OneByteRegister),
    DEC(OneByteRegister),
    JP_NZ(u16),
}

trait ROM {
    fn to_asm(&self) -> String;
    fn from_asm(asm: &String) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: Vec<u8>) -> Self;
}

impl ROM for Vec<Operation> {
    fn to_asm(&self) -> String {
        format!("{:?}", self)
    }
    fn to_bytes(&self) -> Vec<u8> {
        self.iter().map(|ref x| x.to_bytes()).flatten().collect()
    }
    fn from_asm(asm: &String) -> Self {
        unimplemented!()
    }
    fn from_bytes(bytes: Vec<u8>) -> Self {
        unimplemented!()
    }
}

impl Operation {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Data(data) => data.clone(),
            NOP => vec![0b0000_0000],
            INC(register) => vec![0b00_000_100 + 0b00_001_000 * register.index()],
            DEC(register) => vec![0b00_000_101 + 0b00_001_000 * register.index()],
            JP_NZ(address) => {
                let (low, high) = u16_to_u8s(*address);
                vec![0b1100_0010, low, high]
            },
        }
    }

    
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data(data) => write!(f, "DATA {:?}", data),

            NOP => write!(f, "NOP"),
            INC(register) => write!(f, "INC {:?}", register),
            DEC(register) => write!(f, "DEC {:?}", register),
            JP_NZ(address) => write!(f, "JP NZ {:02x}", address),
        }
    }
}