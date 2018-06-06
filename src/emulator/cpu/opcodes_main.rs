use super::operation;
use super::operation::{u8_get_bit, u8_set_bit};

use emulator::cpu::CPUController;
use emulator::memory::MemoryController;

// one-byte opcodes
pub static OPCODES: [operation::Operation; 0xFF] = [
    |_00, _gb| {
        op_execution!{
            cycles: 1;
            asm: "NOP";
        }
    },
    |_01, _gb| unimplemented!("opcode 0x01 not implemented"),
    |_02, _gb| unimplemented!("opcode 0x02 not implemented"),
    |_03, gb| {
        let bc0 = gb.bc();
        let bc1 = bc0.wrapping_add(1);
        gb.set_bc(bc1);
        op_execution!{
            cycles: 2;
            asm: "INC BC";
            trace: "BC₀ = ${:04x}, BC₁ = ${:04x}", bc0, bc1;
        }
    },
    operation::INC,
    operation::DEC,
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
        let a1 = gb.mem(bc);
        gb.cpu.a = a1;
        op_execution!{
            cycles: 2;
            asm: "LD A, (BC)";
            trace: "A₀ = ${:02x}, BC = ${:04x}, (BC) = ${:02x}", a0, bc, a1;
        }
    },
    |_0b, _gb| unimplemented!("opcode 0x0B not implemented"),
    operation::INC,
    operation::DEC,
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
    |_13, gb| {
        let de0 = gb.de();
        let de1 = de0.wrapping_add(1);
        gb.set_de(de1);
        op_execution!{
            cycles: 2;
            asm: "INC DE";
            trace: "DE₀ = ${:04x}, DE₁ = ${:04x}", de0, de1;
        }
    },
    operation::INC,
    operation::DEC,
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
        let a1 = a0 << (1 + if gb.c_flag() { 1 } else { 0 });
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
    |_18, gb| {
        let n = gb.read_immediate_i8();
        gb.relative_jump(n);
        op_execution!{
            cycles: 2;
            asm: "JR {}", n;
        }
    },
    |_19, _gb| unimplemented!("opcode 0x19 not implemented"),
    |_1a, gb| {
        let a0 = gb.cpu.a;
        let de = gb.de();
        let a1 = gb.mem(de);
        gb.cpu.a = a1;
        op_execution!{
            cycles: 2;
            asm: "LD A, (DE)";
            trace: "A₀ = ${:02x}, DE = ${:04x}, (DE) = ${:02x}", a0, de, a1;
        }
    },
    |_1b, _gb| unimplemented!("opcode 0x1B not implemented"),
    operation::INC,
    operation::DEC,
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
        let hl1 = hl0.wrapping_add(1);
        gb.set_mem(hl0, a);
        gb.set_hl(hl1);
        op_execution!{
            cycles: 2;
            asm: "LD (HL+), A";
            trace: "HL₀ = ${:04x}, A = ${:02x}", hl0, a;
        }
    },
    |_23, gb| {
        let hl0 = gb.hl();
        let hl1 = hl0.wrapping_add(1);
        gb.set_hl(hl1);
        op_execution!{
            cycles: 2;
            asm: "INC HL";
            trace: "HL₀ = ${:04x}, HL₁ = ${:04x}", hl0, hl1;
        }
    },
    operation::INC,
    operation::DEC,
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
    operation::INC,
    operation::DEC,
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
        let hl1 = hl0.wrapping_sub(1);
        let a = gb.cpu.a;
        gb.set_mem(hl0, a);
        gb.set_hl(hl1);
        op_execution!{
            cycles: 2;
            asm: "LD (HL-), A";
            trace: "HL₀ = ${:04x}, A = ${:02x}", hl0, a;
        }
    },
    |_33, gb| {
        let sp0 = gb.cpu.sp;
        let sp1 = sp0.wrapping_add(1);
        gb.cpu.sp = sp1;
        op_execution!{
            cycles: 2;
            asm: "INC SP";
            trace: "SP₀ = ${:04x}, SP₁ = ${:04x}", sp0, sp1;
        }
    },
    operation::INC,
    operation::DEC,
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
    operation::INC,
    operation::DEC,
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
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    |_76, _gb| unimplemented!("opcode 0x76, HALT, is not implemented"),
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::INTRA_REGISTER_LOAD,
    operation::ADD,
    operation::ADD,
    operation::ADD,
    operation::ADD,
    operation::ADD,
    operation::ADD,
    operation::ADD,
    operation::ADD,
    |_88, _gb| unimplemented!("opcode 0x88 not implemented"),
    |_89, _gb| unimplemented!("opcode 0x89 not implemented"),
    |_8a, _gb| unimplemented!("opcode 0x8A not implemented"),
    |_8b, _gb| unimplemented!("opcode 0x8B not implemented"),
    |_8c, _gb| unimplemented!("opcode 0x8C not implemented"),
    |_8d, _gb| unimplemented!("opcode 0x8D not implemented"),
    |_8e, _gb| unimplemented!("opcode 0x8E not implemented"),
    |_8f, _gb| unimplemented!("opcode 0x8F not implemented"),
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    |_98, _gb| unimplemented!("opcode 0x98 not implemented"),
    |_99, _gb| unimplemented!("opcode 0x99 not implemented"),
    |_9a, _gb| unimplemented!("opcode 0x9A not implemented"),
    |_9b, _gb| unimplemented!("opcode 0x9B not implemented"),
    |_9c, _gb| unimplemented!("opcode 0x9C not implemented"),
    |_9d, _gb| unimplemented!("opcode 0x9D not implemented"),
    |_9e, _gb| unimplemented!("opcode 0x9E not implemented"),
    |_9f, _gb| unimplemented!("opcode 0x9F not implemented"),
    operation::AND,
    operation::AND,
    operation::AND,
    operation::AND,
    operation::AND,
    operation::AND,
    operation::AND,
    operation::AND,
    operation::XOR,
    operation::XOR,
    operation::XOR,
    operation::XOR,
    operation::XOR,
    operation::XOR,
    operation::XOR,
    operation::XOR,
    operation::OR,
    operation::OR,
    operation::OR,
    operation::OR,
    operation::OR,
    operation::OR,
    operation::OR,
    operation::OR,
    operation::CP,
    operation::CP,
    operation::CP,
    operation::CP,
    operation::CP,
    operation::CP,
    operation::CP,
    operation::CP,
    |_c0, _gb| unimplemented!("opcode 0xC0 not implemented"),
    |_c1, gb| {
        let bc0 = gb.bc();
        let bc1 = gb.stack_pop();
        gb.set_bc(bc1);
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
    |_c9, gb| {
        let pc1 = gb.stack_pop();
        let sp1 = gb.cpu.sp;
        gb.cpu.pc = pc1;
        op_execution!{
            cycles: 2;
            asm: "RET";
            trace: "SP₁ = {:04x}", sp1;
        }
    },
    |_ca, _gb| unimplemented!("opcode 0xCA not implemented"),
    |_cb, _gb| {
        panic!("0xCB prefix is not a complete opcode");
    },
    |_cc, _gb| unimplemented!("opcode 0xCC not implemented"),
    |_cd, gb| {
        let nn = gb.read_immediate_u16();
        let pc0 = gb.cpu.pc;
        gb.stack_push(pc0);
        let sp1 = gb.cpu.sp;
        gb.cpu.pc = nn;
        op_execution!{
            cycles: 3;
            asm: "CALL ${:04x}", nn;
            trace: "SP₁ = {:04x}", sp1;
        }
    },
    |_ce, _gb| unimplemented!("opcode 0xCE not implemented"),
    |_cf, _gb| unimplemented!("opcode 0xCF not implemented"),
    |_d0, _gb| unimplemented!("opcode 0xD0 not implemented"),
    |_d1, gb| {
        let de0 = gb.de();
        let de1 = gb.stack_pop();
        gb.set_de(de1);
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
        gb.set_mem(0xFF00 + n as u16, a);
        op_execution!{
            cycles: 3;
            asm: "LD ($FF00 + ${:02x}), A", n;
            trace: "A = ${:02x}", a;
        }
    },
    |_e1, gb| {
        let hl0 = gb.hl();
        let hl1 = gb.stack_pop();
        gb.set_hl(hl1);
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
        gb.set_mem(address, a);
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
    |_ea, gb| {
        let nn = gb.read_immediate_u16();
        let a = gb.cpu.a;
        gb.set_mem(nn, a);
        op_execution!{
            cycles: 4;
            asm: "LD (${:02x}), A", nn;
            trace: "A = {:02x}", a;
        }
    },
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
    |_f0, gb| {
        let a0 = gb.cpu.a;
        let n = gb.read_immediate_u8();
        let a1 = gb.mem(0xFF00 + n as u16);
        gb.cpu.a = a1;
        op_execution!{
            cycles: 3;
            asm: "LD A, ($FF00 + ${:02x})", n;
            trace: "A₀ = ${:02x}, A₁ = ${:02x}", a0, a1;
        }
    },
    |_f1, gb| {
        let af0 = gb.af();
        let af1 = gb.stack_pop();
        gb.set_af(af1);
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
        let a1 = gb.mem(nn);
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
    |_fe, gb| {
        let n = gb.read_immediate_u8();
        let a = gb.cpu.a;
        let delta = a.wrapping_sub(n);
        gb.set_z_flag(delta == 0);
        gb.set_n_flag(true);
        gb.set_h_flag(u8_get_bit(delta, 4));
        gb.set_c_flag(a < n);
        op_execution!{
            cycles: 2;
            asm: "CP ${:02x}", n;
            trace: "A = ${:02x}, F_Z = {}, F_C = {}", a, gb.z_flag(), gb.n_flag();
        }
    },
];
