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
        println!("; assembly:                        addr:   t/μs:   codes:       flags:");
        println!("; ---------                        -----   -----   ------       ------");

        loop {
            self.cpu.tick(&mut self.mem, &mut self.vid, &mut self.aud);
        }
    }
}

//         // 3.1.1. 8-bit Loads
//         {
//             // 1. LD nn, n
//             // Put value n into nn.
//             {
//                 op(0x06, 2, |gb| {
//                     let b0 = gb.b();
//                     let b1 = gb.read_immediate_u8();
//                     gb.set_b(b1);
//                     (
//                         format!("LD B, ${:02x}", b1),
//                         format!("B₀ = ${:02x}, B₁ = ${:02x}", b0, b1),
//                     )
//                 });
//                 op(0x0E, 2, |gb| {
//                     let c0 = gb.c();
//                     let c1 = gb.read_immediate_u8();
//                     gb.set_c(c1);
//                     (
//                         format!("LD C, ${:02x}", c1),
//                         format!("C₀ = ${:02x}, C₁ = ${:02x}", c0, c1),
//                     )
//                 });
//                 op(0x16, 2, |gb| {
//                     let d0 = gb.d();
//                     let d1 = gb.read_immediate_u8();
//                     gb.set_d(d1);
//                     (
//                         format!("LD D, ${:02x}", d1),
//                         format!("D₀ = ${:02x}, D₁ = ${:02x}", d0, d1),
//                     )
//                 });
//                 op(0x1E, 2, |gb| {
//                     let e0 = gb.e();
//                     let e1 = gb.read_immediate_u8();
//                     gb.set_e(e1);
//                     (
//                         format!("LD E, ${:02x}", e1),
//                         format!("E₀ = ${:02x}, E₁ = ${:02x}", e0, e1),
//                     )
//                 });
//                 op(0x26, 2, |gb| {
//                     let h0 = gb.h();
//                     let h1 = gb.read_immediate_u8();
//                     gb.set_h(h1);
//                     (
//                         format!("LD H, ${:02x}", h1),
//                         format!("H₀ = ${:02x}, H₁ = ${:02x}", h0, h1),
//                     )
//                 });
//                 op(0x2E, 2, |gb| {
//                     let l0 = gb.l();
//                     let l1 = gb.read_immediate_u8();
//                     gb.set_l(l1);
//                     (
//                         format!("LD L, ${:02x}", l1),
//                         format!("L₀ = ${:02x}, L₁ = ${:02x}", l0, l1),
//                     )
//                 });
//             }

