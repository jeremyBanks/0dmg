use super::operation;
use super::operation::Execution;
use super::{u8_get_bit, u8_set_bit};

use super::super::memory::MemoryController;
use super::CPUController;

// one-byte opcodes
pub static OPCODES: [operation::Operation; 0x100] = [
    |_00, _gb| {
        op_execution!{
            cycles: 1;
            asm: "NOP";
        }
    },
    /* 01 */ operation::UNIMPLEMENTED,
    /* 02 */ operation::UNIMPLEMENTED,
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
    /* 07 */ operation::UNIMPLEMENTED,
    /* 08 */ operation::UNIMPLEMENTED,
    /* 09 */ operation::UNIMPLEMENTED,
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
    /* 0b */ operation::UNIMPLEMENTED,
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
    /* 0f */ operation::UNIMPLEMENTED,
    /* 10 */ operation::UNIMPLEMENTED,
    |_11, gb| {
        let de0 = gb.de();
        let de1 = gb.read_immediate_u16();
        gb.set_de(de1);
        op_execution!{
            cycles: 3;
            asm: "LD DE, ${:04x}", de1;
            trace: "DE₁ = ${:04x}", de0;
        }
    },
    /* 12 */ operation::UNIMPLEMENTED,
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
        let fc0 = gb.c_flag();
        let a0 = gb.cpu.a;
        let a1 = (a0 << 1) + if fc0 { 1 } else { 0 };
        let fc1 = a0 & 0b10000000 > 0;
        gb.cpu.a = a1;
        gb.set_z_flag(a1 == 0);
        gb.set_c_flag(fc1);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL A";
            trace: "Fc₀ = {}, A₀ = ${:02x}, Fc₁ = {}, A₁ = ${:02x}", fc0, a0, fc1, a1;
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
    /* 19 */ operation::UNIMPLEMENTED,
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
    /* 1b */ operation::UNIMPLEMENTED,
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
    /* 1f */ operation::UNIMPLEMENTED,
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
            asm: "LD HL, ${:04x}", hl1;
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
    /* 27 */ operation::UNIMPLEMENTED,
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
    /* 29 */ operation::UNIMPLEMENTED,
    /* 2a */ operation::UNIMPLEMENTED,
    /* 2b */ operation::UNIMPLEMENTED,
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
    /* 2f */ operation::UNIMPLEMENTED,
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
            asm: "LD SP, ${:04x}", sp1;
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
    /* 36 */ operation::UNIMPLEMENTED,
    /* 37 */ operation::UNIMPLEMENTED,
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
    /* 39 */ operation::UNIMPLEMENTED,
    /* 3a */ operation::UNIMPLEMENTED,
    /* 3b */ operation::UNIMPLEMENTED,
    operation::INC,
    operation::DEC,
    |_3e, gb| {
        let a0 = gb.cpu.a;
        let a1 = gb.read_immediate_u8();
        gb.cpu.a = a1;
        op_execution!{
            cycles: 2;
            asm: "LD A, ${:02x}", a1;
            trace: "A₀ = ${:02x}", a0;
        }
    },
    /* 3f */ operation::UNIMPLEMENTED,
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
    /* 76 */ operation::UNIMPLEMENTED,
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
    /* 88 */ operation::UNIMPLEMENTED,
    /* 89 */ operation::UNIMPLEMENTED,
    /* 8a */ operation::UNIMPLEMENTED,
    /* 8b */ operation::UNIMPLEMENTED,
    /* 8c */ operation::UNIMPLEMENTED,
    /* 8d */ operation::UNIMPLEMENTED,
    /* 8e */ operation::UNIMPLEMENTED,
    /* 8f */ operation::UNIMPLEMENTED,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    operation::SUB,
    /* 98 */ operation::UNIMPLEMENTED,
    /* 99 */ operation::UNIMPLEMENTED,
    /* 9a */ operation::UNIMPLEMENTED,
    /* 9b */ operation::UNIMPLEMENTED,
    /* 9c */ operation::UNIMPLEMENTED,
    /* 9d */ operation::UNIMPLEMENTED,
    /* 9e */ operation::UNIMPLEMENTED,
    /* 9f */ operation::UNIMPLEMENTED,
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
    /* c0 */ operation::UNIMPLEMENTED,
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
    /* c2 */ operation::UNIMPLEMENTED,
    |_c3, gb| {
        let addr = gb.read_immediate_u16();
        gb.cpu.pc = addr;
        op_execution!{
            cycles: 3;
            asm: "JP ${:04x}", addr;
        }
    },
    /* c4 */ operation::UNIMPLEMENTED,
    |_c5, gb| {
        let bc = gb.bc();
        gb.stack_push(bc);
        op_execution!{
            cycles: 4;
            asm: "PUSH BC";
            trace: "SP₁ = ${:04x}, BC = ${:04x}", gb.cpu.sp, bc;
        }
    },
    /* c6 */ operation::UNIMPLEMENTED,
    operation::RST,
    /* c8 */ operation::UNIMPLEMENTED,
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
    /* ca */ operation::UNIMPLEMENTED,
    |_cb, _gb| {
        panic!("0xCB prefix is not a complete opcode");
    },
    /* cc */ operation::UNIMPLEMENTED,
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
    /* ce */ operation::UNIMPLEMENTED,
    operation::RST,
    /* d0 */ operation::UNIMPLEMENTED,
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
    /* d2 */ operation::UNIMPLEMENTED,
    |_d3, _gb| {
        panic!("0xD3 is not a valid opcode");
    },
    /* d4 */ operation::UNIMPLEMENTED,
    |_d5, gb| {
        let de = gb.de();
        gb.stack_push(de);
        op_execution!{
            cycles: 4;
            asm: "PUSH DE";
            trace: "SP₁ = ${:04x}, DE = ${:04x}", gb.cpu.sp, de;
        }
    },
    /* d6 */ operation::UNIMPLEMENTED,
    operation::RST,
    /* d8 */ operation::UNIMPLEMENTED,
    /* d9 */ operation::UNIMPLEMENTED,
    /* da */ operation::UNIMPLEMENTED,
    |_db, _gb| {
        panic!("0xDB is not a valid opcode");
    },
    /* dc */ operation::UNIMPLEMENTED,
    |_dd, _gb| {
        panic!("0xDD is not a valid opcode");
    },
    /* de */ operation::UNIMPLEMENTED,
    operation::RST,
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
    /* e6 */ operation::UNIMPLEMENTED,
    operation::RST,
    /* e8 */ operation::UNIMPLEMENTED,
    /* e9 */ operation::UNIMPLEMENTED,
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
    /* ee */ operation::UNIMPLEMENTED,
    operation::RST,
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
    /* f2 */ operation::UNIMPLEMENTED,
    |_f3, _gb| {
        println!("ignoring DI instructions because interrupts haven't been implemented!");
        op_execution!{
            cycles: 1;
            asm: "DI";
            trace: "lol NOPE";
        }
    },
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
    /* f6 */ operation::UNIMPLEMENTED,
    operation::RST,
    /* f8 */ operation::UNIMPLEMENTED,
    /* f9 */ operation::UNIMPLEMENTED,
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
    /* fb */ operation::UNIMPLEMENTED,
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
    operation::RST,
];
