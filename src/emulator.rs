use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GameBoy {
    // time/ticks since start
    t: u64,
    // instruction pointer/index
    i: u16,
    main_ram: [u8; 8192],
    video_ram: [u8; 8192],
    high_ram: [u8; 127],
    main_registers: [u8; 12],
    boot_rom: Vec<u8>,
    game_rom: Vec<u8>,
    // the 4-item one-byte 2-bit-greyscale color table at $FF47
    bg_palette: u8,
    debug_current_op_addr: u16,
    debug_current_code: Vec<u8>,

    frame_buffer: Arc<Mutex<Vec<u8>>>,
}

struct Operation {
    code: u8,
    cycles: u8,
    execute: fn(gb: &mut GameBoy) -> (String, String),
}

fn get_operations() -> HashMap<u8, Operation> {
    let mut operations = HashMap::new();

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
                // op(0x0A, 2, |gb| {
                //     let a0 = gb.a();
                //     let bc = gb.bc();
                //     let a1 = gb.get_memory(bc);
                //     gb.set_a(a1);
                //     (
                //         format!("LD A, (BC)"),
                //         format!("A₀ = ${:02x}, BC = ${:04x}, (BC) = ${:04x}", a0, bc, a1),
                //     )
                // });
                // op(0x1A, 2, |gb| {
                //     let a0 = gb.a();
                //     let de = gb.de();
                //     let a1 = gb.get_memory(de);
                //     gb.set_a(a1);
                //     (
                //         format!("LD A, (DE)"),
                //         format!("A₀ = ${:02x}, DE = ${:04x}, (DE) = ${:04x}", a0, de, a1),
                //     )
                // });
                op(0x7E, 2, |gb| {
                    let a0 = gb.a();
                    let hl = gb.hl();
                    let a1 = gb.get_memory(hl);
                    gb.set_a(a1);
                    (
                        format!("LD A, (HL)"),
                        format!("A₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", a0, hl, a1),
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
                op(0x51, 1, |gb| {
                    let d0 = gb.d();
                    let b = gb.b();
                    gb.set_d(b);
                    (
                        format!("LD D, B"),
                        format!("D₀ = ${:02x}, B = ${:02x}", d0, b),
                    )
                });
                op(0x52, 1, |gb| {
                    let d0 = gb.d();
                    let c = gb.c();
                    gb.set_d(c);
                    (
                        format!("LD D, C"),
                        format!("D₀ = ${:02x}, C = ${:02x}", d0, c),
                    )
                });
                op(0x53, 1, |gb| {
                    let d = gb.d();
                    (format!("LD D, D"), format!("D = ${:02x}", d))
                });
                op(0x54, 1, |gb| {
                    let d0 = gb.d();
                    let e = gb.e();
                    gb.set_d(e);
                    (
                        format!("LD D, E"),
                        format!("D₀ = ${:02x}, E = ${:02x}", d0, e),
                    )
                });
                op(0x55, 1, |gb| {
                    let d0 = gb.d();
                    let h = gb.h();
                    gb.set_d(h);
                    (
                        format!("LD D, H"),
                        format!("D₀ = ${:02x}, H = ${:02x}", d0, h),
                    )
                });
                op(0x56, 1, |gb| {
                    let d0 = gb.d();
                    let l = gb.l();
                    gb.set_d(l);
                    (
                        format!("LD D, L"),
                        format!("D₀ = ${:02x}, L = ${:02x}", d0, l),
                    )
                });
                op(0x57, 2, |gb| {
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
            // 1. LD n, nn
            // Put value nn into 16-bit register n.
            // TODO
        }

        // 3.3.5. Miscellaneous
        {
            // 6. NOP
            op(0x00, 1, |_gb| (format!("NOP"), format!("")));
        }

        // 3.3.8 Jumps
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

        // Two-Byte Operations
        {
            op(0xCB, 0, |gb| {
                let opcode_2 = gb.read_immediate_u8();

                match opcode_2 {
                    0x7C => {
                        let result = !u8_get_bit(gb.h(), 7);
                        gb.set_z_flag(result);
                        gb.set_n_flag(false);
                        gb.set_h_flag(true);
                        (format!("BIT 7, H"), format!("Z₁ = {}", result))
                    }

                    _ => {
                        gb.print_current_code(format!("; ERROR: unsupported opcode"), format!(""));
                        panic!("unsupported opcode: $CB ${:02x}", opcode_2);
                    }
                }
            });
        }
    }

    operations
}

impl GameBoy {
    pub fn new(frame_buffer: Arc<Mutex<Vec<u8>>>) -> GameBoy {
        GameBoy {
            t: 0,
            i: 0,
            main_ram: [0u8; 8192],
            video_ram: [0u8; 8192],
            high_ram: [0u8; 127],
            main_registers: [0u8; 12],
            boot_rom: load_boot_rom(),
            game_rom: load_game_rom("Pokemon Red (US)[:256]"),
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
        let operations = get_operations();
        println!(
            "; {:3} one-byte opcodes implemented (~{:3.0}%).",
            operations.len(),
            (operations.len() as f32 / 2.55)
        );
        println!();

        println!("; assembly:                        addr:   t/μs:   codes:       flags:");
        println!("; ---------                        -----   -----   ------       ------");

        loop {
            let opcode = self.read_instruction();

            let op = operations.get(&opcode);
            match op {
                Some(op) => {
                    let (asm, debug) = (op.execute)(self);
                    self.print_current_code(asm, debug);
                    self.t += op.cycles as u64;
                }
                None => {
                    match opcode {
                        0x21 => {
                            // LOAD HL, $1, $2
                            let hl = self.read_immediate_u16();
                            self.print_current_code(format!("LOAD HL, ${:04x}", hl), format!(""));
                            self.set_hl(hl);
                        }

                        0x31 => {
                            // LOAD SP, $1, $2
                            let sp = self.read_immediate_u16();
                            self.print_current_code(format!("LOAD SP ${:04x}", sp), format!(""));
                            self.set_sp(sp);
                        }

                        0x77 => {
                            // Put A into memory address HL.
                            self.print_current_code(
                                "LD (HL), A".to_string(),
                                format!("HL = ${:04x}, A = ${:02x}", self.hl(), self.a()),
                            );
                            let mut hl = self.hl();
                            let a = self.a();
                            self.set_memory(hl, a);
                        }

                        0x32 => {
                            // Put A into memory address HL.
                            self.print_current_code(
                                "LD (HL-), A".to_string(),
                                format!("HL₀ = ${:04x}, A = ${:02x}", self.hl(), self.a()),
                            );
                            let mut hl = self.hl();
                            let a = self.a();
                            self.set_memory(hl, a);
                            //  Decrement HL.
                            hl -= 1;
                            self.set_hl(hl);
                        }

                        0xE2 => {
                            // Put A into memory address 0xFF00 + C.
                            self.print_current_code(
                                "LD ($FF00 + C), A ".to_string(),
                                format!("A = ${:02x}, C = ${:02x}", self.a(), self.c()),
                            );
                            let a = self.a();
                            let address = 0xFF00 + (self.c() as u16);
                            self.set_memory(address, a);
                        }

                        0xAF => {
                            self.print_current_code(
                                "XOR A A".to_string(),
                                format!("A₀ = ${:02x}, A₁ = $00", self.a()).to_string(),
                            );
                            self.set_a(0);
                        }

                        // 8-Bit Arithmatic
                        // Increment the value in register n.
                        // Z flag set iff result is 0.
                        // N flag cleared.
                        // H flag set iff value overflows and wraps.
                        0x3C => {
                            let old_value = self.a();
                            let new_value = old_value + 1;
                            self.print_current_code(
                                "INC A".to_string(),
                                format!("A₀ = ${:02x}, A₁ = ${:02x}", old_value, new_value)
                                    .to_string(),
                            );
                            self.set_a(new_value);
                            self.set_z_flag(new_value == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(old_value > new_value);
                        }
                        0x04 => {
                            let old_value = self.b();
                            let new_value = old_value + 1;
                            self.print_current_code(
                                "INC B".to_string(),
                                format!("B₀ = ${:02x}, B₁ = ${:02x}", old_value, new_value)
                                    .to_string(),
                            );
                            self.set_b(new_value);
                            self.set_z_flag(new_value == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(old_value > new_value);
                        }
                        0x0C => {
                            let old_value = self.c();
                            let new_value = old_value + 1;
                            self.print_current_code(
                                "INC C".to_string(),
                                format!("C₀ = ${:02x}, C₁ = ${:02x}", old_value, new_value)
                                    .to_string(),
                            );
                            self.set_c(new_value);
                            self.set_z_flag(new_value == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(old_value > new_value);
                        }
                        0x14 => {
                            let old_value = self.d();
                            let new_value = old_value + 1;
                            self.print_current_code(
                                "INC D".to_string(),
                                format!("D₀ = ${:02x}, D₁ = ${:02x}", old_value, new_value)
                                    .to_string(),
                            );
                            self.set_d(new_value);
                            self.set_z_flag(new_value == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(old_value > new_value);
                        }
                        0x1C => {
                            let old_value = self.e();
                            let new_value = old_value + 1;
                            self.print_current_code(
                                "INC E".to_string(),
                                format!("E₀ = ${:02x}, E₁ = ${:02x}", old_value, new_value)
                                    .to_string(),
                            );
                            self.set_e(new_value);
                            self.set_z_flag(new_value == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(old_value > new_value);
                        }
                        0x24 => {
                            let old_value = self.h();
                            let new_value = old_value + 1;
                            self.print_current_code(
                                "INC H".to_string(),
                                format!("H₀ = ${:02x}, H₁ = ${:02x}", old_value, new_value)
                                    .to_string(),
                            );
                            self.set_h(new_value);
                            self.set_z_flag(new_value == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(old_value > new_value);
                        }
                        0x2C => {
                            let old_value = self.l();
                            let new_value = old_value + 1;
                            self.print_current_code(
                                "INC L".to_string(),
                                format!("L₀ = ${:02x}, L₁ = ${:02x}", old_value, new_value)
                                    .to_string(),
                            );
                            self.set_l(new_value);
                            self.set_z_flag(new_value == 0);
                            self.set_n_flag(false);
                            self.set_h_flag(old_value > new_value);
                        }

                        _ => {
                            self.print_current_code(
                                format!("; ERROR: unsupported opcode"),
                                format!(""),
                            );
                            panic!("unsupported opcode: ${:02x}", opcode);
                        }
                    }

                    self.t += 1;
                }
            }
        }
    }

    // Register Access

    fn a(&self) -> u8 {
        return self.main_registers[0];
    }

    fn set_a(&mut self, value: u8) {
        self.main_registers[0] = value;
    }

    fn flags(&self) -> u8 {
        return self.main_registers[1];
    }

    fn set_flags(&mut self, value: u8) {
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

    fn z_flag(&self) -> bool {
        u8_get_bit(self.flags(), 1)
    }

    fn set_z_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 1, value);
        self.set_flags(flags);
    }

    fn n_flag(&self) -> bool {
        u8_get_bit(self.flags(), 2)
    }

    fn set_n_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 2, value);
        self.set_flags(flags);
    }

    fn h_flag(&self) -> bool {
        u8_get_bit(self.flags(), 3)
    }

    fn set_h_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 3, value);
        self.set_flags(flags);
    }

    fn c_flag(&self) -> bool {
        u8_get_bit(self.flags(), 4)
    }

    fn set_c_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 4, value);
        self.set_flags(flags);
    }

    // Memory Access

    fn get_memory(&self, address: u16) -> u8 {
        if address <= 0x00FF {
            return self.boot_rom[address as usize];
        } else if 0x8000 <= address && address <= 0x9FFF {
            let i: usize = (address - 0x8000) as usize;
            return self.video_ram[i];
        } else if 0xFF80 <= address && address <= 0xFFFE {
            let i: usize = (address - 0xFF80) as usize;
            return self.high_ram[i];
        } else {
            panic!("I don't know how to get memory address ${:04x}.", address);
        }
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        {
            let mut frame_buffer = self.frame_buffer.lock().unwrap();
            let i = (address as usize) % frame_buffer.len();
            frame_buffer[i] = value;
        }

        if 0x8000 <= address && address <= 0x9FFF {
            let i: usize = (address - 0x8000) as usize;
            self.video_ram[i] = value;
            println!("  ; video_ram[${:04x}] = ${:02x}", i, value);
        } else if 0xFF80 <= address && address <= 0xFFFE {
            let i: usize = (address - 0xFF80) as usize;
            self.high_ram[i] = value;
            println!("  ; high_ram[${:04x}] = ${:02x}", i, value);
        } else if 0xFF10 <= address && address <= 0xFF26 {
            println!("  ; skipping write to sound control memory -- not implemented");
        } else if address == 0xFF47 {
            self.bg_palette = value;
            println!("  ; updated background palette");
        } else {
            panic!("I don't know how to set memory address ${:04x}.", address);
        }
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
        "Pokemon Red (US)[:256]" => {
            return vec![
                0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0,
                0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0,
                0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0xC3, 0x24, 0x20, 0, 0, 0, 0,
                0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0xC3, 0x06, 0x23, 0, 0, 0, 0, 0, 0xC3, 0x25, 0x21, 0,
                0, 0, 0, 0, 0xD9, 0xAF, 0xE0, 0x0F, 0xF0, 0xFF, 0x47, 0xCB, 0x87, 0xE0, 0xFF, 0xF0,
                0x44, 0xFE, 0x91, 0x20, 0xFA, 0xF0, 0x40, 0xE6, 0x7F, 0xE0, 0x40, 0x78, 0xE0, 0xFF,
                0xC9, 0xF0, 0x40, 0xCB, 0xFF, 0xE0, 0x40, 0xC9, 0xAF, 0x21, 0, 0xC3, 0x06, 0xA0,
                0x22, 0x05, 0x20, 0xFC, 0xC9, 0x3E, 0xA0, 0x21, 0, 0xC3, 0x11, 0x04, 0, 0x06, 0x28,
                0x77, 0x19, 0x05, 0x20, 0xFB, 0xC9, 0xEA, 0xE9, 0xCE, 0xF0, 0xB8, 0xF5, 0xFA, 0xE9,
                0xCE, 0xE0, 0xB8, 0xEA, 0, 0x20, 0xCD, 0xB5, 0, 0xF1, 0xE0, 0xB8, 0xEA, 0, 0x20,
                0xC9, 0x2A, 0x12, 0x13, 0x0B, 0x79, 0xB0, 0x20, 0xF8, 0xC9, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0,
            ];
        }

        _ => panic!("Game ROM Not Available: {}", game_name),
    }
}
