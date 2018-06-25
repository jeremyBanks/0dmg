use super::{CPUController, OneByteRegister};

pub type Operation = fn(opcode: u8, gb: &mut super::GameBoy) -> Execution;

#[derive(Debug, Clone)]
pub struct Execution {
    pub cycles: u64,           // number of cycles elapsed
    pub asm: Option<String>,   // generated pseudo-asm
    pub trace: Option<String>, // human-readable trace data
}

#[macro_export]
// Macro for the output of an operation, allowing us to strip the trace info at compile time.
macro_rules! op_execution {
    {cycles: $cycles:expr; asm: $($asm:expr),*; trace: $($trace:expr),*;} => (
        if cfg!(debug_assertions) {
            Execution {
                cycles: $cycles,
                asm: Some(format!($($asm),*)),
                trace: Some(format!($($trace),*)),
            }
        } else {
            Execution {
                cycles: $cycles,
                asm: None,
                trace: None,
            }
        }
    );
    {cycles: $cycles:expr; asm: $($asm:expr),*;} => (
        if cfg!(debug_assertions) {
            Execution {
                cycles: $cycles,
                asm: Some(format!($($asm),*)),
                trace: None,
            }
        } else {
            Execution {
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

use super::CPUData;
pub const UNIMPLEMENTED: Operation = |opcode, gb| {
    gb.print_recent_executions(32);
    unimplemented!(
        "operation ${} at ${:04x} is not implemented",
        gb.cpu
            .current_operation_code
            .clone()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>(),
        gb.cpu.current_operation_address
    )
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
        trace: "A₀ = ${:02x}, {} = ${:02x}, A₁ = ${:02x}", a0, source, source_value, a1;
    }
};

pub const AND: Operation = |opcode, gb| {
    let source = OneByteRegister::from(opcode & 0b111);
    let (source_value, extra_read_cycles) = gb.register(source);
    let a0 = gb.cpu.a;
    let a1 = a0 & source_value;
    gb.cpu.a = a1;
    gb.set_z_flag(a1 == 0);
    gb.set_n_flag(true);
    gb.set_h_flag(true);
    gb.set_c_flag(false);
    op_execution!{
        cycles: 1 + extra_read_cycles;
        asm: "AND {}", source;
        trace: "A₀ = ${:02x}, {} = ${:02x}, A₁ = ${:02x}", a0, source, source_value, a1;
    }
};

pub const ADD: Operation = |opcode, gb| {
    let source = OneByteRegister::from(opcode & 0b111);
    let (source_value, extra_read_cycles) = gb.register(source);
    let a0 = gb.cpu.a;
    let a1 = a0.wrapping_add(source_value);
    gb.cpu.a = a1;
    gb.set_z_flag(a1 == 0);
    gb.set_n_flag(false);
    gb.set_h_flag(super::u8_get_bit(a1, 4));
    gb.set_c_flag(a1 < a0);
    op_execution!{
        cycles: 1 + extra_read_cycles;
        asm: "ADD {}", source;
        trace: "A₀ = ${:02x}, {} = ${:02x}, A₁ = ${:02x}", a0, source, source_value, a1;
    }
};

pub const SUB: Operation = |opcode, gb| {
    let source = OneByteRegister::from(opcode & 0b111);
    let (source_value, extra_read_cycles) = gb.register(source);
    let a0 = gb.cpu.a;
    let a1 = a0.wrapping_sub(source_value);
    gb.cpu.a = a1;
    gb.set_z_flag(a1 == 0);
    gb.set_n_flag(false);
    gb.set_h_flag(super::u8_get_bit(a1, 4));
    gb.set_c_flag(a1 > a0);
    op_execution!{
        cycles: 1 + extra_read_cycles;
        asm: "SUB {}", source;
        trace: "A₀ = ${:02x}, {} = ${:02x}, A₁ = ${:02x}", a0, source, source_value, a1;
    }
};

pub const CP: Operation = |opcode, gb| {
    let source = OneByteRegister::from(opcode & 0b111);
    let (source_value, extra_read_cycles) = gb.register(source);
    let a = gb.cpu.a;
    let delta = a.wrapping_sub(source_value);
    gb.set_z_flag(delta == 0);
    gb.set_n_flag(false);
    gb.set_h_flag(super::u8_get_bit(delta, 4));
    gb.set_c_flag(delta > a);
    op_execution!{
        cycles: 1 + extra_read_cycles;
        asm: "SUB {}", source;
        trace: "A = ${:02x}, {} = ${:02x}", a, source, source_value;
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
        trace: "A₀ = ${:02x}, {} = ${:02x}, A₁ = ${:02x}", a0, source, source_value, a1;
    }
};

pub const INC: Operation = |opcode, gb| {
    let target = OneByteRegister::from((opcode >> 3) & 0b111);
    let (old_value, extra_read_cycles) = gb.register(target);
    let new_value = old_value.wrapping_add(1);
    let extra_write_cycles = gb.set_register(target, new_value);
    gb.set_z_flag(new_value == 0);
    gb.set_n_flag(false);
    gb.set_h_flag(super::u8_get_bit(new_value, 4));
    op_execution!{
        cycles: 1 + extra_read_cycles + extra_write_cycles;
        asm: "INC {}", target;
        trace: "{}₀ = ${:02x}, {}₁ = ${:02x}", target, old_value, target, new_value;
    }
};

pub const DEC: Operation = |opcode, gb| {
    let target = OneByteRegister::from((opcode >> 3) & 0b111);
    let (old_value, extra_read_cycles) = gb.register(target);
    let new_value = old_value.wrapping_sub(1);
    let extra_write_cycles = gb.set_register(target, new_value);
    gb.set_z_flag(new_value == 0);
    gb.set_n_flag(true);
    gb.set_h_flag(super::u8_get_bit(new_value, 4));
    op_execution!{
        cycles: 1 + extra_read_cycles + extra_write_cycles;
        asm: "DEC {}", target;
        trace: "{}₀ = ${:02x}, {}₁ = ${:02x}", target, old_value, target, new_value;
    }
};

pub const RST: Operation = |opcode, gb| {
    let high_byte = opcode & 0b00_111_000;
    let h = gb.cpu.h;
    let pc0 = gb.cpu.pc;
    let pc1 = super::u8s_to_u16(high_byte, h);
    gb.stack_push(pc0);
    gb.cpu.pc = pc1;
    op_execution!{
        cycles: 8;
        asm: "RST ${:02x}H", high_byte;
        trace: "H = ${:02x}", h;
    }
};
