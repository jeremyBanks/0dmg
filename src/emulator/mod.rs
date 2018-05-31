mod audio;
mod cpu;
mod memory;
mod roms;
mod video;

use self::audio::AudioController;
use self::memory::MemoryController;
use self::video::VideoController;

pub struct GameBoy {
    cpu: cpu::CPU,
    mem: MemoryController,
    vid: VideoController,
    aud: AudioController,
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy {
            cpu: cpu::CPU::new(),
            mem: MemoryController::new(roms::GAME_STUB),
            vid: VideoController::new(),
            aud: AudioController::new(),
        }
    }

    pub fn run(&mut self) -> ! {
        println!("; assembly:                        addr:   t/μs:    codes:      flags:");
        println!("; ---------                        -----   -----    ------      ------");

        loop {
            self.cpu.tick(&mut self.mem, &mut self.vid, &mut self.aud);
        }
    }
}

/*

    // LD B, *
    // LD C, *
    
    // LD D, *
    |_50, cpu, _mem| {
        1,
        let d0 = cpu.d;
        let b = cpu.b;
        cpu.set_d(b);
        op_execution!{
            cycles: 000000000;
            asm: "LD D, B"
            debug: "D₀ = ${:02x}, B = ${:02x}", d0, b,
        }
    },
    |_51, cpu, _mem| {
        1,
        let d0 = cpu.d;
        let c = cpu.c();
        cpu.set_d(c);
        op_execution!{
            cycles: 000000000;
            asm: "LD D, C"
            debug: "D₀ = ${:02x}, C = ${:02x}", d0, c,
        }
    },
    |_52, cpu, _mem| {
        1,
        let d = cpu.d;
        (format!("LD D, D"), format!("D = ${:02x}", d)}
    },
    |_53, cpu, _mem| {
        1,
        let d0 = cpu.d;
        let e = cpu.e;
        cpu.set_d(e);
        op_execution!{
            cycles: 000000000;
            asm: "LD D, E"
            debug: "D₀ = ${:02x}, E = ${:02x}", d0, e,
        }
    },
    |_54, cpu, _mem| {
        1,
        let d0 = cpu.d;
        let h = cpu.h();
        cpu.set_d(h);
        op_execution!{
            cycles: 000000000;
            asm: "LD D, H"
            debug: "D₀ = ${:02x}, H = ${:02x}", d0, h,
        }
    },
    |_55, cpu, _mem| {
        1,
        let d0 = cpu.d;
        let l = cpu.l;
        cpu.set_d(l);
        op_execution!{
            cycles: 000000000;
            asm: "LD D, L"
            debug: "D₀ = ${:02x}, L = ${:02x}", d0, l,
        }
    },
    |_56, cpu, _mem| {
        2,
        let d0 = cpu.d;
        let hl = cpu.hl();
        let d1 = mem.get(hl);
        cpu.set_d(d1);
        op_execution!{
            cycles: 000000000;
            asm: "LD D, (HL)"
            debug: "D₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", d0, hl, d1,
        }
    },
    // LD E, *
    |_58, cpu, _mem| {
        1,
        let e0 = cpu.e;
        let b = cpu.b;
        cpu.set_e(b);
        op_execution!{
            cycles: 000000000;
            asm: "LD E, B"
            debug: "E₀ = ${:02x}, B = ${:02x}", e0, b,
        }
    },
    |_59, cpu, _mem| {
        1,
        let e0 = cpu.e;
        let c = cpu.c();
        cpu.set_e(c);
        op_execution!{
            cycles: 000000000;
            asm: "LD E, C"
            debug: "E₀ = ${:02x}, C = ${:02x}", e0, c,
        }
    },
    |_5a, cpu, _mem| {
        1,
        let e0 = cpu.e;
        let d = cpu.d;
        cpu.set_e(d);
        op_execution!{
            cycles: 000000000;
            asm: "LD E, D"
            debug: "E₀ = ${:02x}, D = ${:02x}", e0, d,
        }
    },
    |_5b, cpu, _mem| {
        1,
        let e = cpu.e;
        (format!("LD E, E"), format!("E = ${:02x}", e)}
    },
    |_5c, cpu, _mem| {
        1,
        let e0 = cpu.e;
        let h = cpu.h();
        cpu.set_e(h);
        op_execution!{
            cycles: 000000000;
            asm: "LD E, H"
            debug: "E₀ = ${:02x}, H = ${:02x}", e0, h,
        }
    },
    |_5d, cpu, _mem| {
        1,
        let e0 = cpu.e;
        let l = cpu.l;
        cpu.set_e(l);
        op_execution!{
            cycles: 000000000;
            asm: "LD E, L"
            debug: "E₀ = ${:02x}, L = ${:02x}", e0, l,
        }
    },
    |_5e, cpu, _mem| {
        2,
        let e0 = cpu.e;
        let hl = cpu.hl();
        let e1 = mem.get(hl);
        cpu.set_e(e1);
        op_execution!{
            cycles: 000000000;
            asm: "LD E, (HL)"
            debug: "E₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", e0, hl, e1,
        }
    },
    // LD H, *
    |_60, cpu, _mem| {
        1,
        let h0 = cpu.h();
        let b = cpu.b;
        cpu.set_h(b);
        op_execution!{
            cycles: 000000000;
            asm: "LD H, B"
            debug: "H₀ = ${:02x}, B = ${:02x}", h0, b,
        }
    },
    |_61, cpu, _mem| {
        1,
        let h0 = cpu.h();
        let c = cpu.c();
        cpu.set_h(c);
        op_execution!{
            cycles: 000000000;
            asm: "LD H, C"
            debug: "H₀ = ${:02x}, C = ${:02x}", h0, c,
        }
    },
    |_62, cpu, _mem| {
        1,
        let h0 = cpu.h();
        let d = cpu.d;
        cpu.set_h(d);
        op_execution!{
            cycles: 000000000;
            asm: "LD H, D"
            debug: "H₀ = ${:02x}, D = ${:02x}", h0, d,
        }
    },
    |_63, cpu, _mem| {
        1,
        let h0 = cpu.h();
        let e = cpu.e;
        cpu.set_h(e);
        op_execution!{
            cycles: 000000000;
            asm: "LD H, E"
            debug: "H₀ = ${:02x}, E = ${:02x}", h0, e,
        }
    },
    |_64, cpu, _mem| {
        1,
        let h = cpu.h();
        (format!("LD H, H"), format!("H = ${:02x}", h)}
    },
    |_65, cpu, _mem| {
        1,
        let h0 = cpu.h();
        let l = cpu.l;
        cpu.set_h(l);
        op_execution!{
            cycles: 000000000;
            asm: "LD H, L"
            debug: "H₀ = ${:02x}, L = ${:02x}", h0, l,
        }
    },
    |_66, cpu, _mem| {
        2,
        let h0 = cpu.h();
        let hl = cpu.hl();
        let h1 = mem.get(hl);
        cpu.set_h(h1);
        op_execution!{
            cycles: 000000000;
            asm: "LD H, (HL)"
            debug: "H₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", h0, hl, h1,
        }
    },
    // LD L, *
    |_68, cpu, _mem| {
        1,
        let l0 = cpu.l;
        let b = cpu.b;
        cpu.set_l(b);
        op_execution!{
            cycles: 000000000;
            asm: "LD L, B"
            debug: "L₀ = ${:02x}, B = ${:02x}", l0, b,
        }
    },
    |_69, cpu, _mem| {
        1,
        let l0 = cpu.l;
        let c = cpu.c();
        cpu.set_l(c);
        op_execution!{
            cycles: 000000000;
            asm: "LD L, C"
            debug: "L₀ = ${:02x}, C = ${:02x}", l0, c,
        }
    },
    |_6a, cpu, _mem| {
        1,
        let l0 = cpu.l;
        let d = cpu.d;
        cpu.set_l(d);
        op_execution!{
            cycles: 000000000;
            asm: "LD L, D"
            debug: "L₀ = ${:02x}, D = ${:02x}", l0, d,
        }
    },
    |_6b, cpu, _mem| {
        1,
        let l0 = cpu.l;
        let e = cpu.e;
        cpu.set_l(e);
        op_execution!{
            cycles: 000000000;
            asm: "LD L, E"
            debug: "L₀ = ${:02x}, E = ${:02x}", l0, e,
        }
    },
    |_6c, cpu, _mem| {
        1,
        let l0 = cpu.l;
        let h = cpu.h();
        cpu.set_l(h);
        op_execution!{
            cycles: 000000000;
            asm: "LD L, H"
            debug: "L₀ = ${:02x}, H = ${:02x}", l0, h,
        }
    },
    |_6d, cpu, _mem| {
        1,
        let l0 = cpu.l;
        let l = cpu.l;
        cpu.set_l(l);
        op_execution!{
            cycles: 000000000;
            asm: "LD L, L"
            debug: "L₀ = ${:02x}, L = ${:02x}", l0, l,
        }
    },
    |_6e, cpu, _mem| {
        2,
        let l0 = cpu.l;
        let hl = cpu.hl();
        let l1 = mem.get(hl);
        cpu.set_l(l1);
        op_execution!{
            cycles: 000000000;
            asm: "LD L, (HL)"
            debug: "L₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", l0, hl, l1,
        }
    },
    // LD (HL), *
    |_70, cpu, _mem| {
        2,
        let hl = cpu.hl();
        let b = cpu.b;
        cpu.set_memory(hl, b);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), B"
            debug: "HL = ${:02x}, B = ${:02x}", hl, b,
        }
    },
    |_71, cpu, _mem| {
        2,
        let hl = cpu.hl();
        let c = cpu.c();
        cpu.set_memory(hl, c);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), C"
            debug: "HL = ${:02x}, C = ${:02x}", hl, c,
        }
    },
    |_72, cpu, _mem| {
        2,
        let hl = cpu.hl();
        let d = cpu.d;
        cpu.set_memory(hl, d);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), D"
            debug: "HL = ${:02x}, D = ${:02x}", hl, d,
        }
    },
    |_73, cpu, _mem| {
        2,
        let hl = cpu.hl();
        let e = cpu.e;
        cpu.set_memory(hl, e);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), E"
            debug: "HL = ${:02x}, E = ${:02x}", hl, e,
        }
    },
    |_74, cpu, _mem| {
        2,
        let hl = cpu.hl();
        let h = cpu.h();
        cpu.set_memory(hl, h);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), H"
            debug: "HL = ${:02x}, H = ${:02x}", hl, h,
        }
    },
    |_75, cpu, _mem| {
        2,
        let hl = cpu.hl();
        let l = cpu.l;
        cpu.set_memory(hl, l);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), L"
            debug: "HL = ${:02x}, L = ${:02x}", hl, l,
        }
    },
    |_36, cpu, _mem| {
        3,
        let hl = cpu.hl();
        let n = cpu.read_immediate_u8(mem);
        cpu.set_memory(hl, n);
        (format!("LD (HL), ${:02x}", n), format!("HL = ${:02x}", hl)}
    },
}

// 4. LD n, A
// Put value A into n.
{
    |_47, cpu, _mem| {
        1,
        let b0 = cpu.b;
        let a = cpu.a;
        cpu.b = (a);
        op_execution!{
            cycles: 000000000;
            asm: "LD B, A"
            debug: "B₀ = ${:02x}, A = ${:02x}", b0, a,
        }
    },
    |_4f, cpu, _mem| {
        1,
        let c0 = cpu.c();
        let a = cpu.a;
        cpu.b = (a);
        op_execution!{
            cycles: 000000000;
            asm: "LD C, A"
            debug: "C₀ = ${:02x}, A = ${:02x}", c0, a,
        }
    },
    |_57, cpu, _mem| {
        1,
        let d0 = cpu.d;
        let a = cpu.a;
        cpu.set_d(a);
        op_execution!{
            cycles: 000000000;
            asm: "LD D, A"
            debug: "D₀ = ${:02x}, A = ${:02x}", d0, a,
        }
    },
    |_5f, cpu, _mem| {
        1,
        let e0 = cpu.e;
        let a = cpu.a;
        cpu.set_e(a);
        op_execution!{
            cycles: 000000000;
            asm: "LD E, A"
            debug: "E₀ = ${:02x}, A = ${:02x}", e0, a,
        }
    },
    |_67, cpu, _mem| {
        1,
        let h0 = cpu.h();
        let a = cpu.a;
        cpu.set_h(a);
        op_execution!{
            cycles: 000000000;
            asm: "LD H, A"
            debug: "H₀ = ${:02x}, A = ${:02x}", h0, a,
        }
    },
    |_6f, cpu, _mem| {
        1,
        let l0 = cpu.l;
        let a = cpu.a;
        cpu.set_l(a);
        op_execution!{
            cycles: 000000000;
            asm: "LD L, A"
            debug: "L₀ = ${:02x}, A = ${:02x}", l0, a,
        }
    },
    |_02, cpu, _mem| {
        2,
        let bc = cpu.bc();
        let a = cpu.a;
        cpu.set_memory(bc, a);
        op_execution!{
            cycles: 000000000;
            asm: "LD (BC), A"
            debug: "BC = ${:04x}, A = ${:02x}", bc, cpu.a,
        }
    },
    |_12, cpu, _mem| {
        2,
        let de = cpu.de();
        let a = cpu.a;
        cpu.set_memory(de, a);
        op_execution!{
            cycles: 000000000;
            asm: "LD (DE), A"
            debug: "DE = ${:04x}, A = ${:02x}", de, cpu.a,
        }
    },
    |_77, cpu, _mem| {
        2,
        let hl = cpu.hl();
        let a = cpu.a;
        cpu.set_memory(hl, a);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), A"
            debug: "HL = ${:04x}, A = ${:02x}", hl, cpu.a,
        }
    },
}

// 6. LD ($FF00 + C), A
|_e2, cpu, _mem| {
        2,
    let a = cpu.a;
    let address = 0xFF00 + (cpu.c() as u16);
    cpu.set_memory(address, a);
    (
        format!("LD ($FF00 + C), A "),
        format!("A = ${:02x}, C = ${:02x}", cpu.a, cpu.c()),
    )
});

// 12. LD (HL-), A
// Put A into memory address HL.
// Decrement HL.
|_32, cpu, _mem| {
        2,
    let hl0 = cpu.hl();
    let hl1 = hl0 - 1;
    let a = cpu.a;
    cpu.set_memory(hl0, a);
    cpu.set_hl(hl1);
    (
        format!("LD (HL-), A"),
        format!("HL₀ = ${:04x}, A = ${:02x}", hl0, a),
    )
});

// 19. LDH (n), A
|_e0, cpu, _mem| {
        3,
    let a = cpu.a;
    let n = cpu.read_immediate_u8(mem);
    cpu.set_memory(0xFF00 as u16 + n as u16, a);
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
|_01, cpu, _mem| {
        3,
    let bc0 = cpu.bc();
    let bc1 = cpu.read_immediate_u16(mem);
    cpu.set_bc(bc1);
    (
        format!("LOAD BC, ${:04x}", bc1),
        format!("BC₁ = ${:04x}", bc0),
    )
});
|_11, cpu, _mem| {
        3,
    let de0 = cpu.de();
    let de1 = cpu.read_immediate_u16(mem);
    cpu.set_de(de1);
    (
        format!("LOAD DE, ${:04x}", de1),
        format!("DE₁ = ${:04x}", de0),
    )
});
|_21, cpu, _mem| {
        3,
    let hl0 = cpu.hl();
    let hl1 = cpu.read_immediate_u16(mem);
    cpu.set_hl(hl1);
    (
        format!("LOAD HL, ${:04x}", hl1),
        format!("hl₁ = ${:04x}", hl0),
    )
});


// 2. LD SP, HL
// Put HL into Stack Pointer (SP).
|_f9, cpu, _mem| {
        2,
    let sp0 = cpu.sp();
    let hl = cpu.hl();
    cpu.set_sp(hl);
    (
        format!("LOAD SP, HL"),
        format!("SP₀ = ${:04x}, HL = ${:02x}", sp0, hl),
    )
});

// 5. PUSH nn
// Push register pair nn onto stack.
// Decrement Stack Pointer (SP) twice.
|_f5, cpu, _mem| {
        4,
    let af = cpu.af();
    cpu.stack_push(af);
    (
        format!("PUSH AF"),
        format!("SP₁ = ${:04x}, AF = ${:04x}", cpu.sp(), af),
    )
});
|_c5, cpu, _mem| {
        4,
    let bc = cpu.bc();
    cpu.stack_push(bc);
    (
        format!("PUSH BC"),
        format!("SP₁ = ${:04x}, BC = ${:04x}", cpu.sp(), bc),
    )
});
|_d5, cpu, _mem| {
        4,
    let de = cpu.de();
    cpu.stack_push(de);
    (
        format!("PUSH DE"),
        format!("SP₁ = ${:04x}, DE = ${:04x}", cpu.sp(), de),
    )
});
|_e5, cpu, _mem| {
        4,
    let hl = cpu.hl();
    cpu.stack_push(hl);
    (
        format!("PUSH hl"),
        format!("SP₁ = ${:04x}, HL = ${:04x}", cpu.sp(), hl),
    )
});

// 7. POP nn
// Push two bytes off stack into register pair nn.
// Increment Stack Pointer (SP) twice.
|_f1, cpu, _mem| {
        3,
    let af0 = cpu.af();
    let af1 = cpu.stack_pop();
    (
        format!("POP AF"),
        format!(
            "SP₁ = ${:04x}, AF₀ = ${:04x}, AF₁ = ${:04x}",
            cpu.sp(),
            af0,
            af1
        ),
    )
});
|_c1, cpu, _mem| {
        3,
    let bc0 = cpu.bc();
    let bc1 = cpu.stack_pop();
    (
        format!("POP BC"),
        format!(
            "SP₁ = ${:04x}, BC₀ = ${:04x}, BC₁ = ${:04x}",
            cpu.sp(),
            bc0,
            bc1
        ),
    )
});
|_d1, cpu, _mem| {
        3,
    let de0 = cpu.de();
    let de1 = cpu.stack_pop();
    (
        format!("POP DE"),
        format!(
            "SP₁ = ${:04x}, DE₀ = ${:04x}, DE₁ = ${:04x}",
            cpu.sp(),
            de0,
            de1
        ),
    )
});
|_e1, cpu, _mem| {
        3,
    let hl0 = cpu.hl();
    let hl1 = cpu.stack_pop();
    (
        format!("POP HL"),
        format!(
            "SP₁ = ${:04x}, HL₀ = ${:04x}, HL₁ = ${:04x}",
            cpu.sp(),
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
    |_a8, cpu, _mem| {
        1,
        let b = cpu.b;
        let a0 = cpu.a;
        let a1 = a0 ^ b;
        cpu.a = (a1);
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 000000000;
            asm: "XOR B"
            debug: "A₀ = ${:02x}, B = ${:02x} A₁ = ${:02x}", a0, b, a1,
        }
    },
    |_a9, cpu, _mem| {
        1,
        let c = cpu.c();
        let a0 = cpu.a;
        let a1 = a0 ^ c;
        cpu.a = (a1);
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 000000000;
            asm: "XOR C"
            debug: "A₀ = ${:02x}, C = ${:02x} A₁ = ${:02x}", a0, c, a1,
        }
    },
    |_aa, cpu, _mem| {
        1,
        let d = cpu.d;
        let a0 = cpu.a;
        let a1 = a0 ^ d;
        cpu.a = (a1);
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 000000000;
            asm: "XOR D"
            debug: "A₀ = ${:02x}, D = ${:02x} A₁ = ${:02x}", a0, d, a1,
        }
    },
    |_ab, cpu, _mem| {
        1,
        let e = cpu.e;
        let a0 = cpu.a;
        let a1 = a0 ^ e;
        cpu.a = (a1);
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 000000000;
            asm: "XOR E"
            debug: "A₀ = ${:02x}, E = ${:02x} A₁ = ${:02x}", a0, e, a1,
        }
    },
    |_ac, cpu, _mem| {
        1,
        let h = cpu.h();
        let a0 = cpu.a;
        let a1 = a0 ^ h;
        cpu.a = (a1);
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 000000000;
            asm: "XOR B"
            debug: "A₀ = ${:02x}, H = ${:02x} A₁ = ${:02x}", a0, h, a1,
        }
    },
    |_ad, cpu, _mem| {
        1,
        let l = cpu.l;
        let a0 = cpu.a;
        let a1 = a0 ^ l;
        cpu.a = (a1);
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 000000000;
            asm: "XOR L"
            debug: "A₀ = ${:02x}, L = ${:02x} A₁ = ${:02x}", a0, l, a1,
        }
    },
}

// 9. INC n
{
    |_3c, cpu, _mem| {
        1,
        let a0 = cpu.a;
        let a1 = a0 + 1;
        cpu.a = (a1);
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(a0 > a1);
        op_execution!{
            cycles: 000000000;
            asm: "INC A"
            debug: "A₀ = ${:02x}, A₁ = ${:02x}", a0, a1,
        }
    },
    |_04, cpu, _mem| {
        1,
        let b0 = cpu.b;
        let b1 = b0 + 1;
        cpu.b = (b1);
        cpu.set_z_flag(b1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(b0 > b1);
        op_execution!{
            cycles: 000000000;
            asm: "INC B"
            debug: "B₀ = ${:02x}, B₁ = ${:02x}", b0, b1,
        }
    },
    |_0c, cpu, _mem| {
        1,
        let c0 = cpu.c();
        let c1 = c0 + 1;
        cpu.set_c(c1);
        cpu.set_z_flag(c1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(c0 > c1);
        op_execution!{
            cycles: 000000000;
            asm: "INC C"
            debug: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1,
        }
    },
    |_14, cpu, _mem| {
        1,
        let d0 = cpu.d;
        let d1 = d0 + 1;
        cpu.set_d(d1);
        cpu.set_z_flag(d1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(d0 > d1);
        op_execution!{
            cycles: 000000000;
            asm: "INC D"
            debug: "D₀ = ${:02x}, D₁ = ${:02x}", d0, d1,
        }
    },
    |_1c, cpu, _mem| {
        1,
        let e0 = cpu.e;
        let e1 = e0 + 1;
        cpu.set_e(e1);
        cpu.set_z_flag(e1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(e0 > e1);
        op_execution!{
            cycles: 000000000;
            asm: "INC E"
            debug: "E₀ = ${:02x}, E₁ = ${:02x}", e0, e1,
        }
    },
    |_24, cpu, _mem| {
        1,
        let h0 = cpu.h();
        let h1 = h0 + 1;
        cpu.set_h(h1);
        cpu.set_z_flag(h1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(h0 > h1);
        op_execution!{
            cycles: 000000000;
            asm: "INC H"
            debug: "H₀ = ${:02x}, H₁ = ${:02x}", h0, h1,
        }
    },
    |_2c, cpu, _mem| {
        1,
        let l0 = cpu.l;
        let l1 = l0 + 1;
        cpu.set_l(l1);
        cpu.set_z_flag(l1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(l0 > l1);
        op_execution!{
            cycles: 000000000;
            asm: "INC L"
            debug: "L₀ = ${:02x}, L₁ = ${:02x}", l0, l1,
        }
    },
}

// 10. DEC n
{
    |_3d, cpu, _mem| {
        1,
        let a0 = cpu.a;
        let a1 = a0 - 1;
        cpu.a = (a1);
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(a0 < a1);
        op_execution!{
            cycles: 000000000;
            asm: "DEC A"
            debug: "A₀ = ${:02x}, A₁ = ${:02x}", a0, a1,
        }
    },
    |_05, cpu, _mem| {
        1,
        let b0 = cpu.b;
        let b1 = b0 - 1;
        cpu.b = (b1);
        cpu.set_z_flag(b1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(b0 < b1);
        op_execution!{
            cycles: 000000000;
            asm: "DEC A"
            debug: "A₀ = ${:02x}, A₁ = ${:02x}", b0, b1,
        }
    },
    |_0d, cpu, _mem| {
        1,
        let c0 = cpu.a;
        let c1 = c0 - 1;
        cpu.set_c(c1);
        cpu.set_z_flag(c1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(c0 < c1);
        op_execution!{
            cycles: 000000000;
            asm: "DEC C"
            debug: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1,
        }
    },
    |_15, cpu, _mem| {
        1,
        let d0 = cpu.d;
        let d1 = d0 - 1;
        cpu.set_d(d1);
        cpu.set_z_flag(d1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(d0 < d1);
        op_execution!{
            cycles: 000000000;
            asm: "DEC D"
            debug: "D₀ = ${:02x}, D₁ = ${:02x}", d0, d1,
        }
    },
    |_1d, cpu, _mem| {
        1,
        let e0 = cpu.e;
        let e1 = e0 - 1;
        cpu.set_e(e1);
        cpu.set_z_flag(e1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(e0 < e1);
        op_execution!{
            cycles: 000000000;
            asm: "DEC A"
            debug: "E₀ = ${:02x}, E₁ = ${:02x}", e0, e1,
        }
    },
    |_25, cpu, _mem| {
        1,
        let h0 = cpu.h();
        let h1 = h0 - 1;
        cpu.set_h(h1);
        cpu.set_z_flag(h1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(h0 < h1);
        op_execution!{
            cycles: 000000000;
            asm: "DEC H"
            debug: "H₀ = ${:02x}, H₁ = ${:02x}", h0, h1,
        }
    },
    |_2d, cpu, _mem| {
        1,
        let l0 = cpu.l;
        let l1 = 0 - 1;
        cpu.set_l(l1);
        cpu.set_z_flag(l1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(0 < l1);
        op_execution!{
            cycles: 000000000;
            asm: "DEC L"
            debug: "A₀ = ${:02x}, A₁ = ${:02x}", 0, l1,
        }
    },
}
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
|_17, cpu, _mem| {
        2,
    let a0 = cpu.a;
    let a1 = (a0 << 1) + if cpu.c_flag() { 1 } else { 0 };
    cpu.a = (a1);
    cpu.set_z_flag(a1 == 0);
    cpu.set_c_flag(a0 & 0b10000000 > 0);
    cpu.set_n_flag(false);
    cpu.set_h_flag(false);
    (format!("RLA"), format!("A₀ = {}", a0))
});

// 6. RL n
// Rotate n left through Carry flag.
op_|_17, cpu, _mem| {
        2,
    let a0 = cpu.a;
    let a1 = a0 << 1 + if cpu.c_flag() { 1 } else { 0 };
    cpu.a = (a1);
    cpu.set_z_flag(a1 == 0);
    cpu.set_c_flag(a0 & 0b10000000 > 0);
    cpu.set_n_flag(false);
    cpu.set_h_flag(false);
    (format!("RL A"), format!("A₀ = {}", a0))
});
op_|_10, cpu, _mem| {
        2,
    let b0 = cpu.b;
    let b1 = b0 << 1 + if cpu.c_flag() { 1 } else { 0 };
    cpu.b = (b1);
    cpu.set_z_flag(b1 == 0);
    cpu.set_c_flag(b0 & 0b10000000 > 0);
    cpu.set_n_flag(false);
    cpu.set_h_flag(false);
    (format!("RL B"), format!("B₀ = {}", b0))
});
op_|_11, cpu, _mem| {
        2,
    let c0 = cpu.c();
    let c1 = c0 << 1 + if cpu.c_flag() { 1 } else { 0 };
    cpu.set_c(c1);
    cpu.set_z_flag(c1 == 0);
    cpu.set_c_flag(c0 & 0b10000000 > 0);
    cpu.set_n_flag(false);
    cpu.set_h_flag(false);
    (format!("RL C"), format!("C₀ = {}", c0))
});
op_|_12, cpu, _mem| {
        2,
    let d0 = cpu.d;
    let d1 = d0 << 1 + if cpu.c_flag() { 1 } else { 0 };
    cpu.set_d(d1);
    cpu.set_z_flag(d1 == 0);
    cpu.set_c_flag(d0 & 0b10000000 > 0);
    cpu.set_n_flag(false);
    cpu.set_h_flag(false);
    (format!("RL D"), format!("D₀ = {}", d0))
});
op_|_13, cpu, _mem| {
        2,
    let e0 = cpu.e;
    let e1 = e0 << 1 + if cpu.c_flag() { 1 } else { 0 };
    cpu.set_e(e1);
    cpu.set_z_flag(e1 == 0);
    cpu.set_c_flag(e0 & 0b10000000 > 0);
    cpu.set_n_flag(false);
    cpu.set_h_flag(false);
    (format!("RL E"), format!("E₀ = {}", e0))
});
op_|_14, cpu, _mem| {
        2,
    let h0 = cpu.h();
    let h1 = h0 << 1 + if cpu.c_flag() { 1 } else { 0 };
    cpu.set_h(h1);
    cpu.set_z_flag(h1 == 0);
    cpu.set_c_flag(h0 & 0b10000000 > 0);
    cpu.set_n_flag(false);
    cpu.set_h_flag(false);
    (format!("RL H"), format!("H₀ = {}", h0))
});
op_|_15, cpu, _mem| {
        2,
    let l0 = cpu.l;
    let l1 = l0 << 1 + if cpu.c_flag() { 1 } else { 0 };
    cpu.set_l(l1);
    cpu.set_z_flag(l1 == 0);
    cpu.set_c_flag(l0 & 0b10000000 > 0);
    cpu.set_n_flag(false);
    cpu.set_h_flag(false);
    (format!("RL L"), format!("L₀ = {}", l0))
});
}

// 3.3.7. Bit Opcodes
{
op_|_7C, cpu, _mem| {
        2,
    let result = !u8_get_bit(cpu.h(), 7);
    cpu.set_z_flag(result);
    cpu.set_n_flag(false);
    cpu.set_h_flag(true);
    (format!("BIT 7, H"), format!("Z₁ = {}", result))
});
}

// 3.3.8. Jumps
{
// 1. JP nn
// Jump to address nn.
|_C3, cpu, _mem| {
        3,
    let nn = cpu.read_immediate_u16(mem);
    cpu.i = nn;
    (format!("JP ${:04x}", nn), format!(""))
});
// 2. JP cc, nn
// Jump to address n if condition is true.
|_C2, cpu, _mem| {
        3,
    let nn = cpu.read_immediate_u16(mem);
    let z_flag = cpu.z_flag();
    if z_flag == false {
        cpu.i = nn;
    }
    (format!("JP NZ, ${:04x}", nn), format!("Z = {}", z_flag))
});
|_CA, cpu, _mem| {
        3,
    let nn = cpu.read_immediate_u16(mem);
    let z_flag = cpu.z_flag();
    if z_flag {
        cpu.i = nn;
    }
    (format!("JP Z, ${:04x}", nn), format!("Z = {}", z_flag))
});
|_D2, cpu, _mem| {
        3,
    let nn = cpu.read_immediate_u16(mem);
    let c_flag = cpu.c_flag();
    if c_flag == false {
        cpu.i = nn;
    }
    (format!("JP NC, ${:04x}", nn), format!("C = {}", c_flag))
});
|_DA, cpu, _mem| {
        3,
    let nn = cpu.read_immediate_u16(mem);
    let c_flag = cpu.c_flag();
    if c_flag {
        cpu.i = nn;
    }
    (format!("JP C, ${:04x}", nn), format!("C = {}", c_flag))
});
// 3. JP (HL)
// Jump to address contained in HL.
|_E9, cpu, _mem| {
        1,
    let hl = cpu.hl();
    (format!("JP (HL)"), format!("HL = ${:04x}", hl))
});
// 4. JR n
// Add n to current address and jump to it.
|_18, cpu, _mem| {
        2,
    let n = cpu.read_immediate_i8();
    (format!("JP {}", n), format!(""))
});
// 5. JR cc, n
// If condition is true then add n to current address and jump to it.
|_20, cpu, _mem| {
        2,
    let n = cpu.read_immediate_i8();
    let z_flag = cpu.z_flag();
    if z_flag == false {
        cpu.relative_jump(n);
    }
    (format!("JR NZ, {}", n), format!("Z = {}", z_flag))
});
|_28, cpu, _mem| {
        2,
    let n = cpu.read_immediate_i8();
    let z_flag = cpu.z_flag();
    if z_flag {
        cpu.relative_jump(n);
    }
    (format!("JR Z, {}", n), format!("Z = {}", z_flag))
});
|_30, cpu, _mem| {
        2,
    let n = cpu.read_immediate_i8();
    let c_flag = cpu.c_flag();
    if c_flag == false {
        cpu.relative_jump(n);
    }
    (format!("JR NC, {}", n), format!("C = {}", c_flag))
});
|_38, cpu, _mem| {
        2,
    let n = cpu.read_immediate_i8();
    let c_flag = cpu.c_flag();
    if c_flag {
        cpu.relative_jump(n);
    }
    (format!("JR C, {}", n), format!("C = {}", c_flag))
});
}

// 3.3.9. Calls
{
// 1. CALL nn
// Push address of next instruction onto stack and
// then jump to address nn.

|_CD, cpu, _mem| {
        3,
    let nn = cpu.read_immediate_u16(mem);
    let i0 = cpu.i;
    cpu.stack_push(i0);
    cpu.i = nn;
    (
        format!("CALL ${:04x}", nn),
        format!("SP₁ = {:04x}", cpu.sp()),
    )
});
}
}

(operations, operations_cb)
}

*/
