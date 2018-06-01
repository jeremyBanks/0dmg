use emulator::cpu::CPU;
use emulator::memory::MemoryController;

pub type OpFn = fn(opcode: u8, cpu: &mut CPU, mem: &mut MemoryController) -> Execution;

#[derive(Debug)]
pub struct Execution {
    pub cycles: u64,           // number of cycles elapsed
    pub asm: Option<String>,   // generated pseudo-asm
    pub debug: Option<String>, // human-readable debug data
}

// Macro for the output of an operation, allowing us to strip the debug info at compile time.
macro_rules! op_execution {
    {cycles: $cycles:expr; asm: $($asm:expr),*; debug: $($debug:expr),*;} => (
        if cfg!(debug_assertions) {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: Some(format!($($asm),*)),
                debug: Some(format!($($debug),*)),
            }
        } else {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: None,
                debug: None,
            }
        }
    );
    {cycles: $cycles:expr; asm: $($asm:expr),*;} => (
        if cfg!(debug_assertions) {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: Some(format!($($asm),*)),
                debug: None,
            }
        } else {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: None,
                debug: None,
            }
        }
    );
    {cycles: $cycles:expr; debug: $($debug:expr),*;} => (
    if cfg!(debug_assertions) {
        ::emulator::cpu::operation::Execution {
            cycles: $cycles,
            asm: None,
            debug: Some(format!($($debug),*)),
        }
        } else {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: None,
                debug: None,
            }
        }
    );
    {cycles: $cycles:expr;} => {
        ::emulator::cpu::operation::Execution {
            cycles: $cycles,
            asm: None,
            debug: None,
        }
    };
}