//             // 2. LD r1, r2
//             // Put value r2 into r1.
//             // 3. LD A, n
//             // Put value n into A.
//             {
//                 // LD A, *
//                 op(0x7F, 1, |gb| {
//                     let a = gb.a();
//                     (format!("LD A, A"), format!("A = ${:02x}", a))
//                 });
//                 op(0x78, 1, |gb| {
//                     let a0 = gb.a();
//                     let b = gb.b();
//                     gb.set_a(b);
//                     (
//                         format!("LD A, B"),
//                         format!("A₀ = ${:02x}, B = ${:02x}", a0, b),
//                     )
//                 });
//                 op(0x79, 1, |gb| {
//                     let a0 = gb.a();
//                     let c = gb.c();
//                     gb.set_a(c);
//                     (
//                         format!("LD A, C"),
//                         format!("A₀ = ${:02x}, C = ${:02x}", a0, c),
//                     )
//                 });
//                 op(0x7A, 1, |gb| {
//                     let a0 = gb.a();
//                     let d = gb.d();
//                     gb.set_a(d);
//                     (
//                         format!("LD A, D"),
//                         format!("A₀ = ${:02x}, D = ${:02x}", a0, d),
//                     )
//                 });
//                 op(0x7B, 1, |gb| {
//                     let a0 = gb.a();
//                     let e = gb.e();
//                     gb.set_a(e);
//                     (
//                         format!("LD A, E"),
//                         format!("A₀ = ${:02x}, E = ${:02x}", a0, e),
//                     )
//                 });
//                 op(0x7C, 1, |gb| {
//                     let a0 = gb.a();
//                     let h = gb.h();
//                     gb.set_a(h);
//                     (
//                         format!("LD A, H"),
//                         format!("A₀ = ${:02x}, H = ${:02x}", a0, h),
//                     )
//                 });
//                 op(0x7D, 1, |gb| {
//                     let a0 = gb.a();
//                     let l = gb.l();
//                     gb.set_a(l);
//                     (
//                         format!("LD A, L"),
//                         format!("A₀ = ${:02x}, L = ${:02x}", a0, l),
//                     )
//                 });
//                 op(0x0A, 2, |gb| {
//                     let a0 = gb.a();
//                     let bc = gb.bc();
//                     let a1 = gb.get_memory(bc);
//                     gb.set_a(a1);
//                     (
//                         format!("LD A, (BC)"),
//                         format!("A₀ = ${:02x}, BC = ${:04x}, (BC) = ${:02x}", a0, bc, a1),
//                     )
//                 });
//                 op(0x1A, 2, |gb| {
//                     let a0 = gb.a();
//                     let de = gb.de();
//                     let a1 = gb.get_memory(de);
//                     gb.set_a(a1);
//                     (
//                         format!("LD A, (DE)"),
//                         format!("A₀ = ${:02x}, DE = ${:04x}, (DE) = ${:02x}", a0, de, a1),
//                     )
//                 });
//                 op(0x7E, 2, |gb| {
//                     let a0 = gb.a();
//                     let hl = gb.hl();
//                     let a1 = gb.get_memory(hl);
//                     gb.set_a(a1);
//                     (
//                         format!("LD A, (HL)"),
//                         format!("A₀ = ${:02x}, HL = ${:04x}, (HL) = ${:02x}", a0, hl, a1),
//                     )
//                 });
//                 op(0xFA, 4, |gb| {
//                     let nn = gb.read_immediate_u16();
//                     let a0 = gb.a();
//                     let a1 = gb.get_memory(nn);
//                     gb.set_a(a1);
//                     (
//                         format!("LD A, (${:04x})", nn),
//                         format!("A₀ = ${:02x}, A₁ = ${:04x}", a0, a1),
//                     )
//                 });
//                 op(0x3E, 2, |gb| {
//                     let n = gb.read_immediate_u8();
//                     let a0 = gb.a();
//                     gb.set_a(n);
//                     (format!("LD A, ${:02x}", n), format!("A₀ = ${:02x}", a0))
//                 });
//                 // LD B, *
//                 op(0x40, 1, |gb| {
//                     let b = gb.b();
//                     (format!("LD B, B"), format!("B = ${:02x}", b))
//                 });
//                 op(0x41, 1, |gb| {
//                     let b0 = gb.b();
//                     let c = gb.c();
//                     gb.set_b(c);
//                     (
//                         format!("LD B, C"),
//                         format!("B₀ = ${:02x}, C = ${:02x}", b0, c),
//                     )
//                 });
//                 op(0x42, 1, |gb| {
//                     let b0 = gb.b();
//                     let d = gb.d();
//                     gb.set_b(d);
//                     (
//                         format!("LD B, D"),
//                         format!("B₀ = ${:02x}, D = ${:02x}", b0, d),
//                     )
//                 });
//                 op(0x43, 1, |gb| {
//                     let b0 = gb.b();
//                     let e = gb.e();
//                     gb.set_b(e);
//                     (
//                         format!("LD B, E"),
//                         format!("B₀ = ${:02x}, E = ${:02x}", b0, e),
//                     )
//                 });
//                 op(0x44, 1, |gb| {
//                     let b0 = gb.b();
//                     let h = gb.h();
//                     gb.set_b(h);
//                     (
//                         format!("LD B, H"),
//                         format!("B₀ = ${:02x}, H = ${:02x}", b0, h),
//                     )
//                 });
//                 op(0x45, 1, |gb| {
//                     let b0 = gb.b();
//                     let l = gb.l();
//                     gb.set_b(l);
//                     (
//                         format!("LD B, L"),
//                         format!("B₀ = ${:02x}, L = ${:02x}", b0, l),
//                     )
//                 });
//                 op(0x46, 2, |gb| {
//                     let b0 = gb.b();
//                     let hl = gb.hl();
//                     let b1 = gb.get_memory(hl);
//                     gb.set_b(b1);
//                     (
//                         format!("LD B, (HL)"),
//                         format!("B₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", b0, hl, b1),
//                     )
//                 });
//                 // LD C, *
//                 op(0x48, 1, |gb| {
//                     let c0 = gb.c();
//                     let b = gb.b();
//                     gb.set_c(b);
//                     (
//                         format!("LD C, B"),
//                         format!("C₀ = ${:02x}, B = ${:02x}", c0, b),
//                     )
//                 });
//                 op(0x49, 1, |gb| {
//                     let c = gb.c();
//                     (format!("LD C, C"), format!("C = ${:02x}", c))
//                 });
//                 op(0x4A, 1, |gb| {
//                     let c0 = gb.c();
//                     let d = gb.d();
//                     gb.set_c(d);
//                     (
//                         format!("LD C, D"),
//                         format!("C₀ = ${:02x}, D = ${:02x}", c0, d),
//                     )
//                 });
//                 op(0x4B, 1, |gb| {
//                     let c0 = gb.c();
//                     let e = gb.e();
//                     gb.set_c(e);
//                     (
//                         format!("LD C, E"),
//                         format!("C₀ = ${:02x}, E = ${:02x}", c0, e),
//                     )
//                 });
//                 op(0x4C, 1, |gb| {
//                     let c0 = gb.c();
//                     let h = gb.h();
//                     gb.set_c(h);
//                     (
//                         format!("LD C, H"),
//                         format!("C₀ = ${:02x}, H = ${:02x}", c0, h),
//                     )
//                 });
//                 op(0x4D, 1, |gb| {
//                     let c0 = gb.c();
//                     let l = gb.l();
//                     gb.set_c(l);
//                     (
//                         format!("LD C, L"),
//                         format!("C₀ = ${:02x}, L = ${:02x}", c0, l),
//                     )
//                 });
//                 op(0x4E, 2, |gb| {
//                     let c0 = gb.c();
//                     let hl = gb.hl();
//                     let c1 = gb.get_memory(hl);
//                     gb.set_c(c1);
//                     (
//                         format!("LD C, (HL)"),
//                         format!("C₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", c0, hl, c1),
//                     )
//                 });
//                 // LD D, *
//                 op(0x50, 1, |gb| {
//                     let d0 = gb.d();
//                     let b = gb.b();
//                     gb.set_d(b);
//                     (
//                         format!("LD D, B"),
//                         format!("D₀ = ${:02x}, B = ${:02x}", d0, b),
//                     )
//                 });
//                 op(0x51, 1, |gb| {
//                     let d0 = gb.d();
//                     let c = gb.c();
//                     gb.set_d(c);
//                     (
//                         format!("LD D, C"),
//                         format!("D₀ = ${:02x}, C = ${:02x}", d0, c),
//                     )
//                 });
//                 op(0x52, 1, |gb| {
//                     let d = gb.d();
//                     (format!("LD D, D"), format!("D = ${:02x}", d))
//                 });
//                 op(0x53, 1, |gb| {
//                     let d0 = gb.d();
//                     let e = gb.e();
//                     gb.set_d(e);
//                     (
//                         format!("LD D, E"),
//                         format!("D₀ = ${:02x}, E = ${:02x}", d0, e),
//                     )
//                 });
//                 op(0x54, 1, |gb| {
//                     let d0 = gb.d();
//                     let h = gb.h();
//                     gb.set_d(h);
//                     (
//                         format!("LD D, H"),
//                         format!("D₀ = ${:02x}, H = ${:02x}", d0, h),
//                     )
//                 });
//                 op(0x55, 1, |gb| {
//                     let d0 = gb.d();
//                     let l = gb.l();
//                     gb.set_d(l);
//                     (
//                         format!("LD D, L"),
//                         format!("D₀ = ${:02x}, L = ${:02x}", d0, l),
//                     )
//                 });
//                 op(0x56, 2, |gb| {
//                     let d0 = gb.d();
//                     let hl = gb.hl();
//                     let d1 = gb.get_memory(hl);
//                     gb.set_d(d1);
//                     (
//                         format!("LD D, (HL)"),
//                         format!("D₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", d0, hl, d1),
//                     )
//                 });
//                 // LD E, *
//                 op(0x58, 1, |gb| {
//                     let e0 = gb.e();
//                     let b = gb.b();
//                     gb.set_e(b);
//                     (
//                         format!("LD E, B"),
//                         format!("E₀ = ${:02x}, B = ${:02x}", e0, b),
//                     )
//                 });
//                 op(0x59, 1, |gb| {
//                     let e0 = gb.e();
//                     let c = gb.c();
//                     gb.set_e(c);
//                     (
//                         format!("LD E, C"),
//                         format!("E₀ = ${:02x}, C = ${:02x}", e0, c),
//                     )
//                 });
//                 op(0x5A, 1, |gb| {
//                     let e0 = gb.e();
//                     let d = gb.d();
//                     gb.set_e(d);
//                     (
//                         format!("LD E, D"),
//                         format!("E₀ = ${:02x}, D = ${:02x}", e0, d),
//                     )
//                 });
//                 op(0x5B, 1, |gb| {
//                     let e = gb.e();
//                     (format!("LD E, E"), format!("E = ${:02x}", e))
//                 });
//                 op(0x5C, 1, |gb| {
//                     let e0 = gb.e();
//                     let h = gb.h();
//                     gb.set_e(h);
//                     (
//                         format!("LD E, H"),
//                         format!("E₀ = ${:02x}, H = ${:02x}", e0, h),
//                     )
//                 });
//                 op(0x5D, 1, |gb| {
//                     let e0 = gb.e();
//                     let l = gb.l();
//                     gb.set_e(l);
//                     (
//                         format!("LD E, L"),
//                         format!("E₀ = ${:02x}, L = ${:02x}", e0, l),
//                     )
//                 });
//                 op(0x5E, 2, |gb| {
//                     let e0 = gb.e();
//                     let hl = gb.hl();
//                     let e1 = gb.get_memory(hl);
//                     gb.set_e(e1);
//                     (
//                         format!("LD E, (HL)"),
//                         format!("E₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", e0, hl, e1),
//                     )
//                 });
//                 // LD H, *
//                 op(0x60, 1, |gb| {
//                     let h0 = gb.h();
//                     let b = gb.b();
//                     gb.set_h(b);
//                     (
//                         format!("LD H, B"),
//                         format!("H₀ = ${:02x}, B = ${:02x}", h0, b),
//                     )
//                 });
//                 op(0x61, 1, |gb| {
//                     let h0 = gb.h();
//                     let c = gb.c();
//                     gb.set_h(c);
//                     (
//                         format!("LD H, C"),
//                         format!("H₀ = ${:02x}, C = ${:02x}", h0, c),
//                     )
//                 });
//                 op(0x62, 1, |gb| {
//                     let h0 = gb.h();
//                     let d = gb.d();
//                     gb.set_h(d);
//                     (
//                         format!("LD H, D"),
//                         format!("H₀ = ${:02x}, D = ${:02x}", h0, d),
//                     )
//                 });
//                 op(0x63, 1, |gb| {
//                     let h0 = gb.h();
//                     let e = gb.e();
//                     gb.set_h(e);
//                     (
//                         format!("LD H, E"),
//                         format!("H₀ = ${:02x}, E = ${:02x}", h0, e),
//                     )
//                 });
//                 op(0x64, 1, |gb| {
//                     let h = gb.h();
//                     (format!("LD H, H"), format!("H = ${:02x}", h))
//                 });
//                 op(0x65, 1, |gb| {
//                     let h0 = gb.h();
//                     let l = gb.l();
//                     gb.set_h(l);
//                     (
//                         format!("LD H, L"),
//                         format!("H₀ = ${:02x}, L = ${:02x}", h0, l),
//                     )
//                 });
//                 op(0x66, 2, |gb| {
//                     let h0 = gb.h();
//                     let hl = gb.hl();
//                     let h1 = gb.get_memory(hl);
//                     gb.set_h(h1);
//                     (
//                         format!("LD H, (HL)"),
//                         format!("H₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", h0, hl, h1),
//                     )
//                 });
//                 // LD L, *
//                 op(0x68, 1, |gb| {
//                     let l0 = gb.l();
//                     let b = gb.b();
//                     gb.set_l(b);
//                     (
//                         format!("LD L, B"),
//                         format!("L₀ = ${:02x}, B = ${:02x}", l0, b),
//                     )
//                 });
//                 op(0x69, 1, |gb| {
//                     let l0 = gb.l();
//                     let c = gb.c();
//                     gb.set_l(c);
//                     (
//                         format!("LD L, C"),
//                         format!("L₀ = ${:02x}, C = ${:02x}", l0, c),
//                     )
//                 });
//                 op(0x6A, 1, |gb| {
//                     let l0 = gb.l();
//                     let d = gb.d();
//                     gb.set_l(d);
//                     (
//                         format!("LD L, D"),
//                         format!("L₀ = ${:02x}, D = ${:02x}", l0, d),
//                     )
//                 });
//                 op(0x6B, 1, |gb| {
//                     let l0 = gb.l();
//                     let e = gb.e();
//                     gb.set_l(e);
//                     (
//                         format!("LD L, E"),
//                         format!("L₀ = ${:02x}, E = ${:02x}", l0, e),
//                     )
//                 });
//                 op(0x6C, 1, |gb| {
//                     let l0 = gb.l();
//                     let h = gb.h();
//                     gb.set_l(h);
//                     (
//                         format!("LD L, H"),
//                         format!("L₀ = ${:02x}, H = ${:02x}", l0, h),
//                     )
//                 });
//                 op(0x6D, 1, |gb| {
//                     let l0 = gb.l();
//                     let l = gb.l();
//                     gb.set_l(l);
//                     (
//                         format!("LD L, L"),
//                         format!("L₀ = ${:02x}, L = ${:02x}", l0, l),
//                     )
//                 });
//                 op(0x6E, 2, |gb| {
//                     let l0 = gb.l();
//                     let hl = gb.hl();
//                     let l1 = gb.get_memory(hl);
//                     gb.set_l(l1);
//                     (
//                         format!("LD L, (HL)"),
//                         format!("L₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", l0, hl, l1),
//                     )
//                 });
//                 // LD (HL), *
//                 op(0x70, 2, |gb| {
//                     let hl = gb.hl();
//                     let b = gb.b();
//                     gb.set_memory(hl, b);
//                     (
//                         format!("LD (HL), B"),
//                         format!("HL = ${:02x}, B = ${:02x}", hl, b),
//                     )
//                 });
//                 op(0x71, 2, |gb| {
//                     let hl = gb.hl();
//                     let c = gb.c();
//                     gb.set_memory(hl, c);
//                     (
//                         format!("LD (HL), C"),
//                         format!("HL = ${:02x}, C = ${:02x}", hl, c),
//                     )
//                 });
//                 op(0x72, 2, |gb| {
//                     let hl = gb.hl();
//                     let d = gb.d();
//                     gb.set_memory(hl, d);
//                     (
//                         format!("LD (HL), D"),
//                         format!("HL = ${:02x}, D = ${:02x}", hl, d),
//                     )
//                 });
//                 op(0x73, 2, |gb| {
//                     let hl = gb.hl();
//                     let e = gb.e();
//                     gb.set_memory(hl, e);
//                     (
//                         format!("LD (HL), E"),
//                         format!("HL = ${:02x}, E = ${:02x}", hl, e),
//                     )
//                 });
//                 op(0x74, 2, |gb| {
//                     let hl = gb.hl();
//                     let h = gb.h();
//                     gb.set_memory(hl, h);
//                     (
//                         format!("LD (HL), H"),
//                         format!("HL = ${:02x}, H = ${:02x}", hl, h),
//                     )
//                 });
//                 op(0x75, 2, |gb| {
//                     let hl = gb.hl();
//                     let l = gb.l();
//                     gb.set_memory(hl, l);
//                     (
//                         format!("LD (HL), L"),
//                         format!("HL = ${:02x}, L = ${:02x}", hl, l),
//                     )
//                 });
//                 op(0x36, 3, |gb| {
//                     let hl = gb.hl();
//                     let n = gb.read_immediate_u8();
//                     gb.set_memory(hl, n);
//                     (format!("LD (HL), ${:02x}", n), format!("HL = ${:02x}", hl))
//                 });
//             }

