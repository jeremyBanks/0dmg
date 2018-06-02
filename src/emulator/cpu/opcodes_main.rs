use super::operation;

use emulator::cpu::CPUController;
use emulator::memory::MemoryController;

// one-byte opcodes
pub static OPCODES: [operation::OpFn; 0xFF] = [
    |_00, _gb| {
        op_execution!{
            cycles: 1;
            asm: "NOP";
        }
    },
    |_01, _gb| unimplemented!("opcode 0x01 not implemented"),
    |_02, _gb| unimplemented!("opcode 0x02 not implemented"),
    |_03, _gb| unimplemented!("opcode 0x03 not implemented"),
    |_04, _gb| unimplemented!("opcode 0x04 not implemented"),
    |_05, gb| {
        let b0 = gb.cpu.b;
        let b1 = b0 - 1;
        gb.cpu.b = b1;
        gb.set_z_flag(b1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(b0 < b1);
        op_execution!{
            cycles: 1;
            asm: "DEC A";
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", b0, b1;
        }
    },
    |_06, gb| {
        let b0 = gb.cpu.b;
        let b1 = gb.read_immediate_u8();
        gb.cpu.b = b1;
        op_execution!{
            cycles: 2;
            asm: "LD B, ${:02x}", b1;
            trace: "B₀ = ${:02x}, B₁ = ${:02x}", b0, b1;
        }
    },
    |_07, _gb| unimplemented!("opcode 0x07 not implemented"),
    |_08, _gb| unimplemented!("opcode 0x08 not implemented"),
    |_09, _gb| unimplemented!("opcode 0x09 not implemented"),
    |_0a, gb| {
        let a0 = gb.cpu.a;
        let bc = gb.bc();
        let a1 = gb.get(bc);
        gb.cpu.a = a1;
        op_execution!{
            cycles: 2;
            asm: "LD A, (BC)";
            trace: "A₀ = ${:02x}, BC = ${:04x}, (BC) = ${:02x}", a0, bc, a1;
        }
    },
    |_0b, _gb| unimplemented!("opcode 0x0B not implemented"),
    |_0c, gb| {
        let c0 = gb.cpu.c;
        let c1 = c0 + 1;
        gb.cpu.c = c1;
        gb.set_z_flag(c1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(c0 > c1);
        op_execution!{
            cycles: 1;
            asm: "INC C";
            trace: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1;
        }
    },
    |_0d, gb| {
        let c0 = gb.cpu.a;
        let c1 = c0 - 1;
        gb.cpu.c = c1;
        gb.set_z_flag(c1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(c0 < c1);
        op_execution!{
            cycles: 1;
            asm: "DEC C";
            trace: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1;
        }
    },
    |_0e, gb| {
        let c0 = gb.cpu.c;
        let c1 = gb.read_immediate_u8();
        gb.cpu.c = c1;
        op_execution!{
            cycles: 2;
            asm: "LD C, ${:02x}", c1;
            trace: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1;
        }
    },
    |_0f, _gb| unimplemented!("opcode 0x0F not implemented"),
    |_10, _gb| unimplemented!("opcode 0x10 not implemented"),
    |_11, gb| {
        let de0 = gb.de();
        let de1 = gb.read_immediate_u16();
        gb.set_de(de1);
        op_execution!{
            cycles: 3;
            asm: "LOAD DE, ${:04x}", de1;
            trace: "DE₁ = ${:04x}", de0;
        }
    },
    |_12, _gb| unimplemented!("opcode 0x12 not implemented"),
    |_13, _gb| unimplemented!("opcode 0x13 not implemented"),
    |_14, _gb| unimplemented!("opcode 0x14 not implemented"),
    |_15, gb| {
        let d0 = gb.cpu.d;
        let d1 = d0 - 1;
        gb.cpu.d = d1;
        gb.set_z_flag(d1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(d0 < d1);
        op_execution!{
            cycles: 1;
            asm: "DEC D";
            trace: "D₀ = ${:02x}, D₁ = ${:02x}", d0, d1;
        }
    },
    |_16, gb| {
        let d0 = gb.cpu.d;
        let d1 = gb.read_immediate_u8();
        gb.cpu.d = d1;
        op_execution!{
            cycles: 2;
            asm: "LD D, ${:02x}", d1;
            trace: "D₀ = ${:02x}, D₁ = ${:02x}", d0, d1;
        }
    },
    |_17, gb| {
        let a0 = gb.cpu.a;
        let a1 = a0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_c_flag(a0 & 0b10000000 > 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL A";
            trace: "A₀ = {}", a0;
        }
    },
    |_18, _gb| unimplemented!("opcode 0x18 not implemented"),
    |_19, _gb| unimplemented!("opcode 0x19 not implemented"),
    |_1a, gb| {
        let a0 = gb.cpu.a;
        let de = gb.de();
        let a1 = gb.get(de);
        gb.cpu.a = a1;
        op_execution!{
            cycles: 2;
            asm: "LD A, (DE)";
            trace: "A₀ = ${:02x}, DE = ${:04x}, (DE) = ${:02x}", a0, de, a1;
        }
    },
    |_1b, _gb| unimplemented!("opcode 0x1B not implemented"),
    |_1c, _gb| unimplemented!("opcode 0x1C not implemented"),
    |_1d, gb| {
        let e0 = gb.cpu.e;
        let e1 = e0 - 1;
        gb.cpu.e = e1;
        gb.set_z_flag(e1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(e0 < e1);
        op_execution!{
            cycles: 1;
            asm: "DEC A";
            trace: "E₀ = ${:02x}, E₁ = ${:02x}", e0, e1;
        }
    },
    |_1e, gb| {
        let e0 = gb.cpu.e;
        let e1 = gb.read_immediate_u8();
        gb.cpu.e = e1;
        op_execution!{
            cycles: 2;
            asm: "LD E, ${:02x}", e1;
            trace: "E₀ = ${:02x}, E₁ = ${:02x}", e0, e1;
        }
    },
    |_1f, _gb| unimplemented!("opcode 0x1F not implemented"),
    |_20, gb| {
        let n = gb.read_immediate_i8();
        let z_flag = gb.z_flag();
        if z_flag == false {
            gb.relative_jump(n);
        }
        op_execution!{
            cycles: 2;
            asm: "JR NZ, {}", n;
            trace: "Z = {}", z_flag;
        }
    },
    |_21, gb| {
        let _hl0 = gb.hl();
        let hl1 = gb.read_immediate_u16();
        gb.set_hl(hl1);
        op_execution!{
            cycles: 3;
            asm: "LOAD HL, ${:04x}", hl1;
            trace: "HL₁ = ${:04x}", hl1;
        }
    },
    |_22, gb| {
        let a = gb.cpu.a;
        let hl0 = gb.hl();
        let hl1 = hl0 + 1;
        gb.set(hl0, a);
        gb.set_hl(hl1);
        op_execution!{
            cycles: 2;
            asm: "LD (HL+), A";
            trace: "HL₀ = ${:04x}, A = ${:02x}", hl0, a;
        }
    },
    |_23, _gb| unimplemented!("opcode 0x23 not implemented"),
    |_24, _gb| unimplemented!("opcode 0x24 not implemented"),
    |_25, gb| {
        let h0 = gb.cpu.h;
        let h1 = h0 - 1;
        gb.cpu.h = h1;
        gb.set_z_flag(h1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(h0 < h1);
        op_execution!{
            cycles: 1;
            asm: "DEC H";
            trace: "H₀ = ${:02x}, H₁ = ${:02x}", h0, h1;
        }
    },
    |_26, gb| {
        let h0 = gb.cpu.h;
        let h1 = gb.read_immediate_u8();
        gb.cpu.h = h1;
        op_execution!{
            cycles: 2;
            asm: "LD H, ${:02x}", h1;
            trace: "H₀ = ${:02x}, H₁ = ${:02x}", h0, h1;
        }
    },
    |_27, _gb| unimplemented!("opcode 0x27 not implemented"),
    |_28, gb| {
        let n = gb.read_immediate_i8();
        let z_flag = gb.z_flag();
        if z_flag {
            gb.relative_jump(n);
        }
        op_execution!{
            cycles: 2;
            asm: "JR Z, {}", n;
            trace: "Z = {}", z_flag;
        }
    },
    |_29, _gb| unimplemented!("opcode 0x29 not implemented"),
    |_2a, _gb| unimplemented!("opcode 0x2A not implemented"),
    |_2b, _gb| unimplemented!("opcode 0x2B not implemented"),
    |_2c, _gb| unimplemented!("opcode 0x2C not implemented"),
    |_2d, gb| {
        let l0 = gb.cpu.l;
        let l1 = l0 - 1;
        gb.cpu.l = l1;
        gb.set_z_flag(l1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(0 < l1);
        op_execution!{
            cycles: 1;
            asm: "DEC L";
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", l0, l1;
        }
    },
    |_2e, gb| {
        let l0 = gb.cpu.l;
        let l1 = gb.read_immediate_u8();
        gb.cpu.l = l1;
        op_execution!{
            cycles: 2;
            asm: "LD L, ${:02x}", l1;
            trace: "L₀ = ${:02x}, L₁ = ${:02x}", l0, l1;
        }
    },
    |_2f, _gb| unimplemented!("opcode 0x2F not implemented"),
    |_30, gb| {
        let n = gb.read_immediate_i8();
        let c_flag = gb.c_flag();
        if c_flag == false {
            gb.relative_jump(n);
        }
        op_execution!{
            cycles: 2;
            asm: "JR NC, {}", n;
            trace: "C = {}", c_flag;
        }
    },
    |_31, gb| {
        let sp0 = gb.cpu.sp;
        let sp1 = gb.read_immediate_u16();
        gb.cpu.sp = sp1;
        op_execution!{
            cycles: 3;
            asm: "LOAD SP, ${:04x}", sp1;
            trace: "SP₀ = ${:04x}", sp0;
        }
    },
    |_32, gb| {
        let hl0 = gb.hl();
        let hl1 = hl0 - 1;
        let a = gb.cpu.a;
        gb.set(hl0, a);
        gb.set_hl(hl1);
        op_execution!{
            cycles: 2;
            asm: "LD (HL-), A";
            trace: "HL₀ = ${:04x}, A = ${:02x}", hl0, a;
        }
    },
    |_33, _gb| unimplemented!("opcode 0x33 not implemented"),
    |_34, _gb| unimplemented!("opcode 0x34 not implemented"),
    |_35, _gb| unimplemented!("opcode 0x35 not implemented"),
    |_36, _gb| unimplemented!("opcode 0x36 not implemented"),
    |_37, _gb| unimplemented!("opcode 0x37 not implemented"),
    |_38, gb| {
        let n = gb.read_immediate_i8();
        let c_flag = gb.c_flag();
        if c_flag {
            gb.relative_jump(n);
        }
        op_execution!{
            cycles: 2;
            asm: "JR C, {}", n;
            trace: "C = {}", c_flag;
        }
    },
    |_39, _gb| unimplemented!("opcode 0x39 not implemented"),
    |_3a, _gb| unimplemented!("opcode 0x3A not implemented"),
    |_3b, _gb| unimplemented!("opcode 0x3B not implemented"),
    |_3c, _gb| unimplemented!("opcode 0x3C not implemented"),
    |_3d, gb| {
        let a0 = gb.cpu.a;
        let a1 = a0 - 1;
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(a0 < a1);
        op_execution!{
            cycles: 1;
            asm: "DEC A";
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", a0, a1;
        }
    },
    |_3e, gb| {
        let n = gb.read_immediate_u8();
        let a0 = gb.cpu.a;
        gb.cpu.a = n;
        op_execution!{
            cycles: 2;
            asm: "LD A, ${:02x}", n;
            trace: "A₀ = ${:02x}", a0;
        }
    },
    |_3f, _gb| unimplemented!("opcode 0x3F not implemented"),
    |_40, gb| {
        let b = gb.cpu.b;
        op_execution!{
            cycles: 1;
            asm: "LD B, B";
            trace: "B = ${:02x}", b;
        }
    },
    |_41, gb| {
        let b0 = gb.cpu.b;
        let c = gb.cpu.c;
        gb.cpu.b = c;
        op_execution!{
            cycles: 1;
            asm: "LD B, C";
            trace: "B₀ = ${:02x}, C = ${:02x}", b0, c;
        }
    },
    |_42, gb| {
        let b0 = gb.cpu.b;
        let d = gb.cpu.d;
        gb.cpu.b = d;
        op_execution!{
            cycles: 1;
            asm: "LD B, D";
            trace: "B₀ = ${:02x}, D = ${:02x}", b0, d;
        }
    },
    |_43, gb| {
        let b0 = gb.cpu.b;
        let e = gb.cpu.e;
        gb.cpu.b = e;
        op_execution!{
            cycles: 1;
            asm: "LD B, E";
            trace: "B₀ = ${:02x}, E = ${:02x}", b0, e;
        }
    },
    |_44, gb| {
        let b0 = gb.cpu.b;
        let h = gb.cpu.h;
        gb.cpu.b = h;
        op_execution!{
            cycles: 1;
            asm: "LD B, H";
            trace: "B₀ = ${:02x}, H = ${:02x}", b0, h;
        }
    },
    |_45, gb| {
        let b0 = gb.cpu.b;
        let l = gb.cpu.l;
        gb.cpu.b = l;
        op_execution!{
            cycles: 1;
            asm: "LD B, L";
            trace: "B₀ = ${:02x}, L = ${:02x}", b0, l;
        }
    },
    |_46, gb| {
        let b0 = gb.cpu.b;
        let hl = gb.hl();
        let b1 = gb.get(hl);
        gb.cpu.b = b1;
        op_execution!{
            cycles: 2;
            asm: "LD B, (HL)";
            trace: "B₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", b0, hl, b1;
        }
    },
    |_47, _gb| unimplemented!("opcode 0x47 not implemented"),
    |_48, gb| {
        let c0 = gb.cpu.c;
        let b = gb.cpu.b;
        gb.cpu.c = b;
        op_execution!{
            cycles: 1;
            asm: "LD C, B";
            trace: "C₀ = ${:02x}, B = ${:02x}", c0, b;
        }
    },
    |_49, gb| {
        let c = gb.cpu.c;
        op_execution!{
            cycles: 1;
            asm: "LD C, C";
            trace: "C = ${:02x}", c;
        }
    },
    |_4a, gb| {
        let c0 = gb.cpu.c;
        let d = gb.cpu.d;
        gb.cpu.c = d;
        op_execution!{
            cycles: 1;
            asm: "LD C, D";
            trace: "C₀ = ${:02x}, D = ${:02x}", c0, d;
        }
    },
    |_4b, gb| {
        let c0 = gb.cpu.c;
        let e = gb.cpu.e;
        gb.cpu.c = e;
        op_execution!{
            cycles: 1;
            asm: "LD C, E";
            trace: "C₀ = ${:02x}, E = ${:02x}", c0, e;
        }
    },
    |_4c, gb| {
        let c0 = gb.cpu.c;
        let h = gb.cpu.h;
        gb.cpu.c = h;
        op_execution!{
            cycles: 1;
            asm: "LD C, H";
            trace: "C₀ = ${:02x}, H = ${:02x}", c0, h;
        }
    },
    |_4d, gb| {
        let c0 = gb.cpu.c;
        let l = gb.cpu.l;
        gb.cpu.c = l;
        op_execution!{
            cycles: 1;
            asm: "LD C, L";
            trace: "C₀ = ${:02x}, L = ${:02x}", c0, l;
        }
    },
    |_4e, gb| {
        let c0 = gb.cpu.c;
        let hl = gb.hl();
        let c1 = gb.get(hl);
        gb.cpu.c = c1;
        op_execution!{
            cycles: 2;
            asm: "LD C, (HL)";
            trace: "C₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", c0, hl, c1;
        }
    },
    |_4f, gb| {
        let c0 = gb.cpu.c;
        let a = gb.cpu.a;
        gb.cpu.b = a;
        op_execution!{
            cycles: 1;
            asm: "LD C, A";
            trace: "C₀ = ${:02x}, A = ${:02x}", c0, a;
        }
    },
    |_50, _gb| unimplemented!("opcode 0x50 not implemented"),
    |_51, _gb| unimplemented!("opcode 0x51 not implemented"),
    |_52, _gb| unimplemented!("opcode 0x52 not implemented"),
    |_53, _gb| unimplemented!("opcode 0x53 not implemented"),
    |_54, _gb| unimplemented!("opcode 0x54 not implemented"),
    |_55, _gb| unimplemented!("opcode 0x55 not implemented"),
    |_56, _gb| unimplemented!("opcode 0x56 not implemented"),
    |_57, _gb| unimplemented!("opcode 0x57 not implemented"),
    |_58, _gb| unimplemented!("opcode 0x58 not implemented"),
    |_59, _gb| unimplemented!("opcode 0x59 not implemented"),
    |_5a, _gb| unimplemented!("opcode 0x5A not implemented"),
    |_5b, _gb| unimplemented!("opcode 0x5B not implemented"),
    |_5c, _gb| unimplemented!("opcode 0x5C not implemented"),
    |_5d, _gb| unimplemented!("opcode 0x5D not implemented"),
    |_5e, _gb| unimplemented!("opcode 0x5E not implemented"),
    |_5f, _gb| unimplemented!("opcode 0x5F not implemented"),
    |_60, _gb| unimplemented!("opcode 0x60 not implemented"),
    |_61, _gb| unimplemented!("opcode 0x61 not implemented"),
    |_62, _gb| unimplemented!("opcode 0x62 not implemented"),
    |_63, _gb| unimplemented!("opcode 0x63 not implemented"),
    |_64, _gb| unimplemented!("opcode 0x64 not implemented"),
    |_65, _gb| unimplemented!("opcode 0x65 not implemented"),
    |_66, _gb| unimplemented!("opcode 0x66 not implemented"),
    |_67, _gb| unimplemented!("opcode 0x67 not implemented"),
    |_68, _gb| unimplemented!("opcode 0x68 not implemented"),
    |_69, _gb| unimplemented!("opcode 0x69 not implemented"),
    |_6a, _gb| unimplemented!("opcode 0x6A not implemented"),
    |_6b, _gb| unimplemented!("opcode 0x6B not implemented"),
    |_6c, _gb| unimplemented!("opcode 0x6C not implemented"),
    |_6d, _gb| unimplemented!("opcode 0x6D not implemented"),
    |_6e, _gb| unimplemented!("opcode 0x6E not implemented"),
    |_6f, _gb| unimplemented!("opcode 0x6F not implemented"),
    |_70, _gb| unimplemented!("opcode 0x70 not implemented"),
    |_71, _gb| unimplemented!("opcode 0x71 not implemented"),
    |_72, _gb| unimplemented!("opcode 0x72 not implemented"),
    |_73, _gb| unimplemented!("opcode 0x73 not implemented"),
    |_74, _gb| unimplemented!("opcode 0x74 not implemented"),
    |_75, _gb| unimplemented!("opcode 0x75 not implemented"),
    |_76, _gb| unimplemented!("opcode 0x76 not implemented"),
    |_77, gb| {
        let hl = gb.hl();
        let a = gb.cpu.a;
        gb.set(hl, a);
        op_execution!{
            cycles: 2;
            asm: "LD (HL), A";
            trace: "HL = ${:04x}, A = ${:02x}", hl, gb.cpu.a;
        }
    },
    |_78, gb| {
        let a0 = gb.cpu.a;
        let b = gb.cpu.b;
        gb.cpu.a = b;
        op_execution!{
            cycles: 1;
            asm: "LD A, B";
            trace: "A₀ = ${:02x}, B = ${:02x}", a0, b;
        }
    },
    |_79, gb| {
        let a0 = gb.cpu.a;
        let c = gb.cpu.c;
        gb.cpu.a = c;
        op_execution!{
            cycles: 1;
            asm: "LD A, C";
            trace: "A₀ = ${:02x}, C = ${:02x}", a0, c;
        }
    },
    |_7a, gb| {
        let a0 = gb.cpu.a;
        let d = gb.cpu.d;
        gb.cpu.a = d;
        op_execution!{
            cycles: 1;
            asm: "LD A, D";
            trace: "A₀ = ${:02x}, D = ${:02x}", a0, d;
        }
    },
    |_7b, gb| {
        let a0 = gb.cpu.a;
        let e = gb.cpu.e;
        gb.cpu.a = e;
        op_execution!{
            cycles: 1;
            asm: "LD A, E";
            trace: "A₀ = ${:02x}, E = ${:02x}", a0, e;
        }
    },
    |_7c, gb| {
        let a0 = gb.cpu.a;
        let h = gb.cpu.h;
        gb.cpu.a = h;
        op_execution!{
            cycles: 1;
            asm: "LD A, H";
            trace: "A₀ = ${:02x}, H = ${:02x}", a0, h;
        }
    },
    |_7d, gb| {
        let a0 = gb.cpu.a;
        let l = gb.cpu.l;
        gb.cpu.a = l;
        op_execution!{
            cycles: 1;
            asm: "LD A, L";
            trace: "A₀ = ${:02x}, L = ${:02x}", a0, l;
        }
    },
    |_7e, gb| {
        let a0 = gb.cpu.a;
        let hl = gb.hl();
        let a1 = gb.get(hl);
        gb.cpu.a = a1;
        op_execution!{
            cycles: 2;
            asm: "LD A, (HL)";
            trace: "A₀ = ${:02x}, HL = ${:04x}, (HL) = ${:02x}", a0, hl, a1;
        }
    },
    |_7f, gb| {
        let a = gb.cpu.a;
        op_execution!{
            cycles: 1;
            asm: "LD A, A";
            trace: "A = ${:02x}", a;
        }
    },
    |_80, _gb| unimplemented!("opcode 0x80 not implemented"),
    |_81, _gb| unimplemented!("opcode 0x81 not implemented"),
    |_82, _gb| unimplemented!("opcode 0x82 not implemented"),
    |_83, _gb| unimplemented!("opcode 0x83 not implemented"),
    |_84, _gb| unimplemented!("opcode 0x84 not implemented"),
    |_85, _gb| unimplemented!("opcode 0x85 not implemented"),
    |_86, _gb| unimplemented!("opcode 0x86 not implemented"),
    |_87, _gb| unimplemented!("opcode 0x87 not implemented"),
    |_88, _gb| unimplemented!("opcode 0x88 not implemented"),
    |_89, _gb| unimplemented!("opcode 0x89 not implemented"),
    |_8a, _gb| unimplemented!("opcode 0x8A not implemented"),
    |_8b, _gb| unimplemented!("opcode 0x8B not implemented"),
    |_8c, _gb| unimplemented!("opcode 0x8C not implemented"),
    |_8d, _gb| unimplemented!("opcode 0x8D not implemented"),
    |_8e, _gb| unimplemented!("opcode 0x8E not implemented"),
    |_8f, _gb| unimplemented!("opcode 0x8F not implemented"),
    |_90, _gb| unimplemented!("opcode 0x90 not implemented"),
    |_91, _gb| unimplemented!("opcode 0x91 not implemented"),
    |_92, _gb| unimplemented!("opcode 0x92 not implemented"),
    |_93, _gb| unimplemented!("opcode 0x93 not implemented"),
    |_94, _gb| unimplemented!("opcode 0x94 not implemented"),
    |_95, _gb| unimplemented!("opcode 0x95 not implemented"),
    |_96, _gb| unimplemented!("opcode 0x96 not implemented"),
    |_97, _gb| unimplemented!("opcode 0x97 not implemented"),
    |_98, _gb| unimplemented!("opcode 0x98 not implemented"),
    |_99, _gb| unimplemented!("opcode 0x99 not implemented"),
    |_9a, _gb| unimplemented!("opcode 0x9A not implemented"),
    |_9b, _gb| unimplemented!("opcode 0x9B not implemented"),
    |_9c, _gb| unimplemented!("opcode 0x9C not implemented"),
    |_9d, _gb| unimplemented!("opcode 0x9D not implemented"),
    |_9e, _gb| unimplemented!("opcode 0x9E not implemented"),
    |_9f, _gb| unimplemented!("opcode 0x9F not implemented"),
    |_a0, _gb| unimplemented!("opcode 0xA0 not implemented"),
    |_a1, _gb| unimplemented!("opcode 0xA1 not implemented"),
    |_a2, _gb| unimplemented!("opcode 0xA2 not implemented"),
    |_a3, _gb| unimplemented!("opcode 0xA3 not implemented"),
    |_a4, _gb| unimplemented!("opcode 0xA4 not implemented"),
    |_a5, _gb| unimplemented!("opcode 0xA5 not implemented"),
    |_a6, _gb| unimplemented!("opcode 0xA6 not implemented"),
    |_a7, _gb| unimplemented!("opcode 0xA7 not implemented"),
    |_a8, gb| {
        let b = gb.cpu.b;
        let a0 = gb.cpu.a;
        let a1 = a0 ^ b;
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        gb.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR B";
            trace: "A₀ = ${:02x}, B = ${:02x} A₁ = ${:02x}", a0, b, a1;
        }
    },
    |_a9, gb| {
        let c = gb.cpu.c;
        let a0 = gb.cpu.a;
        let a1 = a0 ^ c;
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        gb.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR C";
            trace: "A₀ = ${:02x}, C = ${:02x} A₁ = ${:02x}", a0, c, a1;
        }
    },
    |_aa, gb| {
        let d = gb.cpu.d;
        let a0 = gb.cpu.a;
        let a1 = a0 ^ d;
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        gb.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR D";
            trace: "A₀ = ${:02x}, D = ${:02x} A₁ = ${:02x}", a0, d, a1;
        }
    },
    |_ab, gb| {
        let e = gb.cpu.e;
        let a0 = gb.cpu.a;
        let a1 = a0 ^ e;
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        gb.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR E";
            trace: "A₀ = ${:02x}, E = ${:02x} A₁ = ${:02x}", a0, e, a1;
        }
    },
    |_ac, gb| {
        let h = gb.cpu.h;
        let a0 = gb.cpu.a;
        let a1 = a0 ^ h;
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        gb.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR B";
            trace: "A₀ = ${:02x}, H = ${:02x} A₁ = ${:02x}", a0, h, a1;
        }
    },
    |_ad, gb| {
        let l = gb.cpu.l;
        let a0 = gb.cpu.a;
        let a1 = a0 ^ l;
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        gb.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR L";
            trace: "A₀ = ${:02x}, L = ${:02x} A₁ = ${:02x}", a0, l, a1;
        }
    },
    |_ae, _gb| unimplemented!("opcode 0xAE not implemented"),
    |_af, gb| {
        let a0 = gb.cpu.a;
        let a1 = a0 ^ a0;
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        gb.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR A";
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", a0, a1;
        }
    },
    |_b0, _gb| unimplemented!("opcode 0xB0 not implemented"),
    |_b1, _gb| unimplemented!("opcode 0xB1 not implemented"),
    |_b2, _gb| unimplemented!("opcode 0xB2 not implemented"),
    |_b3, _gb| unimplemented!("opcode 0xB3 not implemented"),
    |_b4, _gb| unimplemented!("opcode 0xB4 not implemented"),
    |_b5, _gb| unimplemented!("opcode 0xB5 not implemented"),
    |_b6, _gb| unimplemented!("opcode 0xB6 not implemented"),
    |_b7, _gb| unimplemented!("opcode 0xB7 not implemented"),
    |_b8, _gb| unimplemented!("opcode 0xB8 not implemented"),
    |_b9, _gb| unimplemented!("opcode 0xB9 not implemented"),
    |_ba, _gb| unimplemented!("opcode 0xBA not implemented"),
    |_bb, _gb| unimplemented!("opcode 0xBB not implemented"),
    |_bc, _gb| unimplemented!("opcode 0xBC not implemented"),
    |_bd, _gb| unimplemented!("opcode 0xBD not implemented"),
    |_be, _gb| unimplemented!("opcode 0xBE not implemented"),
    |_bf, _gb| unimplemented!("opcode 0xBF not implemented"),
    |_c0, _gb| unimplemented!("opcode 0xC0 not implemented"),
    |_c1, gb| {
        let bc0 = gb.bc();
        let bc1 = gb.stack_pop();
        op_execution!{
            cycles: 3;
            asm: "POP BC";
            trace: "P₁ = ${:04x}, BC₀ = ${:04x}, BC₁ = ${:04x}", gb.cpu.sp, bc0, bc1;
        }
    },
    |_c2, _gb| unimplemented!("opcode 0xC2 not implemented"),
    |_c3, _gb| unimplemented!("opcode 0xC3 not implemented"),
    |_c4, _gb| unimplemented!("opcode 0xC4 not implemented"),
    |_c5, gb| {
        let bc = gb.bc();
        gb.stack_push(bc);
        op_execution!{
            cycles: 4;
            asm: "PUSH BC";
            trace: "SP₁ = ${:04x}, BC = ${:04x}", gb.cpu.sp, bc;
        }
    },
    |_c6, _gb| unimplemented!("opcode 0xC6 not implemented"),
    |_c7, _gb| unimplemented!("opcode 0xC7 not implemented"),
    |_c8, _gb| unimplemented!("opcode 0xC8 not implemented"),
    |_c9, _gb| unimplemented!("opcode 0xC9 not implemented"),
    |_ca, _gb| unimplemented!("opcode 0xCA not implemented"),
    |_cb, _gb| {
        panic!("0xCB prefix is not a complete opcode");
    },
    |_cc, _gb| unimplemented!("opcode 0xCC not implemented"),
    |_cd, gb| {
        let nn = gb.read_immediate_u16();
        let i0 = gb.cpu.i;
        gb.stack_push(i0);
        gb.cpu.i = nn;
        op_execution!{
            cycles: 3;
            asm: "CALL ${:04x}", nn;
            trace: "SP₁ = {:04x}", gb.cpu.sp;
        }
    },
    |_ce, _gb| unimplemented!("opcode 0xCE not implemented"),
    |_cf, _gb| unimplemented!("opcode 0xCF not implemented"),
    |_d0, _gb| unimplemented!("opcode 0xD0 not implemented"),
    |_d1, gb| {
        let de0 = gb.de();
        let de1 = gb.stack_pop();
        op_execution!{
            cycles: 3;
            asm: "POP DE";
            trace: "SP₁ = ${:04x}, DE₀ = ${:04x}, DE₁ = ${:04x}", gb.cpu.sp, de0, de1;
        }
    },
    |_d2, _gb| unimplemented!("opcode 0xD2 not implemented"),
    |_d3, _gb| {
        panic!("0xD3 is not a valid opcode");
    },
    |_d4, _gb| unimplemented!("opcode 0xD4 not implemented"),
    |_d5, gb| {
        let de = gb.de();
        gb.stack_push(de);
        op_execution!{
            cycles: 4;
            asm: "PUSH DE";
            trace: "SP₁ = ${:04x}, DE = ${:04x}", gb.cpu.sp, de;
        }
    },
    |_d6, _gb| unimplemented!("opcode 0xD6 not implemented"),
    |_d7, _gb| unimplemented!("opcode 0xD7 not implemented"),
    |_d8, _gb| unimplemented!("opcode 0xD8 not implemented"),
    |_d9, _gb| unimplemented!("opcode 0xD9 not implemented"),
    |_da, _gb| unimplemented!("opcode 0xDA not implemented"),
    |_db, _gb| {
        panic!("0xDB is not a valid opcode");
    },
    |_dc, _gb| unimplemented!("opcode 0xDC not implemented"),
    |_dd, _gb| {
        panic!("0xDD is not a valid opcode");
    },
    |_de, _gb| unimplemented!("opcode 0xDE not implemented"),
    |_df, _gb| unimplemented!("opcode 0xDF not implemented"),
    |_e0, gb| {
        let a = gb.cpu.a;
        let n = gb.read_immediate_u8();
        gb.set(0xFF00 + n as u16, a);
        op_execution!{
            cycles: 3;
            asm: "LD ($ff00 + ${:02x}), A", n;
            trace: "A = ${:02x}", a;
        }
    },
    |_e1, gb| {
        let hl0 = gb.hl();
        let hl1 = gb.stack_pop();
        op_execution!{
            cycles: 3;
            asm: "POP HL";
            trace: "SP₁ = ${:04x}, HL₀ = ${:04x}, HL₁ = ${:04x}", gb.cpu.sp, hl0, hl1;
        }
    },
    |_e2, gb| {
        let a = gb.cpu.a;
        let c = gb.cpu.c;
        let address = 0xFF00 + (c as u16);
        gb.set(address, a);
        op_execution!{
            cycles: 2;
            asm: "LD ($FF00 + C), A ";
            trace: "A = ${:02x}, C = ${:02x}", a, c;
        }
    },
    |_e3, _gb| {
        panic!("0xE3 is not a valid opcode");
    },
    |_e4, _gb| {
        panic!("0xE4 is not a valid opcode");
    },
    |_e5, gb| {
        let hl = gb.hl();
        gb.stack_push(hl);
        op_execution!{
            cycles: 4;
            asm: "PUSH hl";
            trace: "SP₁ = ${:04x}, HL = ${:04x}", gb.cpu.sp, hl;
        }
    },
    |_e6, _gb| unimplemented!("opcode 0xE6 not implemented"),
    |_e7, _gb| unimplemented!("opcode 0xE7 not implemented"),
    |_e8, _gb| unimplemented!("opcode 0xE8 not implemented"),
    |_e9, _gb| unimplemented!("opcode 0xE9 not implemented"),
    |_ea, _gb| unimplemented!("opcode 0xEA not implemented"),
    |_eb, _gb| {
        panic!("0xEB is not a valid opcode");
    },
    |_ec, _gb| {
        panic!("0xEC is not a valid opcode");
    },
    |_ed, _gb| {
        panic!("0xED is not a valid opcode");
    },
    |_ee, _gb| unimplemented!("opcode 0xEE not implemented"),
    |_ef, _gb| unimplemented!("opcode 0xEF not implemented"),
    |_f0, _gb| unimplemented!("opcode 0xF0 not implemented"),
    |_f1, gb| {
        let af0 = gb.af();
        let af1 = gb.stack_pop();
        op_execution!{
            cycles: 3;
            asm: "POP AF";
            trace: "SP₁ = ${:04x}, AF₀ = ${:04x}, AF₁ = ${:04x}", gb.cpu.sp, af0, af1;
        }
    },
    |_f2, _gb| unimplemented!("opcode 0xF2 not implemented"),
    |_f3, _gb| unimplemented!("opcode 0xF3 not implemented"),
    |_f4, _gb| {
        panic!("0xF4 is not a valid opcode");
    },
    |_f5, gb| {
        let af = gb.af();
        gb.stack_push(af);
        op_execution!{
            cycles: 4;
            asm: "PUSH AF";
            trace: "SP₁ = ${:04x}, AF = ${:04x}", gb.cpu.sp, af;
        }
    },
    |_f6, _gb| unimplemented!("opcode 0xF6 not implemented"),
    |_f7, _gb| unimplemented!("opcode 0xF7 not implemented"),
    |_f8, _gb| unimplemented!("opcode 0xF8 not implemented"),
    |_f9, _gb| unimplemented!("opcode 0xF9 not implemented"),
    |_fa, gb| {
        let nn = gb.read_immediate_u16();
        let a0 = gb.cpu.a;
        let a1 = gb.get(nn);
        gb.cpu.a = a1;
        op_execution!{
            cycles: 4;
            asm: "LD A, (${:04x})", nn;
            trace: "A₀ = ${:02x}, A₁ = ${:04x}", a0, a1;
        }
    },
    |_fb, _gb| unimplemented!("opcode 0xFB not implemented"),
    |_fc, _gb| {
        panic!("0xFC is not a valid opcode");
    },
    |_fd, _gb| {
        panic!("0xFD is not a valid opcode");
    },
    |_fe, _gb| unimplemented!("opcode 0xFE not implemented"),
];
