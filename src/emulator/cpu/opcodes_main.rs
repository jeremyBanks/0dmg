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
    |_05, cpu, _mem| {
        let b0 = cpu.b;
        let b1 = b0 - 1;
        cpu.b = b1;
        cpu.set_z_flag(b1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(b0 < b1);
        op_execution!{
            cycles: 1;
            asm: "DEC A";
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", b0, b1;
        }
    },
    |_06, cpu, mem| {
        let b0 = cpu.b;
        let b1 = cpu.read_immediate_u8(mem);
        cpu.b = b1;
        op_execution!{
            cycles: 2;
            asm: "LD B, ${:02x}", b1;
            trace: "B₀ = ${:02x}, B₁ = ${:02x}", b0, b1;
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
            trace: "A₀ = ${:02x}, BC = ${:04x}, (BC) = ${:02x}", a0, bc, a1;
        }
    },
    |_0b, _cpu, _mem| unimplemented!("opcode 0x0B not implemented"),
    |_0c, cpu, _mem| {
        let c0 = cpu.c;
        let c1 = c0 + 1;
        cpu.c = c1;
        cpu.set_z_flag(c1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(c0 > c1);
        op_execution!{
            cycles: 1;
            asm: "INC C";
            trace: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1;
        }
    },
    |_0d, cpu, _mem| {
        let c0 = cpu.a;
        let c1 = c0 - 1;
        cpu.c = c1;
        cpu.set_z_flag(c1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(c0 < c1);
        op_execution!{
            cycles: 1;
            asm: "DEC C";
            trace: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1;
        }
    },
    |_0e, cpu, mem| {
        let c0 = cpu.c;
        let c1 = cpu.read_immediate_u8(mem);
        cpu.c = c1;
        op_execution!{
            cycles: 2;
            asm: "LD C, ${:02x}", c1;
            trace: "C₀ = ${:02x}, C₁ = ${:02x}", c0, c1;
        }
    },
    |_0f, _cpu, _mem| unimplemented!("opcode 0x0F not implemented"),
    |_10, _cpu, _mem| unimplemented!("opcode 0x10 not implemented"),
    |_11, cpu, mem| {
        let de0 = cpu.de();
        let de1 = cpu.read_immediate_u16(mem);
        cpu.set_de(de1);
        op_execution!{
            cycles: 3;
            asm: "LOAD DE, ${:04x}", de1;
            trace: "DE₁ = ${:04x}", de0;
        }
    },
    |_12, _cpu, _mem| unimplemented!("opcode 0x12 not implemented"),
    |_13, _cpu, _mem| unimplemented!("opcode 0x13 not implemented"),
    |_14, _cpu, _mem| unimplemented!("opcode 0x14 not implemented"),
    |_15, cpu, _mem| {
        let d0 = cpu.d;
        let d1 = d0 - 1;
        cpu.d = d1;
        cpu.set_z_flag(d1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(d0 < d1);
        op_execution!{
            cycles: 1;
            asm: "DEC D";
            trace: "D₀ = ${:02x}, D₁ = ${:02x}", d0, d1;
        }
    },
    |_16, cpu, mem| {
        let d0 = cpu.d;
        let d1 = cpu.read_immediate_u8(mem);
        cpu.d = d1;
        op_execution!{
            cycles: 2;
            asm: "LD D, ${:02x}", d1;
            trace: "D₀ = ${:02x}, D₁ = ${:02x}", d0, d1;
        }
    },
    |_17, cpu, _mem| {
        let a0 = cpu.a;
        let a1 = a0 << 1 + if cpu.c_flag() { 1 } else { 0 };
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_c_flag(a0 & 0b10000000 > 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL A";
            trace: "A₀ = {}", a0;
        }
    },
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
            trace: "A₀ = ${:02x}, DE = ${:04x}, (DE) = ${:02x}", a0, de, a1;
        }
    },
    |_1b, _cpu, _mem| unimplemented!("opcode 0x1B not implemented"),
    |_1c, _cpu, _mem| unimplemented!("opcode 0x1C not implemented"),
    |_1d, cpu, _mem| {
        let e0 = cpu.e;
        let e1 = e0 - 1;
        cpu.e = e1;
        cpu.set_z_flag(e1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(e0 < e1);
        op_execution!{
            cycles: 1;
            asm: "DEC A";
            trace: "E₀ = ${:02x}, E₁ = ${:02x}", e0, e1;
        }
    },
    |_1e, cpu, mem| {
        let e0 = cpu.e;
        let e1 = cpu.read_immediate_u8(mem);
        cpu.e = e1;
        op_execution!{
            cycles: 2;
            asm: "LD E, ${:02x}", e1;
            trace: "E₀ = ${:02x}, E₁ = ${:02x}", e0, e1;
        }
    },
    |_1f, _cpu, _mem| unimplemented!("opcode 0x1F not implemented"),
    |_20, cpu, mem| {
        let n = cpu.read_immediate_i8(mem);
        let z_flag = cpu.z_flag();
        if z_flag == false {
            cpu.relative_jump(n);
        }
        op_execution!{
            cycles: 2;
            asm: "JR NZ, {}", n;
            trace: "Z = {}", z_flag;
        }
    },
    |_21, cpu, mem| {
        let _hl0 = cpu.hl();
        let hl1 = cpu.read_immediate_u16(mem);
        cpu.set_hl(hl1);
        op_execution!{
            cycles: 3;
            asm: "LOAD HL, ${:04x}", hl1;
            trace: "HL₁ = ${:04x}", hl1;
        }
    },
    |_22, cpu, mem| {
        let a = cpu.a;
        let hl0 = cpu.hl();
        let hl1 = hl0 + 1;
        mem.set(hl0, a);
        cpu.set_hl(hl1);
        op_execution!{
            cycles: 2;
            asm: "LD (HL+), A";
            trace: "HL₀ = ${:04x}, A = ${:02x}", hl0, a;
        }
    },
    |_23, _cpu, _mem| unimplemented!("opcode 0x23 not implemented"),
    |_24, _cpu, _mem| unimplemented!("opcode 0x24 not implemented"),
    |_25, cpu, _mem| {
        let h0 = cpu.h;
        let h1 = h0 - 1;
        cpu.h = h1;
        cpu.set_z_flag(h1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(h0 < h1);
        op_execution!{
            cycles: 1;
            asm: "DEC H";
            trace: "H₀ = ${:02x}, H₁ = ${:02x}", h0, h1;
        }
    },
    |_26, cpu, mem| {
        let h0 = cpu.h;
        let h1 = cpu.read_immediate_u8(mem);
        cpu.h = h1;
        op_execution!{
            cycles: 2;
            asm: "LD H, ${:02x}", h1;
            trace: "H₀ = ${:02x}, H₁ = ${:02x}", h0, h1;
        }
    },
    |_27, _cpu, _mem| unimplemented!("opcode 0x27 not implemented"),
    |_28, cpu, mem| {
        let n = cpu.read_immediate_i8(mem);
        let z_flag = cpu.z_flag();
        if z_flag {
            cpu.relative_jump(n);
        }
        op_execution!{
            cycles: 2;
            asm: "JR Z, {}", n;
            trace: "Z = {}", z_flag;
        }
    },
    |_29, _cpu, _mem| unimplemented!("opcode 0x29 not implemented"),
    |_2a, _cpu, _mem| unimplemented!("opcode 0x2A not implemented"),
    |_2b, _cpu, _mem| unimplemented!("opcode 0x2B not implemented"),
    |_2c, _cpu, _mem| unimplemented!("opcode 0x2C not implemented"),
    |_2d, cpu, _mem| {
        let l0 = cpu.l;
        let l1 = l0 - 1;
        cpu.l = l1;
        cpu.set_z_flag(l1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(0 < l1);
        op_execution!{
            cycles: 1;
            asm: "DEC L";
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", l0, l1;
        }
    },
    |_2e, cpu, mem| {
        let l0 = cpu.l;
        let l1 = cpu.read_immediate_u8(mem);
        cpu.l = l1;
        op_execution!{
            cycles: 2;
            asm: "LD L, ${:02x}", l1;
            trace: "L₀ = ${:02x}, L₁ = ${:02x}", l0, l1;
        }
    },
    |_2f, _cpu, _mem| unimplemented!("opcode 0x2F not implemented"),
    |_30, cpu, mem| {
        let n = cpu.read_immediate_i8(mem);
        let c_flag = cpu.c_flag();
        if c_flag == false {
            cpu.relative_jump(n);
        }
        op_execution!{
            cycles: 2;
            asm: "JR NC, {}", n;
            trace: "C = {}", c_flag;
        }
    },
    |_31, cpu, mem| {
        let sp0 = cpu.sp;
        let sp1 = cpu.read_immediate_u16(mem);
        cpu.sp = sp1;
        op_execution!{
            cycles: 3;
            asm: "LOAD SP, ${:04x}", sp1;
            trace: "SP₀ = ${:04x}", sp0;
        }
    },
    |_32, cpu, mem| {
        let hl0 = cpu.hl();
        let hl1 = hl0 - 1;
        let a = cpu.a;
        mem.set(hl0, a);
        cpu.set_hl(hl1);
        op_execution!{
            cycles: 2;
            asm: "LD (HL-), A";
            trace: "HL₀ = ${:04x}, A = ${:02x}", hl0, a;
        }
    },
    |_33, _cpu, _mem| unimplemented!("opcode 0x33 not implemented"),
    |_34, _cpu, _mem| unimplemented!("opcode 0x34 not implemented"),
    |_35, _cpu, _mem| unimplemented!("opcode 0x35 not implemented"),
    |_36, _cpu, _mem| unimplemented!("opcode 0x36 not implemented"),
    |_37, _cpu, _mem| unimplemented!("opcode 0x37 not implemented"),
    |_38, cpu, mem| {
        let n = cpu.read_immediate_i8(mem);
        let c_flag = cpu.c_flag();
        if c_flag {
            cpu.relative_jump(n);
        }
        op_execution!{
            cycles: 2;
            asm: "JR C, {}", n;
            trace: "C = {}", c_flag;
        }
    },
    |_39, _cpu, _mem| unimplemented!("opcode 0x39 not implemented"),
    |_3a, _cpu, _mem| unimplemented!("opcode 0x3A not implemented"),
    |_3b, _cpu, _mem| unimplemented!("opcode 0x3B not implemented"),
    |_3c, _cpu, _mem| unimplemented!("opcode 0x3C not implemented"),
    |_3d, cpu, _mem| {
        let a0 = cpu.a;
        let a1 = a0 - 1;
        cpu.a = a1;
        cpu.set_z_flag(a1 == 0);
        cpu.set_n_flag(false);
        cpu.set_h_flag(a0 < a1);
        op_execution!{
            cycles: 1;
            asm: "DEC A";
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", a0, a1;
        }
    },
    |_3e, cpu, mem| {
        let n = cpu.read_immediate_u8(mem);
        let a0 = cpu.a;
        cpu.a = n;
        op_execution!{
            cycles: 2;
            asm: "LD A, ${:02x}", n;
            trace: "A₀ = ${:02x}", a0;
        }
    },
    |_3f, _cpu, _mem| unimplemented!("opcode 0x3F not implemented"),
    |_40, cpu, _mem| {
        let b = cpu.b;
        op_execution!{
            cycles: 1;
            asm: "LD B, B";
            trace: "B = ${:02x}", b;
        }
    },
    |_41, cpu, _mem| {
        let b0 = cpu.b;
        let c = cpu.c;
        cpu.b = c;
        op_execution!{
            cycles: 1;
            asm: "LD B, C";
            trace: "B₀ = ${:02x}, C = ${:02x}", b0, c;
        }
    },
    |_42, cpu, _mem| {
        let b0 = cpu.b;
        let d = cpu.d;
        cpu.b = d;
        op_execution!{
            cycles: 1;
            asm: "LD B, D";
            trace: "B₀ = ${:02x}, D = ${:02x}", b0, d;
        }
    },
    |_43, cpu, _mem| {
        let b0 = cpu.b;
        let e = cpu.e;
        cpu.b = e;
        op_execution!{
            cycles: 1;
            asm: "LD B, E";
            trace: "B₀ = ${:02x}, E = ${:02x}", b0, e;
        }
    },
    |_44, cpu, _mem| {
        let b0 = cpu.b;
        let h = cpu.h;
        cpu.b = h;
        op_execution!{
            cycles: 1;
            asm: "LD B, H";
            trace: "B₀ = ${:02x}, H = ${:02x}", b0, h;
        }
    },
    |_45, cpu, _mem| {
        let b0 = cpu.b;
        let l = cpu.l;
        cpu.b = l;
        op_execution!{
            cycles: 1;
            asm: "LD B, L";
            trace: "B₀ = ${:02x}, L = ${:02x}", b0, l;
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
            trace: "B₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", b0, hl, b1;
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
            trace: "C₀ = ${:02x}, B = ${:02x}", c0, b;
        }
    },
    |_49, cpu, _mem| {
        let c = cpu.c;
        op_execution!{
            cycles: 1;
            asm: "LD C, C";
            trace: "C = ${:02x}", c;
        }
    },
    |_4a, cpu, _mem| {
        let c0 = cpu.c;
        let d = cpu.d;
        cpu.c = d;
        op_execution!{
            cycles: 1;
            asm: "LD C, D";
            trace: "C₀ = ${:02x}, D = ${:02x}", c0, d;
        }
    },
    |_4b, cpu, _mem| {
        let c0 = cpu.c;
        let e = cpu.e;
        cpu.c = e;
        op_execution!{
            cycles: 1;
            asm: "LD C, E";
            trace: "C₀ = ${:02x}, E = ${:02x}", c0, e;
        }
    },
    |_4c, cpu, _mem| {
        let c0 = cpu.c;
        let h = cpu.h;
        cpu.c = h;
        op_execution!{
            cycles: 1;
            asm: "LD C, H";
            trace: "C₀ = ${:02x}, H = ${:02x}", c0, h;
        }
    },
    |_4d, cpu, _mem| {
        let c0 = cpu.c;
        let l = cpu.l;
        cpu.c = l;
        op_execution!{
            cycles: 1;
            asm: "LD C, L";
            trace: "C₀ = ${:02x}, L = ${:02x}", c0, l;
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
            trace: "C₀ = ${:02x}, HL = ${:04x}, (HL) = ${:04x}", c0, hl, c1;
        }
    },
    |_4f, cpu, _mem| {
        let c0 = cpu.c;
        let a = cpu.a;
        cpu.b = a;
        op_execution!{
            cycles: 1;
            asm: "LD C, A";
            trace: "C₀ = ${:02x}, A = ${:02x}", c0, a;
        }
    },
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
    |_77, cpu, mem| {
        let hl = cpu.hl();
        let a = cpu.a;
        mem.set(hl, a);
        op_execution!{
            cycles: 2;
            asm: "LD (HL), A";
            trace: "HL = ${:04x}, A = ${:02x}", hl, cpu.a;
        }
    },
    |_78, cpu, _mem| {
        let a0 = cpu.a;
        let b = cpu.b;
        cpu.a = b;
        op_execution!{
            cycles: 1;
            asm: "LD A, B";
            trace: "A₀ = ${:02x}, B = ${:02x}", a0, b;
        }
    },
    |_79, cpu, _mem| {
        let a0 = cpu.a;
        let c = cpu.c;
        cpu.a = c;
        op_execution!{
            cycles: 1;
            asm: "LD A, C";
            trace: "A₀ = ${:02x}, C = ${:02x}", a0, c;
        }
    },
    |_7a, cpu, _mem| {
        let a0 = cpu.a;
        let d = cpu.d;
        cpu.a = d;
        op_execution!{
            cycles: 1;
            asm: "LD A, D";
            trace: "A₀ = ${:02x}, D = ${:02x}", a0, d;
        }
    },
    |_7b, cpu, _mem| {
        let a0 = cpu.a;
        let e = cpu.e;
        cpu.a = e;
        op_execution!{
            cycles: 1;
            asm: "LD A, E";
            trace: "A₀ = ${:02x}, E = ${:02x}", a0, e;
        }
    },
    |_7c, cpu, _mem| {
        let a0 = cpu.a;
        let h = cpu.h;
        cpu.a = h;
        op_execution!{
            cycles: 1;
            asm: "LD A, H";
            trace: "A₀ = ${:02x}, H = ${:02x}", a0, h;
        }
    },
    |_7d, cpu, _mem| {
        let a0 = cpu.a;
        let l = cpu.l;
        cpu.a = l;
        op_execution!{
            cycles: 1;
            asm: "LD A, L";
            trace: "A₀ = ${:02x}, L = ${:02x}", a0, l;
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
            trace: "A₀ = ${:02x}, HL = ${:04x}, (HL) = ${:02x}", a0, hl, a1;
        }
    },
    |_7f, cpu, _mem| {
        let a = cpu.a;
        op_execution!{
            cycles: 1;
            asm: "LD A, A";
            trace: "A = ${:02x}", a;
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
            trace: "A₀ = ${:02x}, B = ${:02x} A₁ = ${:02x}", a0, b, a1;
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
            trace: "A₀ = ${:02x}, C = ${:02x} A₁ = ${:02x}", a0, c, a1;
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
            trace: "A₀ = ${:02x}, D = ${:02x} A₁ = ${:02x}", a0, d, a1;
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
            trace: "A₀ = ${:02x}, E = ${:02x} A₁ = ${:02x}", a0, e, a1;
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
            trace: "A₀ = ${:02x}, H = ${:02x} A₁ = ${:02x}", a0, h, a1;
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
            trace: "A₀ = ${:02x}, L = ${:02x} A₁ = ${:02x}", a0, l, a1;
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
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", a0, a1;
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
    |_c1, cpu, mem| {
        let bc0 = cpu.bc();
        let bc1 = cpu.stack_pop(mem);
        op_execution!{
            cycles: 3;
            asm: "POP BC";
            trace: "P₁ = ${:04x}, BC₀ = ${:04x}, BC₁ = ${:04x}", cpu.sp, bc0, bc1;
        }
    },
    |_c2, _cpu, _mem| unimplemented!("opcode 0xC2 not implemented"),
    |_c3, _cpu, _mem| unimplemented!("opcode 0xC3 not implemented"),
    |_c4, _cpu, _mem| unimplemented!("opcode 0xC4 not implemented"),
    |_c5, cpu, mem| {
        let bc = cpu.bc();
        cpu.stack_push(mem, bc);
        op_execution!{
            cycles: 4;
            asm: "PUSH BC";
            trace: "SP₁ = ${:04x}, BC = ${:04x}", cpu.sp, bc;
        }
    },
    |_c6, _cpu, _mem| unimplemented!("opcode 0xC6 not implemented"),
    |_c7, _cpu, _mem| unimplemented!("opcode 0xC7 not implemented"),
    |_c8, _cpu, _mem| unimplemented!("opcode 0xC8 not implemented"),
    |_c9, _cpu, _mem| unimplemented!("opcode 0xC9 not implemented"),
    |_ca, _cpu, _mem| unimplemented!("opcode 0xCA not implemented"),
    |_cb, _cpu, _mem| {
        panic!("0xCB prefix is not a complete opcode");
    },
    |_cc, _cpu, _mem| unimplemented!("opcode 0xCC not implemented"),
    |_cd, cpu, mem| {
        let nn = cpu.read_immediate_u16(mem);
        let i0 = cpu.i;
        cpu.stack_push(mem, i0);
        cpu.i = nn;
        op_execution!{
            cycles: 3;
            asm: "CALL ${:04x}", nn;
            trace: "SP₁ = {:04x}", cpu.sp;
        }
    },
    |_ce, _cpu, _mem| unimplemented!("opcode 0xCE not implemented"),
    |_cf, _cpu, _mem| unimplemented!("opcode 0xCF not implemented"),
    |_d0, _cpu, _mem| unimplemented!("opcode 0xD0 not implemented"),
    |_d1, cpu, mem| {
        let de0 = cpu.de();
        let de1 = cpu.stack_pop(mem);
        op_execution!{
            cycles: 3;
            asm: "POP DE";
            trace: "SP₁ = ${:04x}, DE₀ = ${:04x}, DE₁ = ${:04x}", cpu.sp, de0, de1;
        }
    },
    |_d2, _cpu, _mem| unimplemented!("opcode 0xD2 not implemented"),
    |_d3, _cpu, _mem| {
        panic!("0xD3 is not a valid opcode");
    },
    |_d4, _cpu, _mem| unimplemented!("opcode 0xD4 not implemented"),
    |_d5, cpu, mem| {
        let de = cpu.de();
        cpu.stack_push(mem, de);
        op_execution!{
            cycles: 4;
            asm: "PUSH DE";
            trace: "SP₁ = ${:04x}, DE = ${:04x}", cpu.sp, de;
        }
    },
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
    |_e0, cpu, mem| {
        let a = cpu.a;
        let n = cpu.read_immediate_u8(mem);
        mem.set(0xFF00 + n as u16, a);
        op_execution!{
            cycles: 3;
            asm: "LD ($ff00 + ${:02x}), A", n;
            trace: "A = ${:02x}", a;
        }
    },
    |_e1, cpu, mem| {
        let hl0 = cpu.hl();
        let hl1 = cpu.stack_pop(mem);
        op_execution!{
            cycles: 3;
            asm: "POP HL";
            trace: "SP₁ = ${:04x}, HL₀ = ${:04x}, HL₁ = ${:04x}", cpu.sp, hl0, hl1;
        }
    },
    |_e2, cpu, mem| {
        let a = cpu.a;
        let c = cpu.c;
        let address = 0xFF00 + (c as u16);
        mem.set(address, a);
        op_execution!{
            cycles: 2;
            asm: "LD ($FF00 + C), A ";
            trace: "A = ${:02x}, C = ${:02x}", a, c;
        }
    },
    |_e3, _cpu, _mem| {
        panic!("0xE3 is not a valid opcode");
    },
    |_e4, _cpu, _mem| {
        panic!("0xE4 is not a valid opcode");
    },
    |_e5, cpu, mem| {
        let hl = cpu.hl();
        cpu.stack_push(mem, hl);
        op_execution!{
            cycles: 4;
            asm: "PUSH hl";
            trace: "SP₁ = ${:04x}, HL = ${:04x}", cpu.sp, hl;
        }
    },
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
    |_f1, cpu, mem| {
        let af0 = cpu.af();
        let af1 = cpu.stack_pop(mem);
        op_execution!{
            cycles: 3;
            asm: "POP AF";
            trace: "SP₁ = ${:04x}, AF₀ = ${:04x}, AF₁ = ${:04x}", cpu.sp, af0, af1;
        }
    },
    |_f2, _cpu, _mem| unimplemented!("opcode 0xF2 not implemented"),
    |_f3, _cpu, _mem| unimplemented!("opcode 0xF3 not implemented"),
    |_f4, _cpu, _mem| {
        panic!("0xF4 is not a valid opcode");
    },
    |_f5, cpu, mem| {
        let af = cpu.af();
        cpu.stack_push(mem, af);
        op_execution!{
            cycles: 4;
            asm: "PUSH AF";
            trace: "SP₁ = ${:04x}, AF = ${:04x}", cpu.sp, af;
        }
    },
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
            trace: "A₀ = ${:02x}, A₁ = ${:04x}", a0, a1;
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
