use emulator::cpu::CPU;
use emulator::memory::MemoryController;

pub type OpFn = fn(opcode: u8, cpu: &mut CPU, mem: &mut MemoryController) -> Execution;

#[derive(Debug)]
pub struct Execution {
    pub cycles: u64,           // number of cycles elapsed
    pub asm: Option<String>,   // generated pseudo-asm
    pub trace: Option<String>, // human-readable trace data
}

// Macro for the output of an operation, allowing us to strip the trace info at compile time.
macro_rules! op_execution {
    {cycles: $cycles:expr; asm: $($asm:expr),*; trace: $($trace:expr),*;} => (
        if cfg!(debug_assertions) {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: Some(format!($($asm),*)),
                trace: Some(format!($($trace),*)),
            }
        } else {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: None,
                trace: None,
            }
        }
    );
    {cycles: $cycles:expr; asm: $($asm:expr),*;} => (
        if cfg!(trace_assertions) {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: Some(format!($($asm),*)),
                trace: None,
            }
        } else {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: None,
                trace: None,
            }
        }
    );
    {cycles: $cycles:expr; trace: $($trace:expr),*;} => (
    if cfg!(trace_assertions) {
        ::emulator::cpu::operation::Execution {
            cycles: $cycles,
            asm: None,
            trace: Some(format!($($trace),*)),
        }
        } else {
            ::emulator::cpu::operation::Execution {
                cycles: $cycles,
                asm: None,
                trace: None,
            }
        }
    );
    {cycles: $cycles:expr;} => {
        ::emulator::cpu::operation::Execution {
            cycles: $cycles,
            asm: None,
            trace: None,
        }
    };
}
