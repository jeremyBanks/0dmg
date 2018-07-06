#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]

use zerodmg_codes;

pub fn main() -> Result<(), impl std::fmt::Debug> {
    zerodmg_codes::main()
}