//             // 4. LD n, A
//             // Put value A into n.
//             {
//                 op(0x47, 1, |gb| {
//                     let b0 = gb.b();
//                     let a = gb.a();
//                     gb.set_b(a);
//                     (
//                         format!("LD B, A"),
//                         format!("B₀ = ${:02x}, A = ${:02x}", b0, a),
//                     )
//                 });
//                 op(0x4F, 1, |gb| {
//                     let c0 = gb.c();
//                     let a = gb.a();
//                     gb.set_b(a);
//                     (
//                         format!("LD C, A"),
//                         format!("C₀ = ${:02x}, A = ${:02x}", c0, a),
//                     )
//                 });
//                 op(0x57, 1, |gb| {
//                     let d0 = gb.d();
//                     let a = gb.a();
//                     gb.set_d(a);
//                     (
//                         format!("LD D, A"),
//                         format!("D₀ = ${:02x}, A = ${:02x}", d0, a),
//                     )
//                 });
//                 op(0x5F, 1, |gb| {
//                     let e0 = gb.e();
//                     let a = gb.a();
//                     gb.set_e(a);
//                     (
//                         format!("LD E, A"),
//                         format!("E₀ = ${:02x}, A = ${:02x}", e0, a),
//                     )
//                 });
//                 op(0x67, 1, |gb| {
//                     let h0 = gb.h();
//                     let a = gb.a();
//                     gb.set_h(a);
//                     (
//                         format!("LD H, A"),
//                         format!("H₀ = ${:02x}, A = ${:02x}", h0, a),
//                     )
//                 });
//                 op(0x6F, 1, |gb| {
//                     let l0 = gb.l();
//                     let a = gb.a();
//                     gb.set_l(a);
//                     (
//                         format!("LD L, A"),
//                         format!("L₀ = ${:02x}, A = ${:02x}", l0, a),
//                     )
//                 });
//                 op(0x02, 2, |gb| {
//                     let bc = gb.bc();
//                     let a = gb.a();
//                     gb.set_memory(bc, a);
//                     (
//                         format!("LD (BC), A"),
//                         format!("BC = ${:04x}, A = ${:02x}", bc, gb.a()),
//                     )
//                 });
//                 op(0x12, 2, |gb| {
//                     let de = gb.de();
//                     let a = gb.a();
//                     gb.set_memory(de, a);
//                     (
//                         format!("LD (DE), A"),
//                         format!("DE = ${:04x}, A = ${:02x}", de, gb.a()),
//                     )
//                 });
//                 op(0x77, 2, |gb| {
//                     let hl = gb.hl();
//                     let a = gb.a();
//                     gb.set_memory(hl, a);
//                     (
//                         format!("LD (HL), A"),
//                         format!("HL = ${:04x}, A = ${:02x}", hl, gb.a()),
//                     )
//                 });
//             }

