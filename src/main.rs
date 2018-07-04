#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]

use zerodmg_emulator::asm;

pub fn main() -> Result<(), impl std::fmt::Debug> {
    asm::main()
}
