use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GameBoy {
    // time/ticks since start
    t: u64,
    // instruction pointer/index
    i: u16,
    main_ram: [u8; 8192],
    video_ram: [u8; 8192],
    stack_ram: [u8; 127],
    main_registers: [u8; 12],
    boot_rom: Vec<u8>,
    game_rom: Vec<u8>,
    // the 4-item one-byte 2-bit-greyscale color table at $FF47
    bg_palette: u8,
    debug_current_op_addr: u16,
    debug_current_code: Vec<u8>,

    boot_rom_mapped: bool,

    frame_buffer: Arc<Mutex<Vec<u8>>>,
}

impl GameBoy {
    pub fn new(frame_buffer: Arc<Mutex<Vec<u8>>>) -> GameBoy {
        GameBoy {
            t: 0,
            i: 0,
            main_ram: [0u8; 8192],
            video_ram: [0u8; 8192],
            stack_ram: [0u8; 127],
            main_registers: [0u8; 12],
            boot_rom: load_boot_rom(),
            boot_rom_mapped: true,
            game_rom: load_game_rom("Tetris[:1024]"),
            bg_palette: 0,
            debug_current_op_addr: 0,
            debug_current_code: vec![],
            frame_buffer,
        }
    }

    fn read_instruction(&mut self) -> u8 {
        self.debug_current_code.clear();
        self.debug_current_op_addr = self.i;
        self.read_immediate_u8()
    }