//             // 6. LD ($FF00 + C), A
//             op(0xE2, 2, |gb| {
//                 let a = gb.a();
//                 let address = 0xFF00 + (gb.c() as u16);
//                 gb.set_memory(address, a);
//                 (
//                     format!("LD ($FF00 + C), A "),
//                     format!("A = ${:02x}, C = ${:02x}", gb.a(), gb.c()),
//                 )
//             });

//             // 12. LD (HL-), A
//             // Put A into memory address HL.
//             // Decrement HL.
//             op(0x32, 2, |gb| {
//                 let hl0 = gb.hl();
//                 let hl1 = hl0 - 1;
//                 let a = gb.a();
//                 gb.set_memory(hl0, a);
//                 gb.set_hl(hl1);
//                 (
//                     format!("LD (HL-), A"),
//                     format!("HL₀ = ${:04x}, A = ${:02x}", hl0, a),
//                 )
//             });

//             // 19. LDH (n), A
//             op(0xE0, 3, |gb| {
//                 let a = gb.a();
//                 let n = gb.read_immediate_u8();
//                 gb.set_memory(0xFF00 as u16 + n as u16, a);
//                 (
//                     format!("LD ($ff00 + ${:02x}), A", n),
//                     format!("A = ${:02x}", a),
//                 )
//             });
//         }

