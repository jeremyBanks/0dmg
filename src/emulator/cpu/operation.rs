use emulator::cpu::{CPUController, OneByteRegister};

pub type Operation = fn(opcode: u8, gb: &mut super::GameBoy) -> Execution;

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
}

pub const INTRA_REGISTER_LOAD: Operation = |opcode, gb| {
    let source = OneByteRegister::from(opcode & 0b111);
    let dest = OneByteRegister::from((opcode >> 3) & 0b111);
    let (source_value, extra_read_cycles) = gb.register(source);
    let (dest_value, _) = gb.register(dest);
    let extra_write_cycles = gb.set_register(dest, source_value);
    op_execution! {
        cycles: 1 + extra_read_cycles + extra_write_cycles;
        asm: "LD {}, {}", dest, source;
        trace: "{} = {}, {}₀ = {}", source, source_value, dest, dest_value;
    }
};

pub const XOR: Operation = |opcode, gb| {
    let source = OneByteRegister::from(opcode & 0b111);
    let (source_value, extra_read_cycles) = gb.register(source);
    let a0 = gb.cpu.a;
    let a1 = a0 ^ source_value;
    gb.cpu.a = a1;
    gb.set_z_flag(a1 == 0);
    gb.set_n_flag(false);
    gb.set_h_flag(false);
    gb.set_c_flag(false);
    op_execution!{
        cycles: 1 + extra_read_cycles;
        asm: "XOR {}", source;
        trace: "A₀ = ${:02x}, {} = ${:02x} A₁ = ${:02x}", a0, source, source_value, a1;
    }
};

pub const AND: Operation = |opcode, gb| {
    let source = OneByteRegister::from(opcode & 0b111);
    let (source_value, extra_read_cycles) = gb.register(source);
    let a0 = gb.cpu.a;
    let a1 = a0 & source_value;
    gb.cpu.a = a1;
    gb.set_z_flag(a1 == 0);
    gb.set_n_flag(false);
    gb.set_h_flag(true);
    gb.set_c_flag(false);
    op_execution!{
        cycles: 1 + extra_read_cycles;
        asm: "AND {}", source;
        trace: "A₀ = ${:02x}, {} = ${:02x} A₁ = ${:02x}", a0, source, source_value, a1;
    }
};

pub const OR: Operation = |opcode, gb| {
    let source = OneByteRegister::from(opcode & 0b111);
    let (source_value, extra_read_cycles) = gb.register(source);
    let a0 = gb.cpu.a;
    let a1 = a0 | source_value;
    gb.cpu.a = a1;
    gb.set_z_flag(a1 == 0);
    gb.set_n_flag(false);
    gb.set_h_flag(false);
    gb.set_c_flag(false);
    op_execution!{
        cycles: 1 + extra_read_cycles;
        asm: "OR {}", source;
        trace: "A₀ = ${:02x}, {} = ${:02x} A₁ = ${:02x}", a0, source, source_value, a1;
    }
};

pub const INC: Operation = |opcode, gb| {
    let target = OneByteRegister::from((opcode >> 3) & 0b111);
    let (old_value, extra_read_cycles) = gb.register(target);
    let new_value = old_value.wrapping_add(1);
    let extra_write_cycles = gb.set_register(target, new_value);
    gb.set_z_flag(old_value == 0);
    gb.set_n_flag(false);
    gb.set_h_flag(u8_get_bit(new_value, 4));
    op_execution!{
        cycles: 1 + extra_read_cycles + extra_write_cycles;
        asm: "INC {}", target;
        trace: "{}₀ = ${:02x}, {}₁ = ${:02x}", target, old_value, target, new_value;
    }
};

pub fn u8_get_bit(x: u8, offset: u8) -> bool {
    if offset > 7 {
        panic!();
    }

    (x >> offset) & 1 == 1
}

pub fn u8_set_bit(x: &mut u8, offset: u8, value: bool) {
    if offset > 7 {
        panic!();
    }

    let mask = 1 << offset;
    if value {
        *x |= mask;
    } else {
        *x &= !mask;
    }
}
