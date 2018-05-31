use super::operation;

// one-byte opcodes
pub static OPCODES: [operation::OpFn; 0xFF] = [
    |_00, _cpu, _mem| {
        op_execution!{
            cycles: 1;
            asm: "NOP";
        }
    },
    |_01, _cpu, _mem| unimplemented!("opcode 0x01 not implemented"),
    |_02, _cpu, _mem| unimplemented!("opcode 0x02 not implemented"),
    |_03, _cpu, _mem| unimplemented!("opcode 0x03 not implemented"),
    |_04, _cpu, _mem| unimplemented!("opcode 0x04 not implemented"),
    |_05, _cpu, _mem| unimplemented!("opcode 0x05 not implemented"),
    |_06, cpu, mem| {
        let b0 = cpu.b;
        let b1 = cpu.read_immediate_u8(mem);
        cpu.b = b1;
        op_execution!{
            cycles: 2;
            asm: "LD B, ${:02x}", b1;
            debug: "B₀ = ${:02x}, B₁ = ${:02x}", b0, b1;
        }
    },
    |_07, _cpu, _mem| unimplemented!("opcode 0x07 not implemented"),
    |_08, _cpu, _mem| unimplemented!("opcode 0x08 not implemented"),
    |_09, _cpu, _mem| unimplemented!("opcode 0x09 not implemented"),
    |_0a, cpu, mem| {
        let a0 = cpu.a;
        let bc = cpu.bc();
        let a1 = mem.get(bc);
        cpu.a = a1;
        op_execution!{
            cycles: 2;
            asm: "LD A, (BC)";
            debug: "A₀ = ${:02x}, BC = ${:04x}, (BC) = ${:02x}", a0, bc, a1;
        }
    },
    |_0b, _cpu, _mem| unimplemented!("opcode 0x0B not implemented"),
    |_0c, _cpu, _mem| unimplemented!("opcode 0x0C not implemented"),
    |_0d, _cpu, _mem| unimplemented!("opcode 0x0D not implemented"),
    |_0e, cpu, mem| {
        let c0 = cpu.c;
        let c1 = cpu.read_immediate_u8(mem);
        cpu.c = c1;
        op_execution!{
            cycles: 2;
            asm: "LD C, ${:02x}", c1;
            debug: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1;
        }
    },
    |_0f, _cpu, _mem| unimplemented!("opcode 0x0F not implemented"),
    |_10, _cpu, _mem| unimplemented!("opcode 0x10 not implemented"),
    |_11, _cpu, _mem| unimplemented!("opcode 0x11 not implemented"),
    |_12, _cpu, _mem| unimplemented!("opcode 0x12 not implemented"),
    |_13, _cpu, _mem| unimplemented!("opcode 0x13 not implemented"),
    |_14, _cpu, _mem| unimplemented!("opcode 0x14 not implemented"),
    |_15, _cpu, _mem| unimplemented!("opcode 0x15 not implemented"),
    |_16, cpu, mem| {
        let d0 = cpu.d;
        let d1 = cpu.read_immediate_u8(mem);
        cpu.d = d1;
        op_execution!{
            cycles: 2;
            asm: "LD D, ${:02x}", d1;
            debug: "D₀ = ${:02x}, D₁ = ${:02x}", d0, d1;
        }
    },
    |_17, _cpu, _mem| unimplemented!("opcode 0x17 not implemented"),
    |_18, _cpu, _mem| unimplemented!("opcode 0x18 not implemented"),
    |_19, _cpu, _mem| unimplemented!("opcode 0x19 not implemented"),
    |_1a, cpu, mem| {
        let a0 = cpu.a;
        let de = cpu.de();
        let a1 = mem.get(de);
        cpu.a = a1;
        op_execution!{
            cycles: 2;
            asm: "LD A, (DE)";
            debug: "A₀ = ${:02x}, DE = ${:04x}, (DE) = ${:02x}", a0, de, a1;
        }
    },
    |_1b, _cpu, _mem| unimplemented!("opcode 0x1B not implemented"),
    |_1c, _cpu, _mem| unimplemented!("opcode 0x1C not implemented"),
    |_1d, _cpu, _mem| unimplemented!("opcode 0x1D not implemented"),
    |_1e, cpu, mem| {
        let e0 = cpu.e;
        let e1 = cpu.read_immediate_u8(mem);
        cpu.e = e1;
        op_execution!{
            cycles: 2;
            asm: "LD E, ${:02x}", e1;
            debug: "E₀ = ${:02x}, E₁ = ${:02x}", e0, e1;
        }
    },
    |_1f, _cpu, _mem| unimplemented!("opcode 0x1F not implemented"),
    |_20, _cpu, _mem| unimplemented!("opcode 0x20 not implemented"),
    |_21, cpu, mem| {
        let hl0 = cpu.hl();
        let hl1 = cpu.read_immediate_u16(mem);
        cpu.set_hl(hl1);
        op_execution!{
            cycles: 3;
            asm: "LOAD HL, ${:04x}", hl1;
            debug: "hl₁ = ${:04x}", hl0;
        }
    },
    |_22, _cpu, _mem| unimplemented!("opcode 0x22 not implemented"),
    |_23, _cpu, _mem| unimplemented!("opcode 0x23 not implemented"),
    |_24, _cpu, _mem| unimplemented!("opcode 0x24 not implemented"),
    |_25, _cpu, _mem| unimplemented!("opcode 0x25 not implemented"),
    |_26, cpu, mem| {
        let h0 = cpu.h;
        let h1 = cpu.read_immediate_u8(mem);
        cpu.h = h1;
        op_execution!{
            cycles: 2;
            asm: "LD H, ${:02x}", h1;
            debug: "H₀ = ${:02x}, H₁ = ${:02x}", h0, h1;
        }
    },
    |_27, _cpu, _mem| unimplemented!("opcode 0x27 not implemented"),
    |_28, _cpu, _mem| unimplemented!("opcode 0x28 not implemented"),
    |_29, _cpu, _mem| unimplemented!("opcode 0x29 not implemented"),
    |_2a, _cpu, _mem| unimplemented!("opcode 0x2A not implemented"),
    |_2b, _cpu, _mem| unimplemented!("opcode 0x2B not implemented"),
    |_2c, _cpu, _mem| unimplemented!("opcode 0x2C not implemented"),
    |_2d, _cpu, _mem| unimplemented!("opcode 0x2D not implemented"),
    |_2e, cpu, mem| {
        let l0 = cpu.l;
        let l1 = cpu.read_immediate_u8(mem);
        cpu.l = l1;
        op_execution!{
            cycles: 2;
            asm: "LD L, ${:02x}", l1;
            debug: "L₀ = ${:02x}, L₁ = ${:02x}", l0, l1;
        }
    },
    |_2f, _cpu, _mem| unimplemented!("opcode 0x2F not implemented"),
    |_30, _cpu, _mem| unimplemented!("opcode 0x30 not implemented"),
    |_31, cpu, mem| {
        let sp0 = cpu.sp;
        let sp1 = cpu.read_immediate_u16(mem);
        cpu.sp = sp1;
        op_execution!{
            cycles: 3;
            asm: "LOAD SP, ${:04x}", sp1;
            debug: "SP₀ = ${:04x}", sp0;
        }
    },
    |_32, _cpu, _mem| unimplemented!("opcode 0x32 not implemented"),
    |_33, _cpu, _mem| unimplemented!("opcode 0x33 not implemented"),
    |_34, _cpu, _mem| unimplemented!("opcode 0x34 not implemented"),
    |_35, _cpu, _mem| unimplemented!("opcode 0x35 not implemented"),
    |_36, _cpu, _mem| unimplemented!("opcode 0x36 not implemented"),
    |_37, _cpu, _mem| unimplemented!("opcode 0x37 not implemented"),
    |_38, _cpu, _mem| unimplemented!("opcode 0x38 not implemented"),
    |_39, _cpu, _mem| unimplemented!("opcode 0x39 not implemented"),
    |_3a, _cpu, _mem| unimplemented!("opcode 0x3A not implemented"),
    |_3b, _cpu, _mem| unimplemented!("opcode 0x3B not implemented"),
    |_3c, _cpu, _mem| unimplemented!("opcode 0x3C not implemented"),
    |_3d, _cpu, _mem| unimplemented!("opcode 0x3D not implemented"),
    |_3e, cpu, mem| {
        let n = cpu.read_immediate_u8(mem);
        let a0 = cpu.a;
        cpu.a = n;
        op_execution!{
            cycles: 2;
            asm: "LD A, ${:02x}", n;
            debug: "A₀ = ${:02x}", a0;
        }
    },
    |_3f, _cpu, _mem| unimplemented!("opcode 0x3F not implemented"),
    |_40, cpu, _mem| {
        let b = cpu.b;
        op_execution!{
            cycles: 1;
            asm: "LD B, B";
            debug: "B = ${:02x}", b;
        }
    },
    |_41, cpu, _mem| {
        let b0 = cpu.b;
        let c = cpu.c;
        cpu.b = c;
        op_execution!{
            cycles: 1;
            asm: "LD B, C";
            debug: "B₀ = ${:02x}, C = ${:02x}", b0, c;
        }
    },
    |_42, cpu, _mem| {
        let b0 = cpu.b;
        let d = cpu.d;
        cpu.b = d;
        op_execution!{
            cycles: 1;
            asm: "LD B, D";
            debug: "B₀ = ${:02x}, D = ${:02x}", b0, d;
        }
    },
    |_43, cpu, _mem| {
        let b0 = cpu.b;
        let e = cpu.e;
        cpu.b = e;
        op_execution!{
            cycles: 1;
            asm: "LD B, E";
            debug: "B₀ = ${:02x}, E = ${:02x}", b0, e;
        }
    },
    |_44, cpu, _mem| {
        let b0 = cpu.b;
        let h = cpu.h;
        cpu.b = h;
        op_execution!{
            cycles: 1;
            asm: "LD B, H";
            debug: "B₀ = ${:02x}, H = ${:02x}", b0, h;
        }
    },
    |_45, cpu, _mem| {
        let b0 = cpu.b;
        let l = cpu.l;
        cpu.b = l;
        op_execution!{
            cycles: 1;
            asm: "LD B, L";
            debug: "B₀ = ${:02x}, L = ${:02x}", b0, l;
        }
    },
    |_46, cpu, mem| {
        let b0 = cpu.b;
        let hl = cpu.hl();
        let b1 = mem.get(hl);
        cpu.b = b1;
        op_execution!{
            cycles: 2;
            asm: "LD B, (HL)";
            debug: "B₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", b0, hl, b1;
        }
    },
    |_47, _cpu, _mem| unimplemented!("opcode 0x47 not implemented"),
    |_48, cpu, _mem| {
        let c0 = cpu.c;
        let b = cpu.b;
        cpu.c = b;
        op_execution!{
            cycles: 1;
            asm: "LD C, B";
            debug: "C₀ = ${:02x}, B = ${:02x}", c0, b;
        }
    },
    |_49, cpu, _mem| {
        let c = cpu.c;
        op_execution!{
            cycles: 1;
            asm: "LD C, C";
            debug: "C = ${:02x}", c;
        }
    },
    |_4a, cpu, _mem| {
        let c0 = cpu.c;
        let d = cpu.d;
        cpu.c = d;
        op_execution!{
            cycles: 1;
            asm: "LD C, D";
            debug: "C₀ = ${:02x}, D = ${:02x}", c0, d;
        }
    },
    |_4b, cpu, _mem| {
        let c0 = cpu.c;
        let e = cpu.e;
        cpu.c = e;
        op_execution!{
            cycles: 1;
            asm: "LD C, E";
            debug: "C₀ = ${:02x}, E = ${:02x}", c0, e;
        }
    },
    |_4c, cpu, _mem| {
        let c0 = cpu.c;
        let h = cpu.h;
        cpu.c = h;
        op_execution!{
            cycles: 1;
            asm: "LD C, H";
            debug: "C₀ = ${:02x}, H = ${:02x}", c0, h;
        }
    },
    |_4d, cpu, _mem| {
        let c0 = cpu.c;
        let l = cpu.l;
        cpu.c = l;
        op_execution!{
            cycles: 1;
            asm: "LD C, L";
            debug: "C₀ = ${:02x}, L = ${:02x}", c0, l;
        }
    },
    |_4e, cpu, mem| {
        let c0 = cpu.c;
        let hl = cpu.hl();
        let c1 = mem.get(hl);
        cpu.c = c1;
        op_execution!{
            cycles: 2;
            asm: "LD C, (HL)";
            debug: "C₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", c0, hl, c1;
        }
    },
    |_4f, _cpu, _mem| unimplemented!("opcode 0x4F not implemented"),
    |_50, _cpu, _mem| unimplemented!("opcode 0x50 not implemented"),
    |_51, _cpu, _mem| unimplemented!("opcode 0x51 not implemented"),
    |_52, _cpu, _mem| unimplemented!("opcode 0x52 not implemented"),
    |_53, _cpu, _mem| unimplemented!("opcode 0x53 not implemented"),
    |_54, _cpu, _mem| unimplemented!("opcode 0x54 not implemented"),
    |_55, _cpu, _mem| unimplemented!("opcode 0x55 not implemented"),
    |_56, _cpu, _mem| unimplemented!("opcode 0x56 not implemented"),
    |_57, _cpu, _mem| unimplemented!("opcode 0x57 not implemented"),
    |_58, _cpu, _mem| unimplemented!("opcode 0x58 not implemented"),
    |_59, _cpu, _mem| unimplemented!("opcode 0x59 not implemented"),
    |_5a, _cpu, _mem| unimplemented!("opcode 0x5A not implemented"),
    |_5b, _cpu, _mem| unimplemented!("opcode 0x5B not implemented"),
    |_5c, _cpu, _mem| unimplemented!("opcode 0x5C not implemented"),
    |_5d, _cpu, _mem| unimplemented!("opcode 0x5D not implemented"),
    |_5e, _cpu, _mem| unimplemented!("opcode 0x5E not implemented"),
    |_5f, _cpu, _mem| unimplemented!("opcode 0x5F not implemented"),
    |_60, _cpu, _mem| unimplemented!("opcode 0x60 not implemented"),
    |_61, _cpu, _mem| unimplemented!("opcode 0x61 not implemented"),
    |_62, _cpu, _mem| unimplemented!("opcode 0x62 not implemented"),
    |_63, _cpu, _mem| unimplemented!("opcode 0x63 not implemented"),
    |_64, _cpu, _mem| unimplemented!("opcode 0x64 not implemented"),
    |_65, _cpu, _mem| unimplemented!("opcode 0x65 not implemented"),
    |_66, _cpu, _mem| unimplemented!("opcode 0x66 not implemented"),
    |_67, _cpu, _mem| unimplemented!("opcode 0x67 not implemented"),
    |_68, _cpu, _mem| unimplemented!("opcode 0x68 not implemented"),
    |_69, _cpu, _mem| unimplemented!("opcode 0x69 not implemented"),
    |_6a, _cpu, _mem| unimplemented!("opcode 0x6A not implemented"),
    |_6b, _cpu, _mem| unimplemented!("opcode 0x6B not implemented"),
    |_6c, _cpu, _mem| unimplemented!("opcode 0x6C not implemented"),
    |_6d, _cpu, _mem| unimplemented!("opcode 0x6D not implemented"),
    |_6e, _cpu, _mem| unimplemented!("opcode 0x6E not implemented"),
    |_6f, _cpu, _mem| unimplemented!("opcode 0x6F not implemented"),
    |_70, _cpu, _mem| unimplemented!("opcode 0x70 not implemented"),
    |_71, _cpu, _mem| unimplemented!("opcode 0x71 not implemented"),
    |_72, _cpu, _mem| unimplemented!("opcode 0x72 not implemented"),
    |_73, _cpu, _mem| unimplemented!("opcode 0x73 not implemented"),
    |_74, _cpu, _mem| unimplemented!("opcode 0x74 not implemented"),
    |_75, _cpu, _mem| unimplemented!("opcode 0x75 not implemented"),
    |_76, _cpu, _mem| unimplemented!("opcode 0x76 not implemented"),
    |_77, _cpu, _mem| unimplemented!("opcode 0x77 not implemented"),
    |_78, cpu, _mem| {
        let a0 = cpu.a;
        let b = cpu.b;
        cpu.a = b;
        op_execution!{
            cycles: 1;
            asm: "LD A, B";
            debug: "A₀ = ${:02x}, B = ${:02x}", a0, b;
        }
    },
    |_79, cpu, _mem| {
        let a0 = cpu.a;
        let c = cpu.c;
        cpu.a = c;
        op_execution!{
            cycles: 1;
            asm: "LD A, C";
            debug: "A₀ = ${:02x}, C = ${:02x}", a0, c;
        }
    },
    |_7a, cpu, _mem| {
        let a0 = cpu.a;
        let d = cpu.d;
        cpu.a = d;
        op_execution!{
            cycles: 1;
            asm: "LD A, D";
            debug: "A₀ = ${:02x}, D = ${:02x}", a0, d;
        }
    },
    |_7b, cpu, _mem| {
        let a0 = cpu.a;
        let e = cpu.e;
        cpu.a = e;
        op_execution!{
            cycles: 1;
            asm: "LD A, E";
            debug: "A₀ = ${:02x}, E = ${:02x}", a0, e;
        }
    },
    |_7c, cpu, _mem| {
        let a0 = cpu.a;
        let h = cpu.h;
        cpu.a = h;
        op_execution!{
            cycles: 1;
            asm: "LD A, H";
            debug: "A₀ = ${:02x}, H = ${:02x}", a0, h;
        }
    },
    |_7d, cpu, _mem| {
        let a0 = cpu.a;
        let l = cpu.l;
        cpu.a = l;
        op_execution!{
            cycles: 1;
            asm: "LD A, L";
            debug: "A₀ = ${:02x}, L = ${:02x}", a0, l;
        }
    },
    |_7e, cpu, mem| {
        let a0 = cpu.a;
        let hl = cpu.hl();
        let a1 = mem.get(hl);
        cpu.a = a1;
        op_execution!{
            cycles: 2;
            asm: "LD A, (HL)";
            debug: "A₀ = ${:02x}, HL = ${:04x}, (HL) = ${:02x}", a0, hl, a1;
        }
    },
    |_7f, cpu, _mem| {
        let a = cpu.a;
        op_execution!{
            cycles: 1;
            asm: "LD A, A";
            debug: "A = ${:02x}", a;
        }
    },
    |_80, _cpu, _mem| unimplemented!("opcode 0x80 not implemented"),
    |_81, _cpu, _mem| unimplemented!("opcode 0x81 not implemented"),
    |_82, _cpu, _mem| unimplemented!("opcode 0x82 not implemented"),
    |_83, _cpu, _mem| unimplemented!("opcode 0x83 not implemented"),
    |_84, _cpu, _mem| unimplemented!("opcode 0x84 not implemented"),
    |_85, _cpu, _mem| unimplemented!("opcode 0x85 not implemented"),
    |_86, _cpu, _mem| unimplemented!("opcode 0x86 not implemented"),
    |_87, _cpu, _mem| unimplemented!("opcode 0x87 not implemented"),
    |_88, _cpu, _mem| unimplemented!("opcode 0x88 not implemented"),
    |_89, _cpu, _mem| unimplemented!("opcode 0x89 not implemented"),
    |_8a, _cpu, _mem| unimplemented!("opcode 0x8A not implemented"),
    |_8b, _cpu, _mem| unimplemented!("opcode 0x8B not implemented"),
    |_8c, _cpu, _mem| unimplemented!("opcode 0x8C not implemented"),
    |_8d, _cpu, _mem| unimplemented!("opcode 0x8D not implemented"),
    |_8e, _cpu, _mem| unimplemented!("opcode 0x8E not implemented"),
    |_8f, _cpu, _mem| unimplemented!("opcode 0x8F not implemented"),
    |_90, _cpu, _mem| unimplemented!("opcode 0x90 not implemented"),
    |_91, _cpu, _mem| unimplemented!("opcode 0x91 not implemented"),
    |_92, _cpu, _mem| unimplemented!("opcode 0x92 not implemented"),
    |_93, _cpu, _mem| unimplemented!("opcode 0x93 not implemented"),
    |_94, _cpu, _mem| unimplemented!("opcode 0x94 not implemented"),
    |_95, _cpu, _mem| unimplemented!("opcode 0x95 not implemented"),
    |_96, _cpu, _mem| unimplemented!("opcode 0x96 not implemented"),
    |_97, _cpu, _mem| unimplemented!("opcode 0x97 not implemented"),
    |_98, _cpu, _mem| unimplemented!("opcode 0x98 not implemented"),
    |_99, _cpu, _mem| unimplemented!("opcode 0x99 not implemented"),
    |_9a, _cpu, _mem| unimplemented!("opcode 0x9A not implemented"),
    |_9b, _cpu, _mem| unimplemented!("opcode 0x9B not implemented"),
    |_9c, _cpu, _mem| unimplemented!("opcode 0x9C not implemented"),
    |_9d, _cpu, _mem| unimplemented!("opcode 0x9D not implemented"),
    |_9e, _cpu, _mem| unimplemented!("opcode 0x9E not implemented"),
    |_9f, _cpu, _mem| unimplemented!("opcode 0x9F not implemented"),
    |_a0, _cpu, _mem| unimplemented!("opcode 0xA0 not implemented"),
    |_a1, _cpu, _mem| unimplemented!("opcode 0xA1 not implemented"),
    |_a2, _cpu, _mem| unimplemented!("opcode 0xA2 not implemented"),
    |_a3, _cpu, _mem| unimplemented!("opcode 0xA3 not implemented"),
    |_a4, _cpu, _mem| unimplemented!("opcode 0xA4 not implemented"),
    |_a5, _cpu, _mem| unimplemented!("opcode 0xA5 not implemented"),
    |_a6, _cpu, _mem| unimplemented!("opcode 0xA6 not implemented"),
    |_a7, _cpu, _mem| unimplemented!("opcode 0xA7 not implemented"),
    |_a8, cpu, _mem| {
        let b = cpu.b;
        let a0 = cpu.a;
        let a1 = a0 ^ b;
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR B";
            debug: "A₀ = ${:02x}, B = ${:02x} A₁ = ${:02x}", a0, b, a1;
        }
    },
    |_a9, cpu, _mem| {
        let c = cpu.c;
        let a0 = cpu.a;
        let a1 = a0 ^ c;
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR C";
            debug: "A₀ = ${:02x}, C = ${:02x} A₁ = ${:02x}", a0, c, a1;
        }
    },
    |_aa, cpu, _mem| {
        let d = cpu.d;
        let a0 = cpu.a;
        let a1 = a0 ^ d;
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR D";
            debug: "A₀ = ${:02x}, D = ${:02x} A₁ = ${:02x}", a0, d, a1;
        }
    },
    |_ab, cpu, _mem| {
        let e = cpu.e;
        let a0 = cpu.a;
        let a1 = a0 ^ e;
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR E";
            debug: "A₀ = ${:02x}, E = ${:02x} A₁ = ${:02x}", a0, e, a1;
        }
    },
    |_ac, cpu, _mem| {
        let h = cpu.h;
        let a0 = cpu.a;
        let a1 = a0 ^ h;
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR B";
            debug: "A₀ = ${:02x}, H = ${:02x} A₁ = ${:02x}", a0, h, a1;
        }
    },
    |_ad, cpu, _mem| {
        let l = cpu.l;
        let a0 = cpu.a;
        let a1 = a0 ^ l;
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR L";
            debug: "A₀ = ${:02x}, L = ${:02x} A₁ = ${:02x}", a0, l, a1;
        }
    },
    |_ae, _cpu, _mem| unimplemented!("opcode 0xAE not implemented"),
    |_af, cpu, _mem| {
        let a0 = cpu.a;
        let a1 = a0 ^ a0;
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        cpu.set_c_flag(false);
        op_execution!{
            cycles: 1;
            asm: "XOR A";
            debug: "A₀ = ${:02x}, A₁ = ${:02x}", a0, a1;
        }
    },
    |_b0, _cpu, _mem| unimplemented!("opcode 0xB0 not implemented"),
    |_b1, _cpu, _mem| unimplemented!("opcode 0xB1 not implemented"),
    |_b2, _cpu, _mem| unimplemented!("opcode 0xB2 not implemented"),
    |_b3, _cpu, _mem| unimplemented!("opcode 0xB3 not implemented"),
    |_b4, _cpu, _mem| unimplemented!("opcode 0xB4 not implemented"),
    |_b5, _cpu, _mem| unimplemented!("opcode 0xB5 not implemented"),
    |_b6, _cpu, _mem| unimplemented!("opcode 0xB6 not implemented"),
    |_b7, _cpu, _mem| unimplemented!("opcode 0xB7 not implemented"),
    |_b8, _cpu, _mem| unimplemented!("opcode 0xB8 not implemented"),
    |_b9, _cpu, _mem| unimplemented!("opcode 0xB9 not implemented"),
    |_ba, _cpu, _mem| unimplemented!("opcode 0xBA not implemented"),
    |_bb, _cpu, _mem| unimplemented!("opcode 0xBB not implemented"),
    |_bc, _cpu, _mem| unimplemented!("opcode 0xBC not implemented"),
    |_bd, _cpu, _mem| unimplemented!("opcode 0xBD not implemented"),
    |_be, _cpu, _mem| unimplemented!("opcode 0xBE not implemented"),
    |_bf, _cpu, _mem| unimplemented!("opcode 0xBF not implemented"),
    |_c0, _cpu, _mem| unimplemented!("opcode 0xC0 not implemented"),
    |_c1, _cpu, _mem| unimplemented!("opcode 0xC1 not implemented"),
    |_c2, _cpu, _mem| unimplemented!("opcode 0xC2 not implemented"),
    |_c3, _cpu, _mem| unimplemented!("opcode 0xC3 not implemented"),
    |_c4, _cpu, _mem| unimplemented!("opcode 0xC4 not implemented"),
    |_c5, _cpu, _mem| unimplemented!("opcode 0xC5 not implemented"),
    |_c6, _cpu, _mem| unimplemented!("opcode 0xC6 not implemented"),
    |_c7, _cpu, _mem| unimplemented!("opcode 0xC7 not implemented"),
    |_c8, _cpu, _mem| unimplemented!("opcode 0xC8 not implemented"),
    |_c9, _cpu, _mem| unimplemented!("opcode 0xC9 not implemented"),
    |_ca, _cpu, _mem| unimplemented!("opcode 0xCA not implemented"),
    |_cb, _cpu, _mem| {
        panic!("0xCB prefix is not a complete opcode");
    },
    |_cc, _cpu, _mem| unimplemented!("opcode 0xCC not implemented"),
    |_cd, _cpu, _mem| unimplemented!("opcode 0xCD not implemented"),
    |_ce, _cpu, _mem| unimplemented!("opcode 0xCE not implemented"),
    |_cf, _cpu, _mem| unimplemented!("opcode 0xCF not implemented"),
    |_d0, _cpu, _mem| unimplemented!("opcode 0xD0 not implemented"),
    |_d1, _cpu, _mem| unimplemented!("opcode 0xD1 not implemented"),
    |_d2, _cpu, _mem| unimplemented!("opcode 0xD2 not implemented"),
    |_d3, _cpu, _mem| {
        panic!("0xD3 is not a valid opcode");
    },
    |_d4, _cpu, _mem| unimplemented!("opcode 0xD4 not implemented"),
    |_d5, _cpu, _mem| unimplemented!("opcode 0xD5 not implemented"),
    |_d6, _cpu, _mem| unimplemented!("opcode 0xD6 not implemented"),
    |_d7, _cpu, _mem| unimplemented!("opcode 0xD7 not implemented"),
    |_d8, _cpu, _mem| unimplemented!("opcode 0xD8 not implemented"),
    |_d9, _cpu, _mem| unimplemented!("opcode 0xD9 not implemented"),
    |_da, _cpu, _mem| unimplemented!("opcode 0xDA not implemented"),
    |_db, _cpu, _mem| {
        panic!("0xDB is not a valid opcode");
    },
    |_dc, _cpu, _mem| unimplemented!("opcode 0xDC not implemented"),
    |_dd, _cpu, _mem| {
        panic!("0xDD is not a valid opcode");
    },
    |_de, _cpu, _mem| unimplemented!("opcode 0xDE not implemented"),
    |_df, _cpu, _mem| unimplemented!("opcode 0xDF not implemented"),
    |_e0, _cpu, _mem| unimplemented!("opcode 0xE0 not implemented"),
    |_e1, _cpu, _mem| unimplemented!("opcode 0xE1 not implemented"),
    |_e2, _cpu, _mem| unimplemented!("opcode 0xE2 not implemented"),
    |_e3, _cpu, _mem| {
        panic!("0xE3 is not a valid opcode");
    },
    |_e4, _cpu, _mem| {
        panic!("0xE4 is not a valid opcode");
    },
    |_e5, _cpu, _mem| unimplemented!("opcode 0xE5 not implemented"),
    |_e6, _cpu, _mem| unimplemented!("opcode 0xE6 not implemented"),
    |_e7, _cpu, _mem| unimplemented!("opcode 0xE7 not implemented"),
    |_e8, _cpu, _mem| unimplemented!("opcode 0xE8 not implemented"),
    |_e9, _cpu, _mem| unimplemented!("opcode 0xE9 not implemented"),
    |_ea, _cpu, _mem| unimplemented!("opcode 0xEA not implemented"),
    |_eb, _cpu, _mem| {
        panic!("0xEB is not a valid opcode");
    },
    |_ec, _cpu, _mem| {
        panic!("0xEC is not a valid opcode");
    },
    |_ed, _cpu, _mem| {
        panic!("0xED is not a valid opcode");
    },
    |_ee, _cpu, _mem| unimplemented!("opcode 0xEE not implemented"),
    |_ef, _cpu, _mem| unimplemented!("opcode 0xEF not implemented"),
    |_f0, _cpu, _mem| unimplemented!("opcode 0xF0 not implemented"),
    |_f1, _cpu, _mem| unimplemented!("opcode 0xF1 not implemented"),
    |_f2, _cpu, _mem| unimplemented!("opcode 0xF2 not implemented"),
    |_f3, _cpu, _mem| unimplemented!("opcode 0xF3 not implemented"),
    |_f4, _cpu, _mem| {
        panic!("0xF4 is not a valid opcode");
    },
    |_f5, _cpu, _mem| unimplemented!("opcode 0xF5 not implemented"),
    |_f6, _cpu, _mem| unimplemented!("opcode 0xF6 not implemented"),
    |_f7, _cpu, _mem| unimplemented!("opcode 0xF7 not implemented"),
    |_f8, _cpu, _mem| unimplemented!("opcode 0xF8 not implemented"),
    |_f9, _cpu, _mem| unimplemented!("opcode 0xF9 not implemented"),
    |_fa, cpu, mem| {
        let nn = cpu.read_immediate_u16(mem);
        let a0 = cpu.a;
        let a1 = mem.get(nn);
        cpu.a = a1;
        op_execution!{
            cycles: 4;
            asm: "LD A, (${:04x})", nn;
            debug: "A₀ = ${:02x}, A₁ = ${:04x}", a0, a1;
        }
    },
    |_fb, _cpu, _mem| unimplemented!("opcode 0xFB not implemented"),
    |_fc, _cpu, _mem| {
        panic!("0xFC is not a valid opcode");
    },
    |_fd, _cpu, _mem| {
        panic!("0xFD is not a valid opcode");
    },
    |_fe, _cpu, _mem| unimplemented!("opcode 0xFE not implemented"),
];