//         // 3.3.2 16-Bit Loads
//         {
//             // 3.3.2. 16-Bit Loads
//             // 1. LD n, nn
//             // Put value nn into n.
//             op(0x01, 3, |gb| {
//                 let bc0 = gb.bc();
//                 let bc1 = gb.read_immediate_u16();
//                 gb.set_bc(bc1);
//                 (
//                     format!("LOAD BC, ${:04x}", bc1),
//                     format!("BC₁ = ${:04x}", bc0),
//                 )
//             });
//             op(0x11, 3, |gb| {
//                 let de0 = gb.de();
//                 let de1 = gb.read_immediate_u16();
//                 gb.set_de(de1);
//                 (
//                     format!("LOAD DE, ${:04x}", de1),
//                     format!("DE₁ = ${:04x}", de0),
//                 )
//             });
//             op(0x21, 3, |gb| {
//                 let hl0 = gb.hl();
//                 let hl1 = gb.read_immediate_u16();
//                 gb.set_hl(hl1);
//                 (
//                     format!("LOAD HL, ${:04x}", hl1),
//                     format!("hl₁ = ${:04x}", hl0),
//                 )
//             });
//             op(0x31, 3, |gb| {
//                 let sp0 = gb.sp();
//                 let sp1 = gb.read_immediate_u16();
//                 gb.set_sp(sp1);
//                 (
//                     format!("LOAD SP, ${:04x}", sp1),
//                     format!("SP₀ = ${:04x}", sp0),
//                 )
//             });

//             // 2. LD SP, HL
//             // Put HL into Stack Pointer (SP).
//             op(0xF9, 2, |gb| {
//                 let sp0 = gb.sp();
//                 let hl = gb.hl();
//                 gb.set_sp(hl);
//                 (
//                     format!("LOAD SP, HL"),
//                     format!("SP₀ = ${:04x}, HL = ${:02x}", sp0, hl),
//                 )
//             });

//             // 5. PUSH nn
//             // Push register pair nn onto stack.
//             // Decrement Stack Pointer (SP) twice.
//             op(0xF5, 4, |gb| {
//                 let af = gb.af();
//                 gb.stack_push(af);
//                 (
//                     format!("PUSH AF"),
//                     format!("SP₁ = ${:04x}, AF = ${:04x}", gb.sp(), af),
//                 )
//             });
//             op(0xC5, 4, |gb| {
//                 let bc = gb.bc();
//                 gb.stack_push(bc);
//                 (
//                     format!("PUSH BC"),
//                     format!("SP₁ = ${:04x}, BC = ${:04x}", gb.sp(), bc),
//                 )
//             });
//             op(0xD5, 4, |gb| {
//                 let de = gb.de();
//                 gb.stack_push(de);
//                 (
//                     format!("PUSH DE"),
//                     format!("SP₁ = ${:04x}, DE = ${:04x}", gb.sp(), de),
//                 )
//             });
//             op(0xE5, 4, |gb| {
//                 let hl = gb.hl();
//                 gb.stack_push(hl);
//                 (
//                     format!("PUSH hl"),
//                     format!("SP₁ = ${:04x}, HL = ${:04x}", gb.sp(), hl),
//                 )
//             });

//             // 7. POP nn
//             // Push two bytes off stack into register pair nn.
//             // Increment Stack Pointer (SP) twice.
//             op(0xF1, 3, |gb| {
//                 let af0 = gb.af();
//                 let af1 = gb.stack_pop();
//                 (
//                     format!("POP AF"),
//                     format!(
//                         "SP₁ = ${:04x}, AF₀ = ${:04x}, AF₁ = ${:04x}",
//                         gb.sp(),
//                         af0,
//                         af1
//                     ),
//                 )
//             });
//             op(0xC1, 3, |gb| {
//                 let bc0 = gb.bc();
//                 let bc1 = gb.stack_pop();
//                 (
//                     format!("POP BC"),
//                     format!(
//                         "SP₁ = ${:04x}, BC₀ = ${:04x}, BC₁ = ${:04x}",
//                         gb.sp(),
//                         bc0,
//                         bc1
//                     ),
//                 )
//             });
//             op(0xD1, 3, |gb| {
//                 let de0 = gb.de();
//                 let de1 = gb.stack_pop();
//                 (
//                     format!("POP DE"),
//                     format!(
//                         "SP₁ = ${:04x}, DE₀ = ${:04x}, DE₁ = ${:04x}",
//                         gb.sp(),
//                         de0,
//                         de1
//                     ),
//                 )
//             });
//             op(0xE1, 3, |gb| {
//                 let hl0 = gb.hl();
//                 let hl1 = gb.stack_pop();
//                 (
//                     format!("POP HL"),
//                     format!(
//                         "SP₁ = ${:04x}, HL₀ = ${:04x}, HL₁ = ${:04x}",
//                         gb.sp(),
//                         hl0,
//                         hl1
//                     ),
//                 )
//             });
//         }