    fn read_immediate_u8(&mut self) -> u8 {
        let value = self.get_memory(self.i);
        self.debug_current_code.push(value);
        self.i += 1;
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

    fn relative_jump(&mut self, n: i8) {
        self.i = ((self.i as i32) + (n as i32)) as u16;
    }

    fn stack_push(&mut self, value: u16) {
        let sp0 = self.sp();
        let (value_low, value_high) = u16_to_u8s(sp0);
        self.set_memory(sp0 - 0, value_low);
        self.set_memory(sp0 - 1, value_high);
        let sp1 = sp0 - 2;
        self.set_sp(sp1);
    }

    fn stack_pop(&mut self) -> u16 {
        let sp0 = self.sp();
        let value_low = self.get_memory(sp0 + 0);
        let value_high = self.get_memory(sp0 + 1);
        let value = u8s_to_u16(value_low, value_high);
        let sp1 = sp0 + 2;
        self.set_sp(sp1);
        value
    }

    fn get_memory(&self, address: u16) -> u8 {
        let value;
        if self.boot_rom_mapped && address <= 0x00FF {
            // boot ROM, until unmapped to expose initial bytes of game ROM
            value = self.boot_rom[address as usize];
        } else if address <= 0x7FFF {
            // first page of game ROM
            value = self.game_rom[address as usize];
        } else if 0x8000 <= address && address <= 0x9FFF {
            let i: usize = (address - 0x8000) as usize;
            value = self.video_ram[i];
        } else if 0xFF80 <= address && address <= 0xFFFE {
            let i: usize = (address - 0xFF80) as usize;
            value = self.stack_ram[i];
        } else {
            panic!("I don't know how to get memory address ${:04x}.", address);
        }

        {
            let mut frame_buffer = self.frame_buffer.lock().unwrap();
            let i = (address as usize) % frame_buffer.len();
            frame_buffer[i] = value;
        }

        value
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        if 0x8000 <= address && address <= 0x9FFF {
            let i: usize = (address - 0x8000) as usize;
            self.video_ram[i] = value;
            println!("  ; video_ram[${:04x}] = ${:02x}", i, value);
        } else if 0xFF80 <= address && address <= 0xFFFE {
            let i: usize = (address - 0xFF80) as usize;
            self.stack_ram[i] = value;
            println!("  ; stack_ram[${:02x}] = ${:02x}", i, value);
        } else if 0xFF10 <= address && address <= 0xFF26 {
            println!("  ; skipping write to sound control memory -- not implemented");
        } else if address == 0xFF47 {
            self.bg_palette = value;
            println!("  ; updated background palette");
        } else {
            panic!("I don't know how to set memory address ${:04x}.", address);
        }

        {
            let mut frame_buffer = self.frame_buffer.lock().unwrap();
            let i = (address as usize) % frame_buffer.len();
            frame_buffer[i] = value;
        }
    }

    fn print_current_code(&self, asm: String, info: String) {
        print!("{:32}", asm);
        print!(" ; ${:04x}", self.debug_current_op_addr);
        let code = self.debug_current_code
            .clone()
            .into_iter()
            .map(|c| format!("{:02x}", c))
            .collect::<Vec<String>>()
            .join("");
        print!(" ; {:6}", self.t);
        print!(" ; ${:8}", code);
        if info.len() > 0 {
            print!(" ; {}", info);
        }
        println!();
    }

    // Main Loop

    pub fn run(&mut self) {
        println!();
        let (operations, operations_cb) = get_operations();
        println!(
            "; {:3} one-byte opcodes implemented (~{:3.0}%).",
            operations.len(),
            (operations.len() as f32 / 2.55)
        );
        println!(
            "; {:3} two-byte opcodes implemented (~{:3.0}%).",
            operations_cb.len(),
            (operations_cb.len() as f32 / 2.55)
        );
        println!();

        println!("; assembly:                        addr:   t/μs:   codes:       flags:");
        println!("; ---------                        -----   -----   ------       ------");

        loop {
            let opcode = self.read_instruction();

            let op = if opcode != 0xCB {
                operations.get(&opcode)
            } else {
                let opcode_2 = self.read_immediate_u8();
                operations_cb.get(&opcode_2)
            };

            match op {
                Some(op) => {
                    let (asm, debug) = (op.execute)(self);
                    self.print_current_code(asm, debug);
                    self.t += op.cycles as u64;
                }
                None => {
                    self.print_current_code(format!("; ERROR: unsupported opcode"), format!(""));
                    panic!("unsupported opcode");
                }
            };
        }
    }

    // Register Access

    fn a(&self) -> u8 {
        return self.main_registers[0];
    }

    fn set_a(&mut self, value: u8) {
        self.main_registers[0] = value;
    }

    fn f(&self) -> u8 {
        return self.main_registers[1];
    }

    fn set_f(&mut self, value: u8) {
        self.main_registers[1] = value;
    }

    fn b(&self) -> u8 {
        return self.main_registers[2];
    }

    fn set_b(&mut self, value: u8) {
        self.main_registers[2] = value;
    }

    fn c(&self) -> u8 {
        return self.main_registers[3];
    }

    fn set_c(&mut self, value: u8) {
        self.main_registers[3] = value;
    }

    fn d(&self) -> u8 {
        return self.main_registers[4];
    }

    fn set_d(&mut self, value: u8) {
        self.main_registers[4] = value;
    }

    fn e(&self) -> u8 {
        return self.main_registers[5];
    }

    fn set_e(&mut self, value: u8) {
        self.main_registers[5] = value;
    }

    fn h(&self) -> u8 {
        return self.main_registers[6];
    }

    fn set_h(&mut self, value: u8) {
        self.main_registers[6] = value;
    }

    fn l(&self) -> u8 {
        return self.main_registers[7];
    }

    fn set_l(&mut self, value: u8) {
        self.main_registers[7] = value;
    }

    fn sp_s(&self) -> u8 {
        return self.main_registers[8];
    }

    fn set_sp_s(&mut self, value: u8) {
        self.main_registers[8] = value;
    }

    fn sp_p(&self) -> u8 {
        return self.main_registers[9];
    }

    fn set_sp_p(&mut self, value: u8) {
        self.main_registers[9] = value;
    }

    fn pc_p(&self) -> u8 {
        return self.main_registers[10];
    }

    fn set_pc_p(&mut self, value: u8) {
        self.main_registers[10] = value;
    }

    fn pc_c(&self) -> u8 {
        return self.main_registers[11];
    }

    fn set_pc_c(&mut self, value: u8) {
        self.main_registers[11] = value;
    }

    fn hl(&self) -> u16 {
        return u8s_to_u16(self.l(), self.h());
    }

    fn set_hl(&mut self, value: u16) {
        let (l, h) = u16_to_u8s(value);
        self.set_h(h);
        self.set_l(l);
    }

    fn af(&self) -> u16 {
        return u8s_to_u16(self.f(), self.a());
    }

    fn set_af(&mut self, value: u16) {
        let (f, a) = u16_to_u8s(value);
        self.set_a(a);
        self.set_f(f);
    }

    fn sp(&self) -> u16 {
        return u8s_to_u16(self.sp_p(), self.sp_s());
    }

    fn set_sp(&mut self, value: u16) {
        let (p, s) = u16_to_u8s(value);
        self.set_sp_s(s);
        self.set_sp_p(p);
    }

    fn pc(&self) -> u16 {
        return u8s_to_u16(self.pc_c(), self.pc_p());
    }

    fn set_pc(&mut self, value: u16) {
        let (c, p) = u16_to_u8s(value);
        self.set_pc_p(p);
        self.set_pc_c(c);
    }

    fn bc(&self) -> u16 {
        return u8s_to_u16(self.c(), self.b());
    }

    fn set_bc(&mut self, value: u16) {
        let (c, b) = u16_to_u8s(value);
        self.set_b(b);
        self.set_c(c);
    }

    fn de(&self) -> u16 {
        return u8s_to_u16(self.e(), self.d());
    }

    fn set_de(&mut self, value: u16) {
        let (e, d) = u16_to_u8s(value);
        self.set_d(d);
        self.set_e(e);
    }

    fn z_flag(&self) -> bool {
        u8_get_bit(self.f(), 1)
    }

    fn set_z_flag(&mut self, value: bool) {
        let mut flags = self.f();
        u8_set_bit(&mut flags, 1, value);
        self.set_f(flags);
    }

    fn n_flag(&self) -> bool {
        u8_get_bit(self.f(), 2)
    }

    fn set_n_flag(&mut self, value: bool) {
        let mut flags = self.f();
        u8_set_bit(&mut flags, 2, value);
        self.set_f(flags);
    }

    fn h_flag(&self) -> bool {
        u8_get_bit(self.f(), 3)
    }

    fn set_h_flag(&mut self, value: bool) {
        let mut flags = self.f();
        u8_set_bit(&mut flags, 3, value);
        self.set_f(flags);
    }

    fn c_flag(&self) -> bool {
        u8_get_bit(self.f(), 4)
    }

    fn set_c_flag(&mut self, value: bool) {
        let mut flags = self.f();
        u8_set_bit(&mut flags, 4, value);
        self.set_f(flags);
    }
}

fn u8s_to_u16(a: u8, b: u8) -> u16 {
    return a as u16 + ((b as u16) << 8);
}

fn u16_to_u8s(x: u16) -> (u8, u8) {
    (x as u8, (x >> 8) as u8)
}

fn u8_get_bit(x: u8, offset: u8) -> bool {
    if offset > 7 {
        panic!();
    }

    (x >> offset) & 1 == 1
}

fn u8_set_bit(x: &mut u8, offset: u8, value: bool) {
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

struct Operation {
    code: u8,
    cycles: u8,
    execute: fn(gb: &mut GameBoy) -> (String, String),
}

fn get_operations() -> (HashMap<u8, Operation>, HashMap<u8, Operation>) {
    let mut operations = HashMap::new();
    let mut operations_cb = HashMap::new();

    {
        let mut op = |code: u8, cycles: u8, execute: fn(gb: &mut GameBoy) -> (String, String)| {
            let operation = Operation {
                code: code,
                cycles: cycles,
                execute: execute,
            };
            match operations.insert(operation.code, operation) {
                Some(_existing_op) => panic!("duplicate opcode"),
                None => {}
            }
        };
        let mut op_cb =
            |code: u8, cycles: u8, execute: fn(gb: &mut GameBoy) -> (String, String)| {
                let operation = Operation {
                    code: code,
                    cycles: cycles,
                    execute: execute,
                };
                match operations_cb.insert(operation.code, operation) {
                    Some(_existing_op) => panic!("duplicate opcode"),
                    None => {}
                }
            };

        // 3.1.1. 8-bit Loads
        {
            // 1. LD nn, n
            // Put value n into nn.
            {
                op(0x06, 2, |gb| {
                    let b0 = gb.b();
                    let b1 = gb.read_immediate_u8();
                    gb.set_b(b1);
                    (
                        format!("LD B, ${:02x}", b1),
                        format!("B₀ = ${:02x}, B₁ = ${:02x}", b0, b1),
                    )
                });
                op(0x0E, 2, |gb| {
                    let c0 = gb.c();
                    let c1 = gb.read_immediate_u8();
                    gb.set_c(c1);
                    (
                        format!("LD C, ${:02x}", c1),
                        format!("C₀ = ${:02x}, C₁ = ${:02x}", c0, c1),
                    )
                });
                op(0x16, 2, |gb| {
                    let d0 = gb.d();
                    let d1 = gb.read_immediate_u8();
                    gb.set_d(d1);
                    (
                        format!("LD D, ${:02x}", d1),
                        format!("D₀ = ${:02x}, D₁ = ${:02x}", d0, d1),
                    )
                });
                op(0x1E, 2, |gb| {
                    let e0 = gb.e();
                    let e1 = gb.read_immediate_u8();
                    gb.set_e(e1);
                    (
                        format!("LD E, ${:02x}", e1),
                        format!("E₀ = ${:02x}, E₁ = ${:02x}", e0, e1),
                    )
                });
                op(0x26, 2, |gb| {
                    let h0 = gb.h();
                    let h1 = gb.read_immediate_u8();
                    gb.set_h(h1);
                    (
                        format!("LD H, ${:02x}", h1),
                        format!("H₀ = ${:02x}, H₁ = ${:02x}", h0, h1),
                    )
                });
                op(0x2E, 2, |gb| {
                    let l0 = gb.l();
                    let l1 = gb.read_immediate_u8();
                    gb.set_l(l1);
                    (
                        format!("LD L, ${:02x}", l1),
                        format!("L₀ = ${:02x}, L₁ = ${:02x}", l0, l1),
                    )
                });
            }

            // 2. LD r1, r2
            // Put value r2 into r1.
            // 3. LD A, n
            // Put value n into A.
            {
                // LD A, *
                op(0x7F, 1, |gb| {
                    let a = gb.a();
                    (format!("LD A, A"), format!("A = ${:02x}", a))
                });
                op(0x78, 1, |gb| {
                    let a0 = gb.a();
                    let b = gb.b();
                    gb.set_a(b);
                    (
                        format!("LD A, B"),
                        format!("A₀ = ${:02x}, B = ${:02x}", a0, b),
                    )
                });
                op(0x79, 1, |gb| {
                    let a0 = gb.a();
                    let c = gb.c();
                    gb.set_a(c);
                    (
                        format!("LD A, C"),
                        format!("A₀ = ${:02x}, C = ${:02x}", a0, c),
                    )
                });
                op(0x7A, 1, |gb| {
                    let a0 = gb.a();
                    let d = gb.d();
                    gb.set_a(d);
                    (
                        format!("LD A, D"),
                        format!("A₀ = ${:02x}, D = ${:02x}", a0, d),
                    )
                });
                op(0x7B, 1, |gb| {
                    let a0 = gb.a();
                    let e = gb.e();
                    gb.set_a(e);
                    (
                        format!("LD A, E"),
                        format!("A₀ = ${:02x}, E = ${:02x}", a0, e),
                    )
                });
                op(0x7C, 1, |gb| {
                    let a0 = gb.a();
                    let h = gb.h();
                    gb.set_a(h);
                    (
                        format!("LD A, H"),
                        format!("A₀ = ${:02x}, H = ${:02x}", a0, h),
                    )
                });
                op(0x7D, 1, |gb| {
                    let a0 = gb.a();
                    let l = gb.l();
                    gb.set_a(l);
                    (
                        format!("LD A, L"),
                        format!("A₀ = ${:02x}, L = ${:02x}", a0, l),
                    )
                });
                op(0x0A, 2, |gb| {
                    let a0 = gb.a();
                    let bc = gb.bc();
                    let a1 = gb.get_memory(bc);
                    gb.set_a(a1);
                    (
                        format!("LD A, (BC)"),
                        format!("A₀ = ${:02x}, BC = ${:04x}, (BC) = ${:02x}", a0, bc, a1),
                    )
                });
                op(0x1A, 2, |gb| {
                    let a0 = gb.a();
                    let de = gb.de();
                    let a1 = gb.get_memory(de);
                    gb.set_a(a1);
                    (
                        format!("LD A, (DE)"),
                        format!("A₀ = ${:02x}, DE = ${:04x}, (DE) = ${:02x}", a0, de, a1),
                    )
                });
                op(0x7E, 2, |gb| {
                    let a0 = gb.a();
                    let hl = gb.hl();
                    let a1 = gb.get_memory(hl);
                    gb.set_a(a1);
                    (
                        format!("LD A, (HL)"),
                        format!("A₀ = ${:02x}, HL = ${:04x}, (HL) = ${:02x}", a0, hl, a1),
                    )
                });
                op(0xFA, 4, |gb| {
                    let nn = gb.read_immediate_u16();
                    let a0 = gb.a();
                    let a1 = gb.get_memory(nn);
                    gb.set_a(a1);
                    (
                        format!("LD A, (${:04x})", nn),
                        format!("A₀ = ${:02x}, A₁ = ${:04x}", a0, a1),
                    )
                });
                op(0x3E, 2, |gb| {
                    let n = gb.read_immediate_u8();
                    let a0 = gb.a();
                    gb.set_a(n);
                    (format!("LD A, ${:02x}", n), format!("A₀ = ${:02x}", a0))
                });
                // LD B, *
                op(0x40, 1, |gb| {
                    let b = gb.b();
                    (format!("LD B, B"), format!("B = ${:02x}", b))
                });
                op(0x41, 1, |gb| {
                    let b0 = gb.b();
                    let c = gb.c();
                    gb.set_b(c);
                    (
                        format!("LD B, C"),
                        format!("B₀ = ${:02x}, C = ${:02x}", b0, c),
                    )
                });
                op(0x42, 1, |gb| {
                    let b0 = gb.b();
                    let d = gb.d();
                    gb.set_b(d);
                    (
                        format!("LD B, D"),
                        format!("B₀ = ${:02x}, D = ${:02x}", b0, d),
                    )
                });
                op(0x43, 1, |gb| {
                    let b0 = gb.b();
                    let e = gb.e();
                    gb.set_b(e);
                    (
                        format!("LD B, E"),
                        format!("B₀ = ${:02x}, E = ${:02x}", b0, e),
                    )
                });
                op(0x44, 1, |gb| {
                    let b0 = gb.b();
                    let h = gb.h();
                    gb.set_b(h);
                    (
                        format!("LD B, H"),
                        format!("B₀ = ${:02x}, H = ${:02x}", b0, h),
                    )
                });
                op(0x45, 1, |gb| {
                    let b0 = gb.b();
                    let l = gb.l();
                    gb.set_b(l);
                    (
                        format!("LD B, L"),
                        format!("B₀ = ${:02x}, L = ${:02x}", b0, l),
                    )
                });
                op(0x46, 2, |gb| {
                    let b0 = gb.b();
                    let hl = gb.hl();
                    let b1 = gb.get_memory(hl);
                    gb.set_b(b1);
                    (
                        format!("LD B, (HL)"),
                        format!("B₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", b0, hl, b1),
                    )
                });
                // LD C, *
                op(0x48, 1, |gb| {
                    let c0 = gb.c();
                    let b = gb.b();
                    gb.set_c(b);
                    (
                        format!("LD C, B"),
                        format!("C₀ = ${:02x}, B = ${:02x}", c0, b),
                    )
                });
                op(0x49, 1, |gb| {
                    let c = gb.c();
                    (format!("LD C, C"), format!("C = ${:02x}", c))
                });
                op(0x4A, 1, |gb| {
                    let c0 = gb.c();
                    let d = gb.d();
                    gb.set_c(d);
                    (
                        format!("LD C, D"),
                        format!("C₀ = ${:02x}, D = ${:02x}", c0, d),
                    )
                });
                op(0x4B, 1, |gb| {
                    let c0 = gb.c();
                    let e = gb.e();
                    gb.set_c(e);
                    (
                        format!("LD C, E"),
                        format!("C₀ = ${:02x}, E = ${:02x}", c0, e),
                    )
                });
                op(0x4C, 1, |gb| {
                    let c0 = gb.c();
                    let h = gb.h();
                    gb.set_c(h);
                    (
                        format!("LD C, H"),
                        format!("C₀ = ${:02x}, H = ${:02x}", c0, h),
                    )
                });
                op(0x4D, 1, |gb| {
                    let c0 = gb.c();
                    let l = gb.l();
                    gb.set_c(l);
                    (
                        format!("LD C, L"),
                        format!("C₀ = ${:02x}, L = ${:02x}", c0, l),
                    )
                });
                op(0x4E, 2, |gb| {
                    let c0 = gb.c();
                    let hl = gb.hl();
                    let c1 = gb.get_memory(hl);
                    gb.set_c(c1);
                    (
                        format!("LD C, (HL)"),
                        format!("C₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", c0, hl, c1),
                    )
                });
                // LD D, *
                op(0x50, 1, |gb| {
                    let d0 = gb.d();
                    let b = gb.b();
                    gb.set_d(b);
                    (
                        format!("LD D, B"),
                        format!("D₀ = ${:02x}, B = ${:02x}", d0, b),
                    )
                });
                op(0x51, 1, |gb| {
                    let d0 = gb.d();
                    let c = gb.c();
                    gb.set_d(c);
                    (
                        format!("LD D, C"),
                        format!("D₀ = ${:02x}, C = ${:02x}", d0, c),
                    )
                });
                op(0x52, 1, |gb| {
                    let d = gb.d();
                    (format!("LD D, D"), format!("D = ${:02x}", d))
                });
                op(0x53, 1, |gb| {
                    let d0 = gb.d();
                    let e = gb.e();
                    gb.set_d(e);
                    (
                        format!("LD D, E"),
                        format!("D₀ = ${:02x}, E = ${:02x}", d0, e),
                    )
                });
                op(0x54, 1, |gb| {
                    let d0 = gb.d();
                    let h = gb.h();
                    gb.set_d(h);
                    (
                        format!("LD D, H"),
                        format!("D₀ = ${:02x}, H = ${:02x}", d0, h),
                    )
                });
                op(0x55, 1, |gb| {
                    let d0 = gb.d();
                    let l = gb.l();
                    gb.set_d(l);
                    (
                        format!("LD D, L"),
                        format!("D₀ = ${:02x}, L = ${:02x}", d0, l),
                    )
                });
                op(0x56, 2, |gb| {
                    let d0 = gb.d();
                    let hl = gb.hl();
                    let d1 = gb.get_memory(hl);
                    gb.set_d(d1);
                    (
                        format!("LD D, (HL)"),
                        format!("D₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", d0, hl, d1),
                    )
                });
                // LD E, *
                op(0x58, 1, |gb| {
                    let e0 = gb.e();
                    let b = gb.b();
                    gb.set_e(b);
                    (
                        format!("LD E, B"),
                        format!("E₀ = ${:02x}, B = ${:02x}", e0, b),
                    )
                });
                op(0x59, 1, |gb| {
                    let e0 = gb.e();
                    let c = gb.c();
                    gb.set_e(c);
                    (
                        format!("LD E, C"),
                        format!("E₀ = ${:02x}, C = ${:02x}", e0, c),
                    )
                });
                op(0x5A, 1, |gb| {
                    let e0 = gb.e();
                    let d = gb.d();
                    gb.set_e(d);
                    (
                        format!("LD E, D"),
                        format!("E₀ = ${:02x}, D = ${:02x}", e0, d),
                    )
                });
                op(0x5B, 1, |gb| {
                    let e = gb.e();
                    (format!("LD E, E"), format!("E = ${:02x}", e))
                });
                op(0x5C, 1, |gb| {
                    let e0 = gb.e();
                    let h = gb.h();
                    gb.set_e(h);
                    (
                        format!("LD E, H"),
                        format!("E₀ = ${:02x}, H = ${:02x}", e0, h),
                    )
                });
                op(0x5D, 1, |gb| {
                    let e0 = gb.e();
                    let l = gb.l();
                    gb.set_e(l);
                    (
                        format!("LD E, L"),
                        format!("E₀ = ${:02x}, L = ${:02x}", e0, l),
                    )
                });
                op(0x5E, 2, |gb| {
                    let e0 = gb.e();
                    let hl = gb.hl();
                    let e1 = gb.get_memory(hl);
                    gb.set_e(e1);
                    (
                        format!("LD E, (HL)"),
                        format!("E₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", e0, hl, e1),
                    )
                });
                // LD H, *
                op(0x60, 1, |gb| {
                    let h0 = gb.h();
                    let b = gb.b();
                    gb.set_h(b);
                    (
                        format!("LD H, B"),
                        format!("H₀ = ${:02x}, B = ${:02x}", h0, b),
                    )
                });
                op(0x61, 1, |gb| {
                    let h0 = gb.h();
                    let c = gb.c();
                    gb.set_h(c);
                    (
                        format!("LD H, C"),
                        format!("H₀ = ${:02x}, C = ${:02x}", h0, c),
                    )
                });
                op(0x62, 1, |gb| {
                    let h0 = gb.h();
                    let d = gb.d();
                    gb.set_h(d);
                    (
                        format!("LD H, D"),
                        format!("H₀ = ${:02x}, D = ${:02x}", h0, d),
                    )
                });
                op(0x63, 1, |gb| {
                    let h0 = gb.h();
                    let e = gb.e();
                    gb.set_h(e);
                    (
                        format!("LD H, E"),
                        format!("H₀ = ${:02x}, E = ${:02x}", h0, e),
                    )
                });
                op(0x64, 1, |gb| {
                    let h = gb.h();
                    (format!("LD H, H"), format!("H = ${:02x}", h))
                });
                op(0x65, 1, |gb| {
                    let h0 = gb.h();
                    let l = gb.l();
                    gb.set_h(l);
                    (
                        format!("LD H, L"),
                        format!("H₀ = ${:02x}, L = ${:02x}", h0, l),
                    )
                });
                op(0x66, 2, |gb| {
                    let h0 = gb.h();
                    let hl = gb.hl();
                    let h1 = gb.get_memory(hl);
                    gb.set_h(h1);
                    (
                        format!("LD H, (HL)"),
                        format!("H₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", h0, hl, h1),
                    )
                });
                // LD L, *
                op(0x68, 1, |gb| {
                    let l0 = gb.l();
                    let b = gb.b();
                    gb.set_l(b);
                    (
                        format!("LD L, B"),
                        format!("L₀ = ${:02x}, B = ${:02x}", l0, b),
                    )
                });
                op(0x69, 1, |gb| {
                    let l0 = gb.l();
                    let c = gb.c();
                    gb.set_l(c);
                    (
                        format!("LD L, C"),
                        format!("L₀ = ${:02x}, C = ${:02x}", l0, c),
                    )
                });
                op(0x6A, 1, |gb| {
                    let l0 = gb.l();
                    let d = gb.d();
                    gb.set_l(d);
                    (
                        format!("LD L, D"),
                        format!("L₀ = ${:02x}, D = ${:02x}", l0, d),
                    )
                });
                op(0x6B, 1, |gb| {
                    let l0 = gb.l();
                    let e = gb.e();
                    gb.set_l(e);
                    (
                        format!("LD L, E"),
                        format!("L₀ = ${:02x}, E = ${:02x}", l0, e),
                    )
                });
                op(0x6C, 1, |gb| {
                    let l0 = gb.l();
                    let h = gb.h();
                    gb.set_l(h);
                    (
                        format!("LD L, H"),
                        format!("L₀ = ${:02x}, H = ${:02x}", l0, h),
                    )
                });
                op(0x6D, 1, |gb| {
                    let l0 = gb.l();
                    let l = gb.l();
                    gb.set_l(l);
                    (
                        format!("LD L, L"),
                        format!("L₀ = ${:02x}, L = ${:02x}", l0, l),
                    )
                });
                op(0x6E, 2, |gb| {
                    let l0 = gb.l();
                    let hl = gb.hl();
                    let l1 = gb.get_memory(hl);
                    gb.set_l(l1);
                    (
                        format!("LD L, (HL)"),
                        format!("L₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", l0, hl, l1),
                    )
                });
                // LD (HL), *
                op(0x70, 2, |gb| {
                    let hl = gb.hl();
                    let b = gb.b();
                    gb.set_memory(hl, b);
                    (
                        format!("LD (HL), B"),
                        format!("HL = ${:02x}, B = ${:02x}", hl, b),
                    )
                });
                op(0x71, 2, |gb| {
                    let hl = gb.hl();
                    let c = gb.c();
                    gb.set_memory(hl, c);
                    (
                        format!("LD (HL), C"),
                        format!("HL = ${:02x}, C = ${:02x}", hl, c),
                    )
                });
                op(0x72, 2, |gb| {
                    let hl = gb.hl();
                    let d = gb.d();
                    gb.set_memory(hl, d);
                    (
                        format!("LD (HL), D"),
                        format!("HL = ${:02x}, D = ${:02x}", hl, d),
                    )
                });
                op(0x73, 2, |gb| {
                    let hl = gb.hl();
                    let e = gb.e();
                    gb.set_memory(hl, e);
                    (
                        format!("LD (HL), E"),
                        format!("HL = ${:02x}, E = ${:02x}", hl, e),
                    )
                });
                op(0x74, 2, |gb| {
                    let hl = gb.hl();
                    let h = gb.h();
                    gb.set_memory(hl, h);
                    (
                        format!("LD (HL), H"),
                        format!("HL = ${:02x}, H = ${:02x}", hl, h),
                    )
                });
                op(0x75, 2, |gb| {
                    let hl = gb.hl();
                    let l = gb.l();
                    gb.set_memory(hl, l);
                    (
                        format!("LD (HL), L"),
                        format!("HL = ${:02x}, L = ${:02x}", hl, l),
                    )
                });
                op(0x36, 3, |gb| {
                    let hl = gb.hl();
                    let n = gb.read_immediate_u8();
                    gb.set_memory(hl, n);
                    (format!("LD (HL), ${:02x}", n), format!("HL = ${:02x}", hl))
                });
            }

            // 4. LD n, A
            // Put value A into n.
            {
                op(0x47, 1, |gb| {
                    let b0 = gb.b();
                    let a = gb.a();
                    gb.set_b(a);
                    (
                        format!("LD B, A"),
                        format!("B₀ = ${:02x}, A = ${:02x}", b0, a),
                    )
                });
                op(0x4F, 1, |gb| {
                    let c0 = gb.c();
                    let a = gb.a();
                    gb.set_b(a);
                    (
                        format!("LD C, A"),
                        format!("C₀ = ${:02x}, A = ${:02x}", c0, a),
                    )
                });
                op(0x57, 1, |gb| {
                    let d0 = gb.d();
                    let a = gb.a();
                    gb.set_d(a);
                    (
                        format!("LD D, A"),
                        format!("D₀ = ${:02x}, A = ${:02x}", d0, a),
                    )
                });
                op(0x5F, 1, |gb| {
                    let e0 = gb.e();
                    let a = gb.a();
                    gb.set_e(a);
                    (
                        format!("LD E, A"),
                        format!("E₀ = ${:02x}, A = ${:02x}", e0, a),
                    )
                });
                op(0x67, 1, |gb| {
                    let h0 = gb.h();
                    let a = gb.a();
                    gb.set_h(a);
                    (
                        format!("LD H, A"),
                        format!("H₀ = ${:02x}, A = ${:02x}", h0, a),
                    )
                });
                op(0x6F, 1, |gb| {
                    let l0 = gb.l();
                    let a = gb.a();
                    gb.set_l(a);
                    (
                        format!("LD L, A"),
                        format!("L₀ = ${:02x}, A = ${:02x}", l0, a),
                    )
                });
                op(0x02, 2, |gb| {
                    let bc = gb.bc();
                    let a = gb.a();
                    gb.set_memory(bc, a);
                    (
                        format!("LD (BC), A"),
                        format!("BC = ${:04x}, A = ${:02x}", bc, gb.a()),
                    )
                });
                op(0x12, 2, |gb| {
                    let de = gb.de();
                    let a = gb.a();
                    gb.set_memory(de, a);
                    (
                        format!("LD (DE), A"),
                        format!("DE = ${:04x}, A = ${:02x}", de, gb.a()),
                    )
                });
                op(0x77, 2, |gb| {
                    let hl = gb.hl();
                    let a = gb.a();
                    gb.set_memory(hl, a);
                    (
                        format!("LD (HL), A"),
                        format!("HL = ${:04x}, A = ${:02x}", hl, gb.a()),
                    )
                });
            }

            // 6. LD ($FF00 + C), A
            op(0xE2, 2, |gb| {
                let a = gb.a();
                let address = 0xFF00 + (gb.c() as u16);
                gb.set_memory(address, a);
                (
                    format!("LD ($FF00 + C), A "),
                    format!("A = ${:02x}, C = ${:02x}", gb.a(), gb.c()),
                )
            });

            // 12. LD (HL-), A
            // Put A into memory address HL.
            // Decrement HL.
            op(0x32, 2, |gb| {
                let hl0 = gb.hl();
                let hl1 = hl0 - 1;
                let a = gb.a();
                gb.set_memory(hl0, a);
                gb.set_hl(hl1);
                (
                    format!("LD (HL-), A"),
                    format!("HL₀ = ${:04x}, A = ${:02x}", hl0, a),
                )
            });

            // 19. LDH (n), A
            op(0xE0, 3, |gb| {
                let a = gb.a();
                let n = gb.read_immediate_u8();
                gb.set_memory(0xFF00 as u16 + n as u16, a);
                (
                    format!("LD ($ff00 + ${:02x}), A", n),
                    format!("A = ${:02x}", a),
                )
            });
        }

        // 3.3.2 16-Bit Loads
        {
            // 3.3.2. 16-Bit Loads
            // 1. LD n, nn
            // Put value nn into n.
            op(0x01, 3, |gb| {
                let bc0 = gb.bc();
                let bc1 = gb.read_immediate_u16();
                gb.set_bc(bc1);
                (
                    format!("LOAD BC, ${:04x}", bc1),
                    format!("BC₁ = ${:04x}", bc0),
                )
            });
            op(0x11, 3, |gb| {
                let de0 = gb.de();
                let de1 = gb.read_immediate_u16();
                gb.set_de(de1);
                (
                    format!("LOAD DE, ${:04x}", de1),
                    format!("DE₁ = ${:04x}", de0),
                )
            });
            op(0x21, 3, |gb| {
                let hl0 = gb.hl();
                let hl1 = gb.read_immediate_u16();
                gb.set_hl(hl1);
                (
                    format!("LOAD HL, ${:04x}", hl1),
                    format!("hl₁ = ${:04x}", hl0),
                )
            });
            op(0x31, 3, |gb| {
                let sp0 = gb.sp();
                let sp1 = gb.read_immediate_u16();
                gb.set_sp(sp1);
                (
                    format!("LOAD SP, ${:04x}", sp1),
                    format!("SP₀ = ${:04x}", sp0),
                )
            });

            // 2. LD SP, HL
            // Put HL into Stack Pointer (SP).
            op(0xF9, 2, |gb| {
                let sp0 = gb.sp();
                let hl = gb.hl();
                gb.set_sp(hl);
                (
                    format!("LOAD SP, HL"),
                    format!("SP₀ = ${:04x}, HL = ${:02x}", sp0, hl),
                )
            });

            // 5. PUSH nn
            // Push register pair nn onto stack.
            // Decrement Stack Pointer (SP) twice.
            op(0xF5, 4, |gb| {
                let af = gb.af();
                gb.stack_push(af);
                (
                    format!("PUSH AF"),
                    format!("SP₁ = ${:04x}, AF = ${:04x}", gb.sp(), af),
                )
            });
            op(0xC5, 4, |gb| {
                let bc = gb.bc();
                gb.stack_push(bc);
                (
                    format!("PUSH BC"),
                    format!("SP₁ = ${:04x}, BC = ${:04x}", gb.sp(), bc),
                )
            });
            op(0xD5, 4, |gb| {
                let de = gb.de();
                gb.stack_push(de);
                (
                    format!("PUSH DE"),
                    format!("SP₁ = ${:04x}, DE = ${:04x}", gb.sp(), de),
                )
            });
            op(0xE5, 4, |gb| {
                let hl = gb.hl();
                gb.stack_push(hl);
                (
                    format!("PUSH hl"),
                    format!("SP₁ = ${:04x}, HL = ${:04x}", gb.sp(), hl),
                )
            });

            // 7. POP nn
            // Push two bytes off stack into register pair nn.
            // Increment Stack Pointer (SP) twice.
            op(0xF1, 3, |gb| {
                let af0 = gb.af();
                let af1 = gb.stack_pop();
                (
                    format!("POP AF"),
                    format!(
                        "SP₁ = ${:04x}, AF₀ = ${:04x}, AF₁ = ${:04x}",
                        gb.sp(),
                        af0,
                        af1
                    ),
                )
            });
            op(0xC1, 3, |gb| {
                let bc0 = gb.bc();
                let bc1 = gb.stack_pop();
                (
                    format!("POP BC"),
                    format!(
                        "SP₁ = ${:04x}, BC₀ = ${:04x}, BC₁ = ${:04x}",
                        gb.sp(),
                        bc0,
                        bc1
                    ),
                )
            });
            op(0xD1, 3, |gb| {
                let de0 = gb.de();
                let de1 = gb.stack_pop();
                (
                    format!("POP DE"),
                    format!(
                        "SP₁ = ${:04x}, DE₀ = ${:04x}, DE₁ = ${:04x}",
                        gb.sp(),
                        de0,
                        de1
                    ),
                )
            });
            op(0xE1, 3, |gb| {
                let hl0 = gb.hl();
                let hl1 = gb.stack_pop();
                (
                    format!("POP HL"),
                    format!(
                        "SP₁ = ${:04x}, HL₀ = ${:04x}, HL₁ = ${:04x}",
                        gb.sp(),
                        hl0,
                        hl1
                    ),
                )
            });
        }

        // 3.3.3. 8-Bit ALU
        {
            // 7. XOR n
            {
                op(0xAF, 1, |gb| {
                    let a0 = gb.a();
                    let a1 = a0 ^ a0;
                    gb.set_a(a1);
                    gb.set_z_flag(a1 == 0);
                    gb.set_n_flag(false);
                    gb.set_h_flag(false);
                    gb.set_c_flag(false);
                    (
                        format!("XOR A"),
                        format!("A₀ = ${:02x}, A₁ = ${:02x}", a0, a1),
                    )
                });
                op(0xA8, 1, |gb| {
                    let b = gb.b();
                    let a0 = gb.a();
                    let a1 = a0 ^ b;
                    gb.set_a(a1);
                    gb.set_z_flag(a1 == 0);
                    gb.set_n_flag(false);
                    gb.set_h_flag(false);
                    gb.set_c_flag(false);
                    (
                        format!("XOR B"),
                        format!("A₀ = ${:02x}, B = ${:02x} A₁ = ${:02x}", a0, b, a1),
                    )
                });

                op(0xA9, 1, |gb| {
                    let c = gb.c();
                    let a0 = gb.a();
                    let a1 = a0 ^ c;
                    gb.set_a(a1);
                    gb.set_z_flag(a1 == 0);
                    gb.set_n_flag(false);
                    gb.set_h_flag(false);
                    gb.set_c_flag(false);
                    (
                        format!("XOR C"),
                        format!("A₀ = ${:02x}, C = ${:02x} A₁ = ${:02x}", a0, c, a1),
                    )
                });
                op(0xAA, 1, |gb| {
                    let d = gb.d();
                    let a0 = gb.a();
                    let a1 = a0 ^ d;
                    gb.set_a(a1);
                    gb.set_z_flag(a1 == 0);
                    gb.set_n_flag(false);
                    gb.set_h_flag(false);
                    gb.set_c_flag(false);
                    (
                        format!("XOR D"),
                        format!("A₀ = ${:02x}, D = ${:02x} A₁ = ${:02x}", a0, d, a1),
                    )
                });

                op(0xAB, 1, |gb| {
                    let e = gb.e();
                    let a0 = gb.a();
                    let a1 = a0 ^ e;
                    gb.set_a(a1);
                    gb.set_z_flag(a1 == 0);
                    gb.set_n_flag(false);
                    gb.set_h_flag(false);
                    gb.set_c_flag(false);
                    (
                        format!("XOR E"),
                        format!("A₀ = ${:02x}, E = ${:02x} A₁ = ${:02x}", a0, e, a1),
                    )
                });
                op(0xAC, 1, |gb| {
                    let h = gb.h();
                    let a0 = gb.a();
                    let a1 = a0 ^ h;
                    gb.set_a(a1);
                    gb.set_z_flag(a1 == 0);
                    gb.set_n_flag(false);
                    gb.set_h_flag(false);
                    gb.set_c_flag(false);
                    (
                        format!("XOR B"),
                        format!("A₀ = ${:02x}, H = ${:02x} A₁ = ${:02x}", a0, h, a1),
                    )
                });
                op(0xAD, 1, |gb| {
                    let l = gb.l();
                    let a0 = gb.a();
                    let a1 = a0 ^ l;
                    gb.set_a(a1);
                    gb.set_z_flag(a1 == 0);
                    gb.set_n_flag(false);
                    gb.set_h_flag(false);
                    gb.set_c_flag(false);
                    (
                        format!("XOR L"),
                        format!("A₀ = ${:02x}, L = ${:02x} A₁ = ${:02x}", a0, l, a1),
                    )
                });
            }

            // 9. INC n
            op(0x3C, 1, |gb| {
                let a0 = gb.a();
                let a1 = a0 + 1;
                gb.set_a(a1);
                gb.set_z_flag(a1 == 0);
                gb.set_n_flag(false);
                gb.set_h_flag(a0 > a1);
                (
                    format!("INC A"),
                    format!("A₀ = ${:02x}, A₁ = ${:02x}", a0, a1),
                )
            });
            op(0x04, 1, |gb| {
                let b0 = gb.b();
                let b1 = b0 + 1;
                gb.set_a(b1);
                gb.set_z_flag(b1 == 0);
                gb.set_n_flag(false);
                gb.set_h_flag(b0 > b1);
                (
                    format!("INC B"),
                    format!("B₀ = ${:02x}, B₁ = ${:02x}", b0, b1),
                )
            });
            op(0x0C, 1, |gb| {
                let c0 = gb.c();
                let c1 = c0 + 1;
                gb.set_a(c1);
                gb.set_z_flag(c1 == 0);
                gb.set_n_flag(false);
                gb.set_h_flag(c0 > c1);
                (
                    format!("INC C"),
                    format!("C₀ = ${:02x}, C₁ = ${:02x}", c0, c1),
                )
            });
            op(0x14, 1, |gb| {
                let d0 = gb.d();
                let d1 = d0 + 1;
                gb.set_a(d1);
                gb.set_z_flag(d1 == 0);
                gb.set_n_flag(false);
                gb.set_h_flag(d0 > d1);
                (
                    format!("INC D"),
                    format!("D₀ = ${:02x}, D₁ = ${:02x}", d0, d1),
                )
            });
            op(0x1C, 1, |gb| {
                let e0 = gb.e();
                let e1 = e0 + 1;
                gb.set_a(e1);
                gb.set_z_flag(e1 == 0);
                gb.set_n_flag(false);
                gb.set_h_flag(e0 > e1);
                (
                    format!("INC E"),
                    format!("E₀ = ${:02x}, E₁ = ${:02x}", e0, e1),
                )
            });
            op(0x24, 1, |gb| {
                let h0 = gb.h();
                let h1 = h0 + 1;
                gb.set_a(h1);
                gb.set_z_flag(h1 == 0);
                gb.set_n_flag(false);
                gb.set_h_flag(h0 > h1);
                (
                    format!("INC H"),
                    format!("H₀ = ${:02x}, H₁ = ${:02x}", h0, h1),
                )
            });
            op(0x2C, 1, |gb| {
                let l0 = gb.l();
                let l1 = l0 + 1;
                gb.set_a(l1);
                gb.set_z_flag(l1 == 0);
                gb.set_n_flag(false);
                gb.set_h_flag(l0 > l1);
                (
                    format!("INC L"),
                    format!("L₀ = ${:02x}, L₁ = ${:02x}", l0, l1),
                )
            });
        }

        // 3.3.5. Miscellaneous
        {
            // 6. NOP
            op(0x00, 1, |_gb| (format!("NOP"), format!("")));
        }

        // 3.3.6. Rotates & Shifts
        {
            // 2. RLA
            // Rotate A left through Carry flag.
            // This, 0x17, is the same as 0xCB17 below.
            op(0x17, 2, |gb| {
                let a0 = gb.a();
                let a1 = (a0 << 1) + if gb.c_flag() { 1 } else { 0 };
                gb.set_a(a1);
                gb.set_z_flag(a1 == 0);
                gb.set_c_flag(a0 & 0b10000000 > 0);
                gb.set_n_flag(false);
                gb.set_h_flag(false);
                (format!("RLA"), format!("A₀ = {}", a0))
            });

            // 6. RL n
            // Rotate n left through Carry flag.
            op_cb(0x17, 2, |gb| {
                let a0 = gb.a();
                let a1 = a0 << 1 + if gb.c_flag() { 1 } else { 0 };
                gb.set_a(a1);
                gb.set_z_flag(a1 == 0);
                gb.set_c_flag(a0 & 0b10000000 > 0);
                gb.set_n_flag(false);
                gb.set_h_flag(false);
                (format!("RL A"), format!("A₀ = {}", a0))
            });
            op_cb(0x10, 2, |gb| {
                let b0 = gb.b();
                let b1 = b0 << 1 + if gb.c_flag() { 1 } else { 0 };
                gb.set_b(b1);
                gb.set_z_flag(b1 == 0);
                gb.set_c_flag(b0 & 0b10000000 > 0);
                gb.set_n_flag(false);
                gb.set_h_flag(false);
                (format!("RL B"), format!("B₀ = {}", b0))
            });
            op_cb(0x11, 2, |gb| {
                let c0 = gb.c();
                let c1 = c0 << 1 + if gb.c_flag() { 1 } else { 0 };
                gb.set_c(c1);
                gb.set_z_flag(c1 == 0);
                gb.set_c_flag(c0 & 0b10000000 > 0);
                gb.set_n_flag(false);
                gb.set_h_flag(false);
                (format!("RL C"), format!("C₀ = {}", c0))
            });
            op_cb(0x12, 2, |gb| {
                let d0 = gb.d();
                let d1 = d0 << 1 + if gb.c_flag() { 1 } else { 0 };
                gb.set_d(d1);
                gb.set_z_flag(d1 == 0);
                gb.set_c_flag(d0 & 0b10000000 > 0);
                gb.set_n_flag(false);
                gb.set_h_flag(false);
                (format!("RL D"), format!("D₀ = {}", d0))
            });
            op_cb(0x13, 2, |gb| {
                let e0 = gb.e();
                let e1 = e0 << 1 + if gb.c_flag() { 1 } else { 0 };
                gb.set_e(e1);
                gb.set_z_flag(e1 == 0);
                gb.set_c_flag(e0 & 0b10000000 > 0);
                gb.set_n_flag(false);
                gb.set_h_flag(false);
                (format!("RL E"), format!("E₀ = {}", e0))
            });
            op_cb(0x14, 2, |gb| {
                let h0 = gb.h();
                let h1 = h0 << 1 + if gb.c_flag() { 1 } else { 0 };
                gb.set_h(h1);
                gb.set_z_flag(h1 == 0);
                gb.set_c_flag(h0 & 0b10000000 > 0);
                gb.set_n_flag(false);
                gb.set_h_flag(false);
                (format!("RL H"), format!("H₀ = {}", h0))
            });
            op_cb(0x15, 2, |gb| {
                let l0 = gb.l();
                let l1 = l0 << 1 + if gb.c_flag() { 1 } else { 0 };
                gb.set_l(l1);
                gb.set_z_flag(l1 == 0);
                gb.set_c_flag(l0 & 0b10000000 > 0);
                gb.set_n_flag(false);
                gb.set_h_flag(false);
                (format!("RL L"), format!("L₀ = {}", l0))
            });
        }

        // 3.3.7. Bit Opcodes
        {
            op_cb(0x7C, 2, |gb| {
                let result = !u8_get_bit(gb.h(), 7);
                gb.set_z_flag(result);
                gb.set_n_flag(false);
                gb.set_h_flag(true);
                (format!("BIT 7, H"), format!("Z₁ = {}", result))
            });
        }

        // 3.3.8. Jumps
        {
            // 1. JP nn
            // Jump to address nn.
            op(0xC3, 3, |gb| {
                let nn = gb.read_immediate_u16();
                gb.i = nn;
                (format!("JP ${:04x}", nn), format!(""))
            });
            // 2. JP cc, nn
            // Jump to address n if condition is true.
            op(0xC2, 3, |gb| {
                let nn = gb.read_immediate_u16();
                let z_flag = gb.z_flag();
                if z_flag == false {
                    gb.i = nn;
                }
                (format!("JP NZ, ${:04x}", nn), format!("Z = {}", z_flag))
            });
            op(0xCA, 3, |gb| {
                let nn = gb.read_immediate_u16();
                let z_flag = gb.z_flag();
                if z_flag {
                    gb.i = nn;
                }
                (format!("JP Z, ${:04x}", nn), format!("Z = {}", z_flag))
            });
            op(0xD2, 3, |gb| {
                let nn = gb.read_immediate_u16();
                let c_flag = gb.c_flag();
                if c_flag == false {
                    gb.i = nn;
                }
                (format!("JP NC, ${:04x}", nn), format!("C = {}", c_flag))
            });
            op(0xDA, 3, |gb| {
                let nn = gb.read_immediate_u16();
                let c_flag = gb.c_flag();
                if c_flag {
                    gb.i = nn;
                }
                (format!("JP C, ${:04x}", nn), format!("C = {}", c_flag))
            });
            // 3. JP (HL)
            // Jump to address contained in HL.
            op(0xE9, 1, |gb| {
                let hl = gb.hl();
                (format!("JP (HL)"), format!("HL = ${:04x}", hl))
            });
            // 4. JR n
            // Add n to current address and jump to it.
            op(0x18, 2, |gb| {
                let n = gb.read_immediate_i8();
                (format!("JP {}", n), format!(""))
            });
            // 5. JR cc, n
            // If condition is true then add n to current address and jump to it.
            op(0x20, 2, |gb| {
                let n = gb.read_immediate_i8();
                let z_flag = gb.z_flag();
                if z_flag == false {
                    gb.relative_jump(n);
                }
                (format!("JR NZ, {}", n), format!("Z = {}", z_flag))
            });
            op(0x28, 2, |gb| {
                let n = gb.read_immediate_i8();
                let z_flag = gb.z_flag();
                if z_flag {
                    gb.relative_jump(n);
                }
                (format!("JR Z, {}", n), format!("Z = {}", z_flag))
            });
            op(0x30, 2, |gb| {
                let n = gb.read_immediate_i8();
                let c_flag = gb.c_flag();
                if c_flag == false {
                    gb.relative_jump(n);
                }
                (format!("JR NC, {}", n), format!("C = {}", c_flag))
            });
            op(0x38, 2, |gb| {
                let n = gb.read_immediate_i8();
                let c_flag = gb.c_flag();
                if c_flag {
                    gb.relative_jump(n);
                }
                (format!("JR C, {}", n), format!("C = {}", c_flag))
            });
        }

        // 3.3.9. Calls
        {
            // 1. CALL nn
            // Push address of next instruction onto stack and
            // then jump to address nn.

            op(0xCD, 3, |gb| {
                let nn = gb.read_immediate_u16();
                let i0 = gb.i;
                gb.stack_push(i0);
                gb.i = nn;
                (
                    format!("CALL ${:04x}", nn),
                    format!("SP₁ = {:04x}", gb.sp()),
                )
            });
        }
    }

    (operations, operations_cb)
}

fn load_boot_rom() -> Vec<u8> {
    return vec![
        0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF,
        0x0E, 0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E,
        0xFC, 0xE0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0, 0xCD, 0x96, 0,
        0x13, 0x7B, 0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23,
        0x05, 0x20, 0xF9, 0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28,
        0x08, 0x32, 0x0D, 0x20, 0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42,
        0x3E, 0x91, 0xE0, 0x40, 0x04, 0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA,
        0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2, 0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28,
        0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06, 0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42,
        0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20, 0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06,
        0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17, 0x05, 0x20, 0xF5, 0x22, 0x23, 0x22,
        0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0, 0x0B, 0x03, 0x73, 0, 0x83, 0, 0x0C, 0,
        0x0D, 0, 0x08, 0x11, 0x1F, 0x88, 0x89, 0, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9,
        0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9,
        0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C, 0x21, 0x04, 0x01, 0x11, 0xA8,
        0, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20, 0xF5, 0x06, 0x19, 0x78,
        0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
    ];
}

fn load_game_rom(game_name: &str) -> Vec<u8> {
    match game_name {
        "Tetris[:1024]" => {
            return vec![
                0xC3, 0x0C, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC3, 0x0C, 0x02, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x87, 0xE1,
                0x5F, 0x16, 0x00, 0x19, 0x5E, 0x23, 0x56, 0xD5, 0xE1, 0xE9, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC3, 0x7E, 0x01, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xC3, 0xBE, 0x26, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC3, 0xBE, 0x26, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xC3, 0x5B, 0x00, 0xF5, 0xE5, 0xD5, 0xC5, 0xCD, 0x6B, 0x00,
                0x3E, 0x01, 0xE0, 0xCC, 0xC1, 0xD1, 0xE1, 0xF1, 0xD9, 0xF0, 0xCD, 0xEF, 0x78, 0x00,
                0x9F, 0x00, 0xA4, 0x00, 0xBA, 0x00, 0xEA, 0x27, 0xF0, 0xE1, 0xFE, 0x07, 0x28, 0x08,
                0xFE, 0x06, 0xC8, 0x3E, 0x06, 0xE0, 0xE1, 0xC9, 0xF0, 0x01, 0xFE, 0x55, 0x20, 0x08,
                0x3E, 0x29, 0xE0, 0xCB, 0x3E, 0x01, 0x18, 0x08, 0xFE, 0x29, 0xC0, 0x3E, 0x55, 0xE0,
                0xCB, 0xAF, 0xE0, 0x02, 0xC9, 0xF0, 0x01, 0xE0, 0xD0, 0xC9, 0xF0, 0x01, 0xE0, 0xD0,
                0xF0, 0xCB, 0xFE, 0x29, 0xC8, 0xF0, 0xCF, 0xE0, 0x01, 0x3E, 0xFF, 0xE0, 0xCF, 0x3E,
                0x80, 0xE0, 0x02, 0xC9, 0xF0, 0x01, 0xE0, 0xD0, 0xF0, 0xCB, 0xFE, 0x29, 0xC8, 0xF0,
                0xCF, 0xE0, 0x01, 0xFB, 0xCD, 0x98, 0x0A, 0x3E, 0x80, 0xE0, 0x02, 0xC9, 0xF0, 0xCD,
                0xFE, 0x02, 0xC0, 0xAF, 0xE0, 0x0F, 0xFB, 0xC9, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xC3, 0x50, 0x01, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D,
                0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F,
                0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB,
                0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
                0x54, 0x45, 0x54, 0x52, 0x49, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0A, 0x16, 0xBF,
                0xC3, 0x0C, 0x02, 0xCD, 0xE3, 0x29, 0xF0, 0x41, 0xE6, 0x03, 0x20, 0xFA, 0x46, 0xF0,
                0x41, 0xE6, 0x03, 0x20, 0xFA, 0x7E, 0xA0, 0xC9, 0x7B, 0x86, 0x27, 0x22, 0x7A, 0x8E,
                0x27, 0x22, 0x3E, 0x00, 0x8E, 0x27, 0x77, 0x3E, 0x01, 0xE0, 0xE0, 0xD0, 0x3E, 0x99,
                0x32, 0x32, 0x77, 0xC9, 0xF5, 0xC5, 0xD5, 0xE5, 0xF0, 0xCE, 0xA7, 0x28, 0x12, 0xF0,
                0xCB, 0xFE, 0x29, 0x20, 0x0C, 0xAF, 0xE0, 0xCE, 0xF0, 0xCF, 0xE0, 0x01, 0x21, 0x02,
                0xFF, 0x36, 0x81, 0xCD, 0xE0, 0x21, 0xCD, 0xCC, 0x23, 0xCD, 0xB7, 0x23, 0xCD, 0x9E,
                0x23, 0xCD, 0x8C, 0x23, 0xCD, 0x7D, 0x23, 0xCD, 0x6E, 0x23, 0xCD, 0x5F, 0x23, 0xCD,
                0x50, 0x23, 0xCD, 0x41, 0x23, 0xCD, 0x32, 0x23, 0xCD, 0x23, 0x23, 0xCD, 0xF8, 0x22,
                0xCD, 0xE9, 0x22, 0xCD, 0xDA, 0x22, 0xCD, 0xCB, 0x22, 0xCD, 0xBC, 0x22, 0xCD, 0xAD,
                0x22, 0xCD, 0x9E, 0x22, 0xCD, 0xD7, 0x1E, 0xCD, 0xB6, 0xFF, 0xCD, 0xCA, 0x18, 0xFA,
                0xCE, 0xC0, 0xA7, 0x28, 0x1A, 0xF0, 0x98, 0xFE, 0x03, 0x20, 0x14, 0x21, 0x6D, 0x98,
                0xCD, 0x3B, 0x24, 0x3E, 0x01, 0xE0, 0xE0, 0x21, 0x6D, 0x9C, 0xCD, 0x3B, 0x24, 0xAF,
                0xEA, 0xCE, 0xC0, 0x21, 0xE2, 0xFF, 0x34, 0xAF, 0xE0, 0x43, 0xE0, 0x42, 0x3C, 0xE0,
                0x85, 0xE1, 0xD1, 0xC1, 0xF1, 0xD9, 0xAF, 0x21, 0xFF, 0xDF, 0x0E, 0x10, 0x06, 0x00,
                0x32, 0x05, 0x20, 0xFC, 0x0D, 0x20, 0xF9, 0x3E, 0x01, 0xF3, 0xE0, 0x0F, 0xE0, 0xFF,
                0xAF, 0xE0, 0x42, 0xE0, 0x43, 0xE0, 0xA4, 0xE0, 0x41, 0xE0, 0x01, 0xE0, 0x02, 0x3E,
                0x80, 0xE0, 0x40, 0xF0, 0x44, 0xFE, 0x94, 0x20, 0xFA, 0x3E, 0x03, 0xE0, 0x40, 0x3E,
                0xE4, 0xE0, 0x47, 0xE0, 0x48, 0x3E, 0xC4, 0xE0, 0x49, 0x21, 0x26, 0xFF, 0x3E, 0x80,
                0x32, 0x3E, 0xFF, 0x32, 0x36, 0x77, 0x3E, 0x01, 0xEA, 0x00, 0x20, 0x31, 0xFF, 0xCF,
                0xAF, 0x21, 0xFF, 0xDF, 0x06, 0x00, 0x32, 0x05, 0x20, 0xFC, 0x21, 0xFF, 0xCF, 0x0E,
                0x10, 0x06, 0x00, 0x32, 0x05, 0x20, 0xFC, 0x0D, 0x20, 0xF9, 0x21, 0xFF, 0x9F, 0x0E,
                0x20, 0xAF, 0x06, 0x00, 0x32, 0x05, 0x20, 0xFC, 0x0D, 0x20, 0xF9, 0x21, 0xFF, 0xFE,
                0x06, 0x00, 0x32, 0x05, 0x20, 0xFC, 0x21, 0xFE, 0xFF, 0x06, 0x80, 0x32, 0x05, 0x20,
                0xFC, 0x0E, 0xB6, 0x06, 0x0C, 0x21, 0x7F, 0x2A, 0x2A, 0xE2, 0x0C, 0x05, 0x20, 0xFA,
                0xCD, 0x95, 0x27, 0xCD, 0xF3, 0x7F, 0x3E, 0x09, 0xE0, 0xFF, 0x3E, 0x37, 0xE0, 0xC0,
                0x3E, 0x1C, 0xE0, 0xC1, 0x3E, 0x24, 0xE0, 0xE1, 0x3E, 0x80, 0xE0, 0x40, 0xFB, 0xAF,
                0xE0, 0x0F, 0xE0, 0x4A, 0xE0, 0x4B, 0xE0, 0x06, 0xCD, 0xA6, 0x29, 0xCD, 0xF8, 0x02,
                0xCD, 0xF0, 0x7F, 0xF0, 0x80, 0xE6, 0x0F, 0xFE, 0x0F, 0xCA, 0x1B, 0x02, 0x21, 0xA6,
                0xFF, 0x06, 0x02, 0x7E, 0xA7, 0x28, 0x01, 0x35, 0x2C, 0x05, 0x20, 0xF7, 0xF0, 0xC5,
                0xA7, 0x28, 0x04, 0x3E, 0x09, 0xE0, 0xFF, 0xF0, 0x85, 0xA7, 0x28, 0xFB, 0xAF, 0xE0,
                0x85, 0xC3, 0xC4, 0x02, 0xF0, 0xE1, 0xEF, 0xCE, 0x1B, 0xE2, 0x1C, 0x44, 0x12, 0x7B,
                0x12, 0x06, 0x1D, 0x26, 0x1D, 0xAE, 0x03, 0x79, 0x04, 0x44, 0x14, 0x8C, 0x14, 0x07,
                0x1A, 0xC0, 0x1D, 0x16, 0x1F, 0x1F, 0x1F, 0x25, 0x15, 0xB0, 0x14, 0x7B, 0x15, 0xBF,
                0x15, 0x29, 0x16, 0x7A, 0x16, 0xEB, 0x16, 0x13, 0x19, 0x77, 0x06, 0x2C, 0x07, 0x25,
                0x08, 0xE4, 0x08, 0x31, 0x0B, 0xEB, 0x0C, 0xD2, 0x0A, 0x32, 0x0D, 0x23, 0x0E, 0x12,
                0x11, 0x99, 0x0D, 0x8A, 0x0E, 0xCE, 0x1D, 0x41, 0x1E, 0x69, 0x03, 0x93, 0x03, 0x67,
                0x11, 0xE6, 0x11, 0xFC, 0x11, 0x1C, 0x12, 0xC7, 0x05, 0xF7, 0x05, 0xB3, 0x12, 0x05,
                0x13, 0x24, 0x13, 0x51, 0x13, 0x67, 0x13, 0x7E, 0x13, 0xB5, 0x13, 0xE5, 0x13, 0x1B,
                0x13, 0xA0, 0x03, 0xEA, 0x27, 0xCD, 0x20, 0x28, 0xCD, 0xD7, 0x27, 0x11, 0x07, 0x4A,
                0xCD, 0xEB, 0x27, 0xCD, 0x8A, 0x17, 0x21, 0x00, 0xC3, 0x11, 0x50, 0x64, 0x1A, 0x22,
                0x13, 0x7C, 0xFE, 0xC4, 0x20, 0xF8, 0x3E, 0xD3, 0xE0, 0x40, 0x3E, 0xFA, 0xE0, 0xA6,
                0x3E, 0x25, 0xE0, 0xE1, 0xC9, 0xF0, 0xA6, 0xA7, 0xC0, 0x3E, 0xFA, 0xE0, 0xA6, 0x3E,
                0x35, 0xE0, 0xE1, 0xC9, 0xF0, 0x81, 0xA7, 0x20, 0x04, 0xF0, 0xA6, 0xA7, 0xC0, 0x3E,
                0x06, 0xE0, 0xE1, 0xC9, 0xCD, 0x20, 0x28, 0xAF, 0xE0, 0xE9, 0xE0, 0x98, 0xE0, 0x9C,
                0xE0, 0x9B, 0xE0, 0xFB, 0xE0, 0x9F, 0xE0, 0xE3, 0xE0, 0xC7, 0xCD, 0x93, 0x22, 0xCD,
                0x51, 0x26, 0xCD, 0xD7, 0x27, 0x21, 0x00, 0xC8, 0x3E, 0x2F, 0x22, 0x7C, 0xFE, 0xCC,
                0x20, 0xF8, 0x21, 0x01, 0xC8, 0xCD, 0xA9, 0x26, 0x21, 0x0C, 0xC8, 0xCD, 0xA9, 0x26,
                0x21, 0x41, 0xCA, 0x06, 0x0C, 0x3E, 0x8E, 0x22, 0x05, 0x20, 0xFC, 0x11, 0x6F, 0x4B,
                0xCD, 0xEB, 0x27, 0xCD, 0x8A, 0x17, 0x21, 0x00, 0xC0, 0x36, 0x80, 0x2C, 0x36, 0x10,
                0x2C, 0x36,
            ];
        }

        _ => panic!("Game ROM Not Available: {}", game_name),
    }
}
