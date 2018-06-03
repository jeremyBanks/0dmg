#[macro_use]
mod operation;
mod opcodes_cb;
mod opcodes_main;

use super::memory::MemoryController;
use super::GameBoy;

pub struct CPUData {
    // clock ticks
    t: u64,
    // instruction pointer
    i: u16,
    // A accumulator register
    a: u8,
    // F flags register
    f: u8,
    // BC register/B and C registers
    b: u8,
    c: u8,
    // DE register/D and E registers
    d: u8,
    e: u8,
    // HL register/H and L registers
    h: u8,
    l: u8,
    // SP stack pointer register
    sp: u16,
    // PC program counter register
    pc: u16,
    // state only used for logging/debugging
    debug_current_code: Vec<u8>,
    debug_current_op_addr: u16,
}

pub trait CPUController {
    fn tick(&mut self) -> operation::Execution;
    fn log_execution(&self, asm: Option<String>, info: Option<String>);
    fn read_instruction(&mut self) -> u8;
    fn read_immediate_u8(&mut self) -> u8;
    fn read_immediate_i8(&mut self) -> i8;
    fn read_immediate_u16(&mut self) -> u16;
    fn absolute_jump(&mut self, nn: u16);
    fn relative_jump(&mut self, n: i8);
    fn stack_push(&mut self, value: u16);
    fn stack_pop(&mut self) -> u16;
    fn bc(&self) -> u16;
    fn set_bc(&mut self, value: u16);
    fn hl(&self) -> u16;
    fn set_hl(&mut self, value: u16);
    fn af(&self) -> u16;
    fn set_af(&mut self, value: u16);
    fn de(&self) -> u16;
    fn set_de(&mut self, value: u16);
    fn c_flag(&self) -> bool;
    fn set_c_flag(&mut self, value: bool);
    fn h_flag(&self) -> bool;
    fn set_h_flag(&mut self, value: bool);
    fn n_flag(&self) -> bool;
    fn set_n_flag(&mut self, value: bool);
    fn z_flag(&self) -> bool;
    fn set_z_flag(&mut self, value: bool);
    fn register(&self, code: u8) -> (&'static str, u8, u64);
    fn set_register(&mut self, code: u8, value: u8) -> (&'static str, u64);
}

impl CPUData {
    pub fn new() -> Self {
        Self {
            t: 0x00,
            i: 0x00,
            a: 0x00,
            f: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            sp: 0x0000,
            pc: 0x0000,
            debug_current_code: Vec::new(),
            debug_current_op_addr: 0x0000,
        }
    }
}

impl CPUController for GameBoy {
    fn tick(&mut self) -> operation::Execution {
        let opcode = self.read_instruction();
        let opcode_final;
        let op;

        if opcode != 0xCB {
            op = opcodes_main::OPCODES[opcode as usize];
            opcode_final = opcode;
        } else {
            let opcode_2 = self.read_immediate_u8();
            op = opcodes_cb::OPCODES[opcode_2 as usize];
            opcode_final = opcode_2;
        };

        let execution = op(opcode_final, self);
        let cycles = execution.cycles;
        let asm = execution.asm.clone();
        let debug = execution.trace.clone();
        self.log_execution(asm, debug);
        self.cpu.t += cycles as u64;
        execution
    }

    fn log_execution(&self, asm: Option<String>, info: Option<String>) {
        print!("{:32}", if let Some(s) = asm { s } else { format!("") });
        print!(" ; ${:04x}", self.cpu.debug_current_op_addr);
        let code = self
            .cpu
            .debug_current_code
            .clone()
            .into_iter()
            .map(|c| format!("{:02x}", c))
            .collect::<Vec<String>>()
            .join("");
        print!(" ; {:6}", self.cpu.t);
        print!(" ; ${:8}", code);
        if let Some(s) = info {
            print!(" ; {}", s);
        }
        println!();
    }

    fn read_instruction(&mut self) -> u8 {
        self.cpu.debug_current_code.clear();
        self.cpu.debug_current_op_addr = self.cpu.i;
        self.read_immediate_u8()
    }

    fn read_immediate_u8(&mut self) -> u8 {
        let i = self.cpu.i;
        let value = self.mem(i);
        self.cpu.debug_current_code.push(value);
        self.cpu.i += 1;
        value
    }

    fn read_immediate_i8(&mut self) -> i8 {
        self.read_immediate_u8() as i8
    }

    fn read_immediate_u16(&mut self) -> u16 {
        let n1 = self.read_immediate_u8();
        let n2 = self.read_immediate_u8();
        u8s_to_u16(n1, n2)
    }

    fn absolute_jump(&mut self, nn: u16) {
        self.cpu.i = nn;
    }

    fn relative_jump(&mut self, n: i8) {
        self.cpu.i = ((self.cpu.i as i32) + (n as i32)) as u16;
    }

    fn stack_push(&mut self, value: u16) {
        let sp0 = self.cpu.sp;
        let sp1 = sp0 - 2;
        let (value_low, value_high) = u16_to_u8s(value);
        self.set_mem(sp1 + 1, value_low);
        self.set_mem(sp1 + 0, value_high);
        self.cpu.sp = sp1;
    }

    fn stack_pop(&mut self) -> u16 {
        let sp0 = self.cpu.sp;
        let sp1 = sp0 + 2;
        let value_low = self.mem(sp0 + 1);
        let value_high = self.mem(sp0 + 0);
        let value = u8s_to_u16(value_low, value_high);
        self.cpu.sp = sp1;
        value
    }

    fn bc(&self) -> u16 {
        return u8s_to_u16(self.cpu.c, self.cpu.b);
    }

    fn set_bc(&mut self, value: u16) {
        let (c, b) = u16_to_u8s(value);
        self.cpu.b = b;
        self.cpu.c = c;
    }

    fn hl(&self) -> u16 {
        return u8s_to_u16(self.cpu.l, self.cpu.h);
    }

    fn set_hl(&mut self, value: u16) {
        let (l, h) = u16_to_u8s(value);
        self.cpu.h = h;
        self.cpu.l = l;
    }

    fn af(&self) -> u16 {
        return u8s_to_u16(self.cpu.f, self.cpu.a);
    }

    fn set_af(&mut self, value: u16) {
        let (f, a) = u16_to_u8s(value);
        self.cpu.a = a;
        self.cpu.f = f;
    }

    fn de(&self) -> u16 {
        return u8s_to_u16(self.cpu.e, self.cpu.d);
    }

    fn set_de(&mut self, value: u16) {
        let (e, d) = u16_to_u8s(value);
        self.cpu.d = d;
        self.cpu.e = e;
    }

    fn c_flag(&self) -> bool {
        (self.cpu.f & 0x10) == 0x10
    }

    fn set_c_flag(&mut self, value: bool) {
        if value {
            self.cpu.f |= 0x10;
        } else {
            self.cpu.f &= !0x10;
        }
    }

    fn h_flag(&self) -> bool {
        (self.cpu.f & 0x20) == 0x20
    }

    fn set_h_flag(&mut self, value: bool) {
        if value {
            self.cpu.f |= 0x20;
        } else {
            self.cpu.f &= !0x20;
        }
    }

    fn n_flag(&self) -> bool {
        (self.cpu.f & 0x40) == 0x40
    }

    fn set_n_flag(&mut self, value: bool) {
        if value {
            self.cpu.f |= 0x40;
        } else {
            self.cpu.f &= !0x40;
        }
    }

    fn z_flag(&self) -> bool {
        (self.cpu.f & 0x80) == 0x80
    }

    fn set_z_flag(&mut self, value: bool) {
        if value {
            self.cpu.f |= 0x80;
        } else {
            self.cpu.f &= !0x80;
        }
    }

    fn register(&self, code: u8) -> (&'static str, u8, u64) {
        match code {
            0b000 => ("B", self.cpu.b, 0),
            0b001 => ("C", self.cpu.c, 0),
            0b010 => ("D", self.cpu.d, 0),
            0b011 => ("E", self.cpu.e, 0),
            0b100 => ("H", self.cpu.h, 0),
            0b101 => ("L", self.cpu.l, 0),
            0b110 => {
                let hl = self.hl();
                ("(HL)", self.mem(hl), 1)
            }
            0b111 => ("A", self.cpu.a, 0),
            _ => panic!("invalid register code {}", code),
        }
    }

    fn set_register(&mut self, code: u8, value: u8) -> (&'static str, u64) {
        match code {
            0b000 => {
                self.cpu.b = value;
                ("B", 0)
            }
            0b001 => {
                self.cpu.c = value;
                ("C", 0)
            }
            0b010 => {
                self.cpu.d = value;
                ("D", 0)
            }
            0b011 => {
                self.cpu.e = value;
                ("E", 0)
            }
            0b100 => {
                self.cpu.h = value;
                ("H", 0)
            }
            0b101 => {
                self.cpu.l = value;
                ("L", 0)
            }
            0b110 => {
                let hl = self.hl();
                self.set_mem(hl, value);
                ("(HL)", 1)
            }
            0b111 => {
                self.cpu.a = value;
                ("A", 0)
            }
            _ => panic!("invalid register code {}", code),
        }
    }
}

fn u8s_to_u16(a: u8, b: u8) -> u16 {
    return a as u16 + ((b as u16) << 8);
}

fn u16_to_u8s(x: u16) -> (u8, u8) {
    (x as u8, (x >> 8) as u8)
}