//         // 3.3.3. 8-Bit ALU
//         {
//             // 7. XOR n
//             {
//                 op(0xAF, 1, |gb| {
//                     let a0 = gb.a();
//                     let a1 = a0 ^ a0;
//                     gb.set_a(a1);
//                     gb.set_z_flag(a1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(false);
//                     gb.set_c_flag(false);
//                     (
//                         format!("XOR A"),
//                         format!("A₀ = ${:02x}, A₁ = ${:02x}", a0, a1),
//                     )
//                 });
//                 op(0xA8, 1, |gb| {
//                     let b = gb.b();
//                     let a0 = gb.a();
//                     let a1 = a0 ^ b;
//                     gb.set_a(a1);
//                     gb.set_z_flag(a1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(false);
//                     gb.set_c_flag(false);
//                     (
//                         format!("XOR B"),
//                         format!("A₀ = ${:02x}, B = ${:02x} A₁ = ${:02x}", a0, b, a1),
//                     )
//                 });
//                 op(0xA9, 1, |gb| {
//                     let c = gb.c();
//                     let a0 = gb.a();
//                     let a1 = a0 ^ c;
//                     gb.set_a(a1);
//                     gb.set_z_flag(a1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(false);
//                     gb.set_c_flag(false);
//                     (
//                         format!("XOR C"),
//                         format!("A₀ = ${:02x}, C = ${:02x} A₁ = ${:02x}", a0, c, a1),
//                     )
//                 });
//                 op(0xAA, 1, |gb| {
//                     let d = gb.d();
//                     let a0 = gb.a();
//                     let a1 = a0 ^ d;
//                     gb.set_a(a1);
//                     gb.set_z_flag(a1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(false);
//                     gb.set_c_flag(false);
//                     (
//                         format!("XOR D"),
//                         format!("A₀ = ${:02x}, D = ${:02x} A₁ = ${:02x}", a0, d, a1),
//                     )
//                 });
//                 op(0xAB, 1, |gb| {
//                     let e = gb.e();
//                     let a0 = gb.a();
//                     let a1 = a0 ^ e;
//                     gb.set_a(a1);
//                     gb.set_z_flag(a1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(false);
//                     gb.set_c_flag(false);
//                     (
//                         format!("XOR E"),
//                         format!("A₀ = ${:02x}, E = ${:02x} A₁ = ${:02x}", a0, e, a1),
//                     )
//                 });
//                 op(0xAC, 1, |gb| {
//                     let h = gb.h();
//                     let a0 = gb.a();
//                     let a1 = a0 ^ h;
//                     gb.set_a(a1);
//                     gb.set_z_flag(a1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(false);
//                     gb.set_c_flag(false);
//                     (
//                         format!("XOR B"),
//                         format!("A₀ = ${:02x}, H = ${:02x} A₁ = ${:02x}", a0, h, a1),
//                     )
//                 });
//                 op(0xAD, 1, |gb| {
//                     let l = gb.l();
//                     let a0 = gb.a();
//                     let a1 = a0 ^ l;
//                     gb.set_a(a1);
//                     gb.set_z_flag(a1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(false);
//                     gb.set_c_flag(false);
//                     (
//                         format!("XOR L"),
//                         format!("A₀ = ${:02x}, L = ${:02x} A₁ = ${:02x}", a0, l, a1),
//                     )
//                 });
//             }

//             // 9. INC n
//             {
//                 op(0x3C, 1, |gb| {
//                     let a0 = gb.a();
//                     let a1 = a0 + 1;
//                     gb.set_a(a1);
//                     gb.set_z_flag(a1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(a0 > a1);
//                     (
//                         format!("INC A"),
//                         format!("A₀ = ${:02x}, A₁ = ${:02x}", a0, a1),
//                     )
//                 });
//                 op(0x04, 1, |gb| {
//                     let b0 = gb.b();
//                     let b1 = b0 + 1;
//                     gb.set_b(b1);
//                     gb.set_z_flag(b1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(b0 > b1);
//                     (
//                         format!("INC B"),
//                         format!("B₀ = ${:02x}, B₁ = ${:02x}", b0, b1),
//                     )
//                 });
//                 op(0x0C, 1, |gb| {
//                     let c0 = gb.c();
//                     let c1 = c0 + 1;
//                     gb.set_c(c1);
//                     gb.set_z_flag(c1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(c0 > c1);
//                     (
//                         format!("INC C"),
//                         format!("C₀ = ${:02x}, C₁ = ${:02x}", c0, c1),
//                     )
//                 });
//                 op(0x14, 1, |gb| {
//                     let d0 = gb.d();
//                     let d1 = d0 + 1;
//                     gb.set_d(d1);
//                     gb.set_z_flag(d1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(d0 > d1);
//                     (
//                         format!("INC D"),
//                         format!("D₀ = ${:02x}, D₁ = ${:02x}", d0, d1),
//                     )
//                 });
//                 op(0x1C, 1, |gb| {
//                     let e0 = gb.e();
//                     let e1 = e0 + 1;
//                     gb.set_e(e1);
//                     gb.set_z_flag(e1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(e0 > e1);
//                     (
//                         format!("INC E"),
//                         format!("E₀ = ${:02x}, E₁ = ${:02x}", e0, e1),
//                     )
//                 });
//                 op(0x24, 1, |gb| {
//                     let h0 = gb.h();
//                     let h1 = h0 + 1;
//                     gb.set_h(h1);
//                     gb.set_z_flag(h1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(h0 > h1);
//                     (
//                         format!("INC H"),
//                         format!("H₀ = ${:02x}, H₁ = ${:02x}", h0, h1),
//                     )
//                 });
//                 op(0x2C, 1, |gb| {
//                     let l0 = gb.l();
//                     let l1 = l0 + 1;
//                     gb.set_l(l1);
//                     gb.set_z_flag(l1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(l0 > l1);
//                     (
//                         format!("INC L"),
//                         format!("L₀ = ${:02x}, L₁ = ${:02x}", l0, l1),
//                     )
//                 });
//             }

