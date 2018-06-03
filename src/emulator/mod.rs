mod audio;
mod cpu;
mod memory;
mod roms;
mod video;

use self::audio::AudioData;
use self::cpu::{CPUController, CPUData};
use self::memory::MemoryData;
use self::video::VideoData;

pub struct GameBoy {
    cpu: CPUData,
    mem: MemoryData,
    aud: AudioData,
    vid: VideoData,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: CPUData::new(),
            mem: MemoryData::new(),
            aud: AudioData::new(),
            vid: VideoData::new(),
        }
    }

    pub fn run(&mut self) -> ! {
        println!("; assembly:                        addr:   t/μs:    codes:      flags:");
        println!("; ---------                        -----   -----    ------      ------");

        loop {
            self.tick();
        }
    }
}

/*

    // LD B, *
    // LD C, *
    
    // LD D, *
    |_50, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let d0 = cpu.d;
        let b = cpu.b;
        cpu.d = b;
        op_execution!{
            cycles: 000000000;
            asm: "LD D, B"
            trace: "D₀ = ${:02x}, B = ${:02x}", d0, b,
        }
    },
    |_51, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let d0 = cpu.d;
        let c = cpu.c();
        cpu.d = c;
        op_execution!{
            cycles: 000000000;
            asm: "LD D, C"
            trace: "D₀ = ${:02x}, C = ${:02x}", d0, c,
        }
    },
    |_52, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let d = cpu.d;
        (format!("LD D, D"), format!("D = ${:02x}", d)}
    },
    |_53, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let d0 = cpu.d;
        let e = cpu.e;
        cpu.d = e;
        op_execution!{
            cycles: 000000000;
            asm: "LD D, E"
            trace: "D₀ = ${:02x}, E = ${:02x}", d0, e,
        }
    },
    |_54, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let d0 = cpu.d;
        let h = cpu.h();
        cpu.d = h;
        op_execution!{
            cycles: 000000000;
            asm: "LD D, H"
            trace: "D₀ = ${:02x}, H = ${:02x}", d0, h,
        }
    },
    |_55, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let d0 = cpu.d;
        let l = cpu.l;
        cpu.d = l;
        op_execution!{
            cycles: 000000000;
            asm: "LD D, L"
            trace: "D₀ = ${:02x}, L = ${:02x}", d0, l,
        }
    },
    |_56, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let d0 = cpu.d;
        let hl = cpu.hl();
        let d1 = mem.get(hl);
        cpu.d = d1;
        op_execution!{
            cycles: 000000000;
            asm: "LD D, (HL)"
            trace: "D₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", d0, hl, d1,
        }
    },
    // LD E, *
    |_58, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let e0 = cpu.e;
        let b = cpu.b;
        cpu.e = b;
        op_execution!{
            cycles: 000000000;
            asm: "LD E, B"
            trace: "E₀ = ${:02x}, B = ${:02x}", e0, b,
        }
    },
    |_59, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let e0 = cpu.e;
        let c = cpu.c();
        cpu.e = c;
        op_execution!{
            cycles: 000000000;
            asm: "LD E, C"
            trace: "E₀ = ${:02x}, C = ${:02x}", e0, c,
        }
    },
    |_5a, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let e0 = cpu.e;
        let d = cpu.d;
        cpu.e = d;
        op_execution!{
            cycles: 000000000;
            asm: "LD E, D"
            trace: "E₀ = ${:02x}, D = ${:02x}", e0, d,
        }
    },
    |_5b, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let e = cpu.e;
        (format!("LD E, E"), format!("E = ${:02x}", e)}
    },
    |_5c, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let e0 = cpu.e;
        let h = cpu.h();
        cpu.e = h;
        op_execution!{
            cycles: 000000000;
            asm: "LD E, H"
            trace: "E₀ = ${:02x}, H = ${:02x}", e0, h,
        }
    },
    |_5d, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let e0 = cpu.e;
        let l = cpu.l;
        cpu.e = l;
        op_execution!{
            cycles: 000000000;
            asm: "LD E, L"
            trace: "E₀ = ${:02x}, L = ${:02x}", e0, l,
        }
    },
    |_5e, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let e0 = cpu.e;
        let hl = cpu.hl();
        let e1 = mem.get(hl);
        cpu.e = e1;
        op_execution!{
            cycles: 000000000;
            asm: "LD E, (HL)"
            trace: "E₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", e0, hl, e1,
        }
    },
    // LD H, *
    |_60, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let h0 = cpu.h();
        let b = cpu.b;
        cpu.h = b;
        op_execution!{
            cycles: 000000000;
            asm: "LD H, B"
            trace: "H₀ = ${:02x}, B = ${:02x}", h0, b,
        }
    },
    |_61, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let h0 = cpu.h();
        let c = cpu.c();
        cpu.h = c;
        op_execution!{
            cycles: 000000000;
            asm: "LD H, C"
            trace: "H₀ = ${:02x}, C = ${:02x}", h0, c,
        }
    },
    |_62, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let h0 = cpu.h();
        let d = cpu.d;
        cpu.h = d;
        op_execution!{
            cycles: 000000000;
            asm: "LD H, D"
            trace: "H₀ = ${:02x}, D = ${:02x}", h0, d,
        }
    },
    |_63, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let h0 = cpu.h();
        let e = cpu.e;
        cpu.h = e;
        op_execution!{
            cycles: 000000000;
            asm: "LD H, E"
            trace: "H₀ = ${:02x}, E = ${:02x}", h0, e,
        }
    },
    |_64, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let h = cpu.h();
        (format!("LD H, H"), format!("H = ${:02x}", h)}
    },
    |_65, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let h0 = cpu.h();
        let l = cpu.l;
        cpu.h = l;
        op_execution!{
            cycles: 000000000;
            asm: "LD H, L"
            trace: "H₀ = ${:02x}, L = ${:02x}", h0, l,
        }
    },
    |_66, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let h0 = cpu.h();
        let hl = cpu.hl();
        let h1 = mem.get(hl);
        cpu.h = h1;
        op_execution!{
            cycles: 000000000;
            asm: "LD H, (HL)"
            trace: "H₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", h0, hl, h1,
        }
    },
    // LD L, *
    |_68, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let l0 = cpu.l;
        let b = cpu.b;
        cpu.l = b;
        op_execution!{
            cycles: 000000000;
            asm: "LD L, B"
            trace: "L₀ = ${:02x}, B = ${:02x}", l0, b,
        }
    },
    |_69, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let l0 = cpu.l;
        let c = cpu.c();
        cpu.l = c;
        op_execution!{
            cycles: 000000000;
            asm: "LD L, C"
            trace: "L₀ = ${:02x}, C = ${:02x}", l0, c,
        }
    },
    |_6a, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let l0 = cpu.l;
        let d = cpu.d;
        cpu.l = d;
        op_execution!{
            cycles: 000000000;
            asm: "LD L, D"
            trace: "L₀ = ${:02x}, D = ${:02x}", l0, d,
        }
    },
    |_6b, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let l0 = cpu.l;
        let e = cpu.e;
        cpu.l = e;
        op_execution!{
            cycles: 000000000;
            asm: "LD L, E"
            trace: "L₀ = ${:02x}, E = ${:02x}", l0, e,
        }
    },
    |_6c, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let l0 = cpu.l;
        let h = cpu.h();
        cpu.l = h;
        op_execution!{
            cycles: 000000000;
            asm: "LD L, H"
            trace: "L₀ = ${:02x}, H = ${:02x}", l0, h,
        }
    },
    |_6d, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let l0 = cpu.l;
        let l = cpu.l;
        cpu.l = l;
        op_execution!{
            cycles: 000000000;
            asm: "LD L, L"
            trace: "L₀ = ${:02x}, L = ${:02x}", l0, l,
        }
    },
    |_6e, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let l0 = cpu.l;
        let hl = cpu.hl();
        let l1 = mem.get(hl);
        cpu.l = l1;
        op_execution!{
            cycles: 000000000;
            asm: "LD L, (HL)"
            trace: "L₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", l0, hl, l1,
        }
    },
    // LD (HL), *
    |_70, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let hl = cpu.hl();
        let b = cpu.b;
        mem.set(hl, b);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), B"
            trace: "HL = ${:02x}, B = ${:02x}", hl, b,
        }
    },
    |_71, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let hl = cpu.hl();
        let c = cpu.c();
        mem.set(hl, c);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), C"
            trace: "HL = ${:02x}, C = ${:02x}", hl, c,
        }
    },
    |_72, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let hl = cpu.hl();
        let d = cpu.d;
        mem.set(hl, d);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), D"
            trace: "HL = ${:02x}, D = ${:02x}", hl, d,
        }
    },
    |_73, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let hl = cpu.hl();
        let e = cpu.e;
        mem.set(hl, e);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), E"
            trace: "HL = ${:02x}, E = ${:02x}", hl, e,
        }
    },
    |_74, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let hl = cpu.hl();
        let h = cpu.h();
        mem.set(hl, h);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), H"
            trace: "HL = ${:02x}, H = ${:02x}", hl, h,
        }
    },
    |_75, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let hl = cpu.hl();
        let l = cpu.l;
        mem.set(hl, l);
        op_execution!{
            cycles: 000000000;
            asm: "LD (HL), L"
            trace: "HL = ${:02x}, L = ${:02x}", hl, l,
        }
    },
    |_36, cpu, _mem| {
    op_execution!{
        cycles: 3;
    }
        let hl = cpu.hl();
        let n = cpu.read_immediate_u8(mem);
        mem.set(hl, n);
        (format!("LD (HL), ${:02x}", n), format!("HL = ${:02x}", hl)}
    },
}

// 4. LD n, A
// Put value A into n.
{
    |_47, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let b0 = cpu.b;
        let a = cpu.a;
        cpu.b = (a);
        op_execution!{
            cycles: 000000000;
            asm: "LD B, A"
            trace: "B₀ = ${:02x}, A = ${:02x}", b0, a,
        }
    },
    |_57, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let d0 = cpu.d;
        let a = cpu.a;
        cpu.d = a;
        op_execution!{
            cycles: 000000000;
            asm: "LD D, A"
            trace: "D₀ = ${:02x}, A = ${:02x}", d0, a,
        }
    },
    |_5f, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let e0 = cpu.e;
        let a = cpu.a;
        cpu.e = a;
        op_execution!{
            cycles: 000000000;
            asm: "LD E, A"
            trace: "E₀ = ${:02x}, A = ${:02x}", e0, a,
        }
    },
    |_67, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let h0 = cpu.h();
        let a = cpu.a;
        cpu.h = a;
        op_execution!{
            cycles: 000000000;
            asm: "LD H, A"
            trace: "H₀ = ${:02x}, A = ${:02x}", h0, a,
        }
    },
    |_6f, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let l0 = cpu.l;
        let a = cpu.a;
        cpu.l = a;
        op_execution!{
            cycles: 000000000;
            asm: "LD L, A"
            trace: "L₀ = ${:02x}, A = ${:02x}", l0, a,
        }
    },
    |_02, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let bc = cpu.bc();
        let a = cpu.a;
        mem.set(bc, a);
        op_execution!{
            cycles: 000000000;
            asm: "LD (BC), A"
            trace: "BC = ${:04x}, A = ${:02x}", bc, cpu.a,
        }
    },
    |_12, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
        let de = cpu.de();
        let a = cpu.a;
        mem.set(de, a);
        op_execution!{
            cycles: 000000000;
            asm: "LD (DE), A"
            trace: "DE = ${:04x}, A = ${:02x}", de, cpu.a,
        }
    },
}

// 6. LD ($FF00 + C), A


// 12. LD (HL-), A
// Put A into memory address HL.
// Decrement HL.


// 19. LDH (n), A

}

// 3.3.2 16-Bit Loads
{
// 3.3.2. 16-Bit Loads
// 1. LD n, nn
// Put value nn into n.
|_01, cpu, _mem| {
    op_execution!{
        cycles: 3;
    }
    let bc0 = cpu.bc();
    let bc1 = cpu.read_immediate_u16(mem);
    cpu.set_bc(bc1);
    (
        asm: "LOAD BC, ${:04x}", bc1;
        trace: "BC₁ = ${:04x}", bc0;
    }
},

);


// 2. LD SP, HL
// Put HL into Stack Pointer (SP).
|_f9, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
    let sp0 = cpu.sp();
    let hl = cpu.hl();
    cpu.set_sp(hl);
    (
        asm: "LOAD SP, HL";
        trace: "SP₀ = ${:04x}, HL = ${:02x}", sp0, hl;
    }
},

// 9. INC n
{
    |_3c, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let a0 = cpu.a;
        let a1 = a0 + 1;
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(a0 > a1);
        op_execution!{
            cycles: 000000000;
            asm: "INC A"
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", a0, a1,
        }
    },
    |_04, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let b0 = cpu.b;
        let b1 = b0 + 1;
        cpu.b = (b1);
        cpu.set_z_flag(b1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(b0 > b1);
        op_execution!{
            cycles: 000000000;
            asm: "INC B"
            trace: "B₀ = ${:02x}, B₁ = ${:02x}", b0, b1,
        }
    },
    |_0c, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let c0 = cpu.c();
        let c1 = c0 + 1;
        cpu.c = c1;
        cpu.set_z_flag(c1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(c0 > c1);
        op_execution!{
            cycles: 000000000;
            asm: "INC C"
            trace: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1,
        }
    },
    |_14, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let d0 = cpu.d;
        let d1 = d0 + 1;
        cpu.d = d1;
        cpu.set_z_flag(d1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(d0 > d1);
        op_execution!{
            cycles: 000000000;
            asm: "INC D"
            trace: "D₀ = ${:02x}, D₁ = ${:02x}", d0, d1,
        }
    },
    |_1c, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let e0 = cpu.e;
        let e1 = e0 + 1;
        cpu.e = e1;
        cpu.set_z_flag(e1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(e0 > e1);
        op_execution!{
            cycles: 000000000;
            asm: "INC E"
            trace: "E₀ = ${:02x}, E₁ = ${:02x}", e0, e1,
        }
    },
    |_24, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let h0 = cpu.h();
        let h1 = h0 + 1;
        cpu.h = h1;
        cpu.set_z_flag(h1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(h0 > h1);
        op_execution!{
            cycles: 000000000;
            asm: "INC H"
            trace: "H₀ = ${:02x}, H₁ = ${:02x}", h0, h1,
        }
    },
    |_2c, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
        let l0 = cpu.l;
        let l1 = l0 + 1;
        cpu.l = l1;
        cpu.set_z_flag(l1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(l0 > l1);
        op_execution!{
            cycles: 000000000;
            asm: "INC L"
            trace: "L₀ = ${:02x}, L₁ = ${:02x}", l0, l1,
        }
    },
}

// 10. DEC n
{
    
    
    
    
    
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
        op_execution!{
            cycles: 2;
        }
    let a0 = cpu.a;
    let a1 = (a0 << 1) + if cpu.c_flag() { 1 } else { 0 };
    cpu.a = a1;
    cpu.set_z_flag(a1 == 0);
    cpu.set_c_flag(a0 & 0b10000000 > 0);
    cpu.set_n_flag(false);
    cpu.set_h_flag(false);
    (format!("RLA"), format!("A₀ = {}", a0))
},

// 6. RL n
// Rotate n left through Carry flag.

}

// 3.3.7. Bit Opcodes
{
op_|);
}

// 3.3.8. Jumps
{
// 1. JP nn
// Jump to address nn.
|_c3, cpu, _mem| {
    op_execution!{
        cycles: 3;
    }
    let nn = cpu.read_immediate_u16(mem);
    cpu.i = nn;
    (format!("JP ${:04x}", nn), format!(""))
},
// 2. JP cc, nn
// Jump to address n if condition is true.
|_c2, cpu, _mem| {
    op_execution!{
        cycles: 3;
    }
    let nn = cpu.read_immediate_u16(mem);
    let z_flag = cpu.z_flag();
    if z_flag == false {
        cpu.i = nn;
    }
    (format!("JP NZ, ${:04x}", nn), format!("Z = {}", z_flag))
},
|_ca, cpu, _mem| {
    op_execution!{
        cycles: 3;
    }
    let nn = cpu.read_immediate_u16(mem);
    let z_flag = cpu.z_flag();
    if z_flag {
        cpu.i = nn;
    }
    (format!("JP Z, ${:04x}", nn), format!("Z = {}", z_flag))
},
|_d2, cpu, _mem| {
    op_execution!{
        cycles: 3;
    }
    let nn = cpu.read_immediate_u16(mem);
    let c_flag = cpu.c_flag();
    if c_flag == false {
        cpu.i = nn;
    }
    (format!("JP NC, ${:04x}", nn), format!("C = {}", c_flag))
},
|_da, cpu, _mem| {
    op_execution!{
        cycles: 3;
    }
    let nn = cpu.read_immediate_u16(mem);
    let c_flag = cpu.c_flag();
    if c_flag {
        cpu.i = nn;
    }
    (format!("JP C, ${:04x}", nn), format!("C = {}", c_flag))
},
// 3. JP (HL)
// Jump to address contained in HL.
|_e9, cpu, _mem| {
        op_execution!{
            cycles: 1;
        }
    let hl = cpu.hl();
    (format!("JP (HL)"), format!("HL = ${:04x}", hl))
},
// 4. JR n
// Add n to current address and jump to it.
|_18, cpu, _mem| {
        op_execution!{
            cycles: 2;
        }
    let n = cpu.read_immediate_i8();
    (format!("JP {}", n), format!(""))
},
// 5. JR cc, n
// If condition is true then add n to current address and jump to it.
}

// 3.3.9. Calls
{
// 1. CALL nn
// Push address of next instruction onto stack and
// then jump to address nn.


}
}

(operations, operations_cb)
}

*/
