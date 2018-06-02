#[macro_use]
mod operation;
mod opcodes_cb;
mod opcodes_main;

use super::audio::AudioController;
use super::memory::MemoryController;
use super::video::VideoController;

#[derive(Debug)]
pub struct CPU {
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

impl CPU {
    pub fn new() -> Self {
        CPU {
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

    pub fn tick(
        &mut self,
        mem: &mut MemoryController,
        _vid: &mut VideoController,
        _aud: &mut AudioController,
    ) -> operation::Execution {
        let opcode = self.read_instruction(mem);
        let opcode_final;
        let op;

        if opcode != 0xCB {
            op = opcodes_main::OPCODES[opcode as usize];
            opcode_final = opcode;
        } else {
            let opcode_2 = self.read_immediate_u8(mem);
            op = opcodes_cb::OPCODES[opcode_2 as usize];
            opcode_final = opcode_2;
        };

        let execution = op(opcode_final, self, mem);
        let cycles = execution.cycles;
        let asm = execution.asm.clone();
        let debug = execution.trace.clone();
        self.log_execution(asm, debug);
        self.t += cycles as u64;
        execution
    }

    fn log_execution(&self, asm: Option<String>, info: Option<String>) {
        print!("{:32}", if let Some(s) = asm { s } else { format!("") });
        print!(" ; ${:04x}", self.debug_current_op_addr);
        let code = self
            .debug_current_code
            .clone()
            .into_iter()
            .map(|c| format!("{:02x}", c))
            .collect::<Vec<String>>()
            .join("");
        print!(" ; {:6}", self.t);
        print!(" ; ${:8}", code);
        if let Some(s) = info {
            print!(" ; {}", s);
        }
        println!();
    }

    fn read_instruction(&mut self, mem: &mut MemoryController) -> u8 {
        self.debug_current_code.clear();
        self.debug_current_op_addr = self.i;
        self.read_immediate_u8(mem)
    }

    fn read_immediate_u8(&mut self, mem: &mut MemoryController) -> u8 {
        let value = mem.get(self.i);
        self.debug_current_code.push(value);
        self.i += 1;
        value
    }

    fn read_immediate_i8(&mut self, mem: &mut MemoryController) -> i8 {
        self.read_immediate_u8(mem) as i8
    }

    fn read_immediate_u16(&mut self, mem: &mut MemoryController) -> u16 {
        let n1 = self.read_immediate_u8(mem);
        let n2 = self.read_immediate_u8(mem);
        u8s_to_u16(n1, n2)
    }

    fn absolute_jump(&mut self, nn: u16) {
        self.i = nn;
    }

    fn relative_jump(&mut self, n: i8) {
        self.i = ((self.i as i32) + (n as i32)) as u16;
    }

    fn stack_push(&mut self, mem: &mut MemoryController, value: u16) {
        let sp0 = self.sp;
        let (value_low, value_high) = u16_to_u8s(value);
        mem.set(sp0 - 0, value_low);
        mem.set(sp0 - 1, value_high);
        let sp1 = sp0 - 2;
        self.sp = sp1;
    }

    fn stack_pop(&mut self, mem: &mut MemoryController) -> u16 {
        let sp0 = self.sp;
        let value_low = mem.get(sp0 + 0);
        let value_high = mem.get(sp0 + 1);
        let value = u8s_to_u16(value_low, value_high);
        let sp1 = sp0 + 2;
        self.sp = sp1;
        value
    }

    fn bc(&self) -> u16 {
        return u8s_to_u16(self.c, self.b);
    }

    fn set_bc(&mut self, value: u16) {
        let (c, b) = u16_to_u8s(value);
        self.b = b;
        self.c = c;
    }

    fn hl(&self) -> u16 {
        return u8s_to_u16(self.l, self.h);
    }

    fn set_hl(&mut self, value: u16) {
        let (l, h) = u16_to_u8s(value);
        self.h = h;
        self.l = l;
    }

    fn af(&self) -> u16 {
        return u8s_to_u16(self.f, self.a);
    }

    fn set_af(&mut self, value: u16) {
        let (f, a) = u16_to_u8s(value);
        self.a = a;
        self.f = f;
    }

    fn de(&self) -> u16 {
        return u8s_to_u16(self.e, self.d);
    }

    fn set_de(&mut self, value: u16) {
        let (e, d) = u16_to_u8s(value);
        self.d = d;
        self.e = e;
    }

    fn c_flag(&self) -> bool {
        (self.f & 0x10) == 0x10
    }

    fn set_c_flag(&mut self, value: bool) {
        if value {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }
    }

    fn h_flag(&self) -> bool {
        (self.f & 0x20) == 0x20
    }

    fn set_h_flag(&mut self, value: bool) {
        if value {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
    }

    fn n_flag(&self) -> bool {
        (self.f & 0x40) == 0x40
    }

    fn set_n_flag(&mut self, value: bool) {
        if value {
            self.f |= 0x40;
        } else {
            self.f &= !0x40;
        }
    }

    fn z_flag(&self) -> bool {
        (self.f & 0x80) == 0x80
    }

    fn set_z_flag(&mut self, value: bool) {
        if value {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }
    }
}

fn u8s_to_u16(a: u8, b: u8) -> u16 {
    return a as u16 + ((b as u16) << 8);
}

fn u16_to_u8s(x: u16) -> (u8, u8) {
    (x as u8, (x >> 8) as u8)
}