//             // 10. DEC n
//             {
//                 op(0x3D, 1, |gb| {
//                     let a0 = gb.a();
//                     let a1 = a0 - 1;
//                     gb.set_a(a1);
//                     gb.set_z_flag(a1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(a0 < a1);
//                     (
//                         format!("DEC A"),
//                         format!("A₀ = ${:02x}, A₁ = ${:02x}", a0, a1),
//                     )
//                 });
//                 op(0x05, 1, |gb| {
//                     let b0 = gb.b();
//                     let b1 = b0 - 1;
//                     gb.set_b(b1);
//                     gb.set_z_flag(b1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(b0 < b1);
//                     (
//                         format!("DEC A"),
//                         format!("A₀ = ${:02x}, A₁ = ${:02x}", b0, b1),
//                     )
//                 });
//                 op(0x0D, 1, |gb| {
//                     let c0 = gb.a();
//                     let c1 = c0 - 1;
//                     gb.set_c(c1);
//                     gb.set_z_flag(c1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(c0 < c1);
//                     (
//                         format!("DEC C"),
//                         format!("C₀ = ${:02x}, C₁ = ${:02x}", c0, c1),
//                     )
//                 });
//                 op(0x15, 1, |gb| {
//                     let d0 = gb.d();
//                     let d1 = d0 - 1;
//                     gb.set_d(d1);
//                     gb.set_z_flag(d1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(d0 < d1);
//                     (
//                         format!("DEC D"),
//                         format!("D₀ = ${:02x}, D₁ = ${:02x}", d0, d1),
//                     )
//                 });
//                 op(0x1D, 1, |gb| {
//                     let e0 = gb.e();
//                     let e1 = e0 - 1;
//                     gb.set_e(e1);
//                     gb.set_z_flag(e1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(e0 < e1);
//                     (
//                         format!("DEC A"),
//                         format!("E₀ = ${:02x}, E₁ = ${:02x}", e0, e1),
//                     )
//                 });
//                 op(0x25, 1, |gb| {
//                     let h0 = gb.h();
//                     let h1 = h0 - 1;
//                     gb.set_h(h1);
//                     gb.set_z_flag(h1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(h0 < h1);
//                     (
//                         format!("DEC H"),
//                         format!("H₀ = ${:02x}, H₁ = ${:02x}", h0, h1),
//                     )
//                 });
//                 op(0x2D, 1, |gb| {
//                     let l0 = gb.l();
//                     let l1 = 0 - 1;
//                     gb.set_l(l1);
//                     gb.set_z_flag(l1 == 0);
//                     gb.set_n_flag(false);
//                     gb.set_h_flag(0 < l1);
//                     (
//                         format!("DEC L"),
//                         format!("A₀ = ${:02x}, A₁ = ${:02x}", 0, l1),
//                     )
//                 });
//             }
//         }

//         // 3.3.5. Miscellaneous
//         {
//             // 6. NOP
//             op(0x00, 1, |_gb| (format!("NOP"), format!("")));
//         }

//         // 3.3.6. Rotates & Shifts
//         {
//             // 2. RLA
//             // Rotate A left through Carry flag.
//             // This, 0x17, is the same as 0xCB17 below.
//             op(0x17, 2, |gb| {
//                 let a0 = gb.a();
//                 let a1 = (a0 << 1) + if gb.c_flag() { 1 } else { 0 };
//                 gb.set_a(a1);
//                 gb.set_z_flag(a1 == 0);
//                 gb.set_c_flag(a0 & 0b10000000 > 0);
//                 gb.set_n_flag(false);
//                 gb.set_h_flag(false);
//                 (format!("RLA"), format!("A₀ = {}", a0))
//             });

//             // 6. RL n
//             // Rotate n left through Carry flag.
//             op_cb(0x17, 2, |gb| {
//                 let a0 = gb.a();
//                 let a1 = a0 << 1 + if gb.c_flag() { 1 } else { 0 };
//                 gb.set_a(a1);
//                 gb.set_z_flag(a1 == 0);
//                 gb.set_c_flag(a0 & 0b10000000 > 0);
//                 gb.set_n_flag(false);
//                 gb.set_h_flag(false);
//                 (format!("RL A"), format!("A₀ = {}", a0))
//             });
//             op_cb(0x10, 2, |gb| {
//                 let b0 = gb.b();
//                 let b1 = b0 << 1 + if gb.c_flag() { 1 } else { 0 };
//                 gb.set_b(b1);
//                 gb.set_z_flag(b1 == 0);
//                 gb.set_c_flag(b0 & 0b10000000 > 0);
//                 gb.set_n_flag(false);
//                 gb.set_h_flag(false);
//                 (format!("RL B"), format!("B₀ = {}", b0))
//             });
//             op_cb(0x11, 2, |gb| {
//                 let c0 = gb.c();
//                 let c1 = c0 << 1 + if gb.c_flag() { 1 } else { 0 };
//                 gb.set_c(c1);
//                 gb.set_z_flag(c1 == 0);
//                 gb.set_c_flag(c0 & 0b10000000 > 0);
//                 gb.set_n_flag(false);
//                 gb.set_h_flag(false);
//                 (format!("RL C"), format!("C₀ = {}", c0))
//             });
//             op_cb(0x12, 2, |gb| {
//                 let d0 = gb.d();
//                 let d1 = d0 << 1 + if gb.c_flag() { 1 } else { 0 };
//                 gb.set_d(d1);
//                 gb.set_z_flag(d1 == 0);
//                 gb.set_c_flag(d0 & 0b10000000 > 0);
//                 gb.set_n_flag(false);
//                 gb.set_h_flag(false);
//                 (format!("RL D"), format!("D₀ = {}", d0))
//             });
//             op_cb(0x13, 2, |gb| {
//                 let e0 = gb.e();
//                 let e1 = e0 << 1 + if gb.c_flag() { 1 } else { 0 };
//                 gb.set_e(e1);
//                 gb.set_z_flag(e1 == 0);
//                 gb.set_c_flag(e0 & 0b10000000 > 0);
//                 gb.set_n_flag(false);
//                 gb.set_h_flag(false);
//                 (format!("RL E"), format!("E₀ = {}", e0))
//             });
//             op_cb(0x14, 2, |gb| {
//                 let h0 = gb.h();
//                 let h1 = h0 << 1 + if gb.c_flag() { 1 } else { 0 };
//                 gb.set_h(h1);
//                 gb.set_z_flag(h1 == 0);
//                 gb.set_c_flag(h0 & 0b10000000 > 0);
//                 gb.set_n_flag(false);
//                 gb.set_h_flag(false);
//                 (format!("RL H"), format!("H₀ = {}", h0))
//             });
//             op_cb(0x15, 2, |gb| {
//                 let l0 = gb.l();
//                 let l1 = l0 << 1 + if gb.c_flag() { 1 } else { 0 };
//                 gb.set_l(l1);
//                 gb.set_z_flag(l1 == 0);
//                 gb.set_c_flag(l0 & 0b10000000 > 0);
//                 gb.set_n_flag(false);
//                 gb.set_h_flag(false);
//                 (format!("RL L"), format!("L₀ = {}", l0))
//             });
//         }

//         // 3.3.7. Bit Opcodes
//         {
//             op_cb(0x7C, 2, |gb| {
//                 let result = !u8_get_bit(gb.h(), 7);
//                 gb.set_z_flag(result);
//                 gb.set_n_flag(false);
//                 gb.set_h_flag(true);
//                 (format!("BIT 7, H"), format!("Z₁ = {}", result))
//             });
//         }

//         // 3.3.8. Jumps
//         {
//             // 1. JP nn
//             // Jump to address nn.
//             op(0xC3, 3, |gb| {
//                 let nn = gb.read_immediate_u16();
//                 gb.i = nn;
//                 (format!("JP ${:04x}", nn), format!(""))
//             });
//             // 2. JP cc, nn
//             // Jump to address n if condition is true.
//             op(0xC2, 3, |gb| {
//                 let nn = gb.read_immediate_u16();
//                 let z_flag = gb.z_flag();
//                 if z_flag == false {
//                     gb.i = nn;
//                 }
//                 (format!("JP NZ, ${:04x}", nn), format!("Z = {}", z_flag))
//             });
//             op(0xCA, 3, |gb| {
//                 let nn = gb.read_immediate_u16();
//                 let z_flag = gb.z_flag();
//                 if z_flag {
//                     gb.i = nn;
//                 }
//                 (format!("JP Z, ${:04x}", nn), format!("Z = {}", z_flag))
//             });
//             op(0xD2, 3, |gb| {
//                 let nn = gb.read_immediate_u16();
//                 let c_flag = gb.c_flag();
//                 if c_flag == false {
//                     gb.i = nn;
//                 }
//                 (format!("JP NC, ${:04x}", nn), format!("C = {}", c_flag))
//             });
//             op(0xDA, 3, |gb| {
//                 let nn = gb.read_immediate_u16();
//                 let c_flag = gb.c_flag();
//                 if c_flag {
//                     gb.i = nn;
//                 }
//                 (format!("JP C, ${:04x}", nn), format!("C = {}", c_flag))
//             });
//             // 3. JP (HL)
//             // Jump to address contained in HL.
//             op(0xE9, 1, |gb| {
//                 let hl = gb.hl();
//                 (format!("JP (HL)"), format!("HL = ${:04x}", hl))
//             });
//             // 4. JR n
//             // Add n to current address and jump to it.
//             op(0x18, 2, |gb| {
//                 let n = gb.read_immediate_i8();
//                 (format!("JP {}", n), format!(""))
//             });
//             // 5. JR cc, n
//             // If condition is true then add n to current address and jump to it.
//             op(0x20, 2, |gb| {
//                 let n = gb.read_immediate_i8();
//                 let z_flag = gb.z_flag();
//                 if z_flag == false {
//                     gb.relative_jump(n);
//                 }
//                 (format!("JR NZ, {}", n), format!("Z = {}", z_flag))
//             });
//             op(0x28, 2, |gb| {
//                 let n = gb.read_immediate_i8();
//                 let z_flag = gb.z_flag();
//                 if z_flag {
//                     gb.relative_jump(n);
//                 }
//                 (format!("JR Z, {}", n), format!("Z = {}", z_flag))
//             });
//             op(0x30, 2, |gb| {
//                 let n = gb.read_immediate_i8();
//                 let c_flag = gb.c_flag();
//                 if c_flag == false {
//                     gb.relative_jump(n);
//                 }
//                 (format!("JR NC, {}", n), format!("C = {}", c_flag))
//             });
//             op(0x38, 2, |gb| {
//                 let n = gb.read_immediate_i8();
//                 let c_flag = gb.c_flag();
//                 if c_flag {
//                     gb.relative_jump(n);
//                 }
//                 (format!("JR C, {}", n), format!("C = {}", c_flag))
//             });
//         }

//         // 3.3.9. Calls
//         {
//             // 1. CALL nn
//             // Push address of next instruction onto stack and
//             // then jump to address nn.

//             op(0xCD, 3, |gb| {
//                 let nn = gb.read_immediate_u16();
//                 let i0 = gb.i;
//                 gb.stack_push(i0);
//                 gb.i = nn;
//                 (
//                     format!("CALL ${:04x}", nn),
//                     format!("SP₁ = {:04x}", gb.sp()),
//                 )
//             });
//         }
//     }

//     (operations, operations_cb)
// }
