use zerodmg_utils::little_endian::u8_get_bit;

use super::operation;
use super::operation::Execution;

use super::CPUController;

// 0xCB-prefixed two-byte opcodes
pub static OPCODES: [operation::Operation; 0x100] = [
    /* cb00 */ operation::UNIMPLEMENTED,
    /* cb01 */ operation::UNIMPLEMENTED,
    /* cb02 */ operation::UNIMPLEMENTED,
    /* cb03 */ operation::UNIMPLEMENTED,
    /* cb04 */ operation::UNIMPLEMENTED,
    /* cb05 */ operation::UNIMPLEMENTED,
    /* cb06 */ operation::UNIMPLEMENTED,
    /* cb07 */ operation::UNIMPLEMENTED,
    /* cb08 */ operation::UNIMPLEMENTED,
    /* cb09 */ operation::UNIMPLEMENTED,
    /* cb0a */ operation::UNIMPLEMENTED,
    /* cb0b */ operation::UNIMPLEMENTED,
    /* cb0c */ operation::UNIMPLEMENTED,
    /* cb0d */ operation::UNIMPLEMENTED,
    /* cb0e */ operation::UNIMPLEMENTED,
    /* cb0f */ operation::UNIMPLEMENTED,
    |_10, gb| {
        let b0 = gb.cpu.b;
        let fc0 = gb.c_flag();
        let b1 = (b0 << 1) + if fc0 { 1 } else { 0 };
        let fc1 = b0 & 0b10000000 > 0;
        gb.cpu.b = b1;
        gb.set_z_flag(b1 == 0);
        gb.set_c_flag(fc1);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL B";
            trace: "Fc₀ = {}, B₀ = ${:02x}, Fc₁ = {}, B₁ = ${:02x}", fc0, b0, fc1, b1;
        }
    },
    |_11, gb| {
        let c0 = gb.cpu.c;
        let fc0 = gb.c_flag();
        let c1 = (c0 << 1) + if fc0 { 1 } else { 0 };
        let fc1 = c0 & 0b10000000 > 0;
        gb.cpu.c = c1;
        gb.set_z_flag(c1 == 0);
        gb.set_c_flag(fc1);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL C";
            trace: "Fc₀ = {}, C₀ = ${:02x}, Fc₁ = {}, C₁ = ${:02x}", fc0, c0, fc1, c1;
        }
    },
    |_12, gb| {
        let d0 = gb.cpu.d;
        let fc0 = gb.c_flag();
        let d1 = (d0 << 1) + if fc0 { 1 } else { 0 };
        let fc1 = d0 & 0b10000000 > 0;
        gb.cpu.d = d1;
        gb.set_z_flag(d1 == 0);
        gb.set_c_flag(fc1);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL D";
            trace: "Fc₀ = {}, D₀ = ${:02x}, Fc₁ = {}, D₁ = ${:02x}", fc0, d0, fc1, d1;
        }
    },
    |_13, gb| {
        let e0 = gb.cpu.e;
        let fc0 = gb.c_flag();
        let e1 = (e0 << 1) + if fc0 { 1 } else { 0 };
        let fc1 = e0 & 0b10000000 > 0;
        gb.cpu.e = e1;
        gb.set_z_flag(e1 == 0);
        gb.set_c_flag(fc1);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL E";
            trace: "Fc₀ = {}, E₀ = ${:02x}, Fc₁ = {}, E₁ = ${:02x}", fc0, e0, fc1, e1;
        }
    },
    |_14, gb| {
        let h0 = gb.cpu.h;
        let fc0 = gb.c_flag();
        let h1 = (h0 << 1) + if fc0 { 1 } else { 0 };
        let fc1 = h0 & 0b10000000 > 0;
        gb.cpu.h = h1;
        gb.set_z_flag(h1 == 0);
        gb.set_c_flag(fc1);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL H";
            trace: "Fc₀ = {}, H₀ = ${:02x}, Fc₁ = {}, H₁ = ${:02x}", fc0, h0, fc1, h1;
        }
    },
    |_15, gb| {
        let l0 = gb.cpu.l;
        let fc0 = gb.c_flag();
        let l1 = (l0 << 1) + if fc0 { 1 } else { 0 };
        let fc1 = l0 & 0b10000000 > 0;
        gb.cpu.l = l1;
        gb.set_z_flag(l1 == 0);
        gb.set_c_flag(fc1);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL L";
            trace: "Fc₀ = {}, L₀ = ${:02x}, Fc₁ = {}, L₁ = ${:02x}", fc0, l0, fc1, l1;
        }
    },
    /* cb16 */ operation::UNIMPLEMENTED,
    /* cb17 */ operation::UNIMPLEMENTED,
    /* cb18 */ operation::UNIMPLEMENTED,
    /* cb19 */ operation::UNIMPLEMENTED,
    /* cb1a */ operation::UNIMPLEMENTED,
    /* cb1b */ operation::UNIMPLEMENTED,
    /* cb1c */ operation::UNIMPLEMENTED,
    /* cb1d */ operation::UNIMPLEMENTED,
    /* cb1e */ operation::UNIMPLEMENTED,
    /* cb1f */ operation::UNIMPLEMENTED,
    /* cb20 */ operation::UNIMPLEMENTED,
    /* cb21 */ operation::UNIMPLEMENTED,
    /* cb22 */ operation::UNIMPLEMENTED,
    /* cb23 */ operation::UNIMPLEMENTED,
    /* cb24 */ operation::UNIMPLEMENTED,
    /* cb25 */ operation::UNIMPLEMENTED,
    /* cb26 */ operation::UNIMPLEMENTED,
    /* cb27 */ operation::UNIMPLEMENTED,
    /* cb28 */ operation::UNIMPLEMENTED,
    /* cb29 */ operation::UNIMPLEMENTED,
    /* cb2a */ operation::UNIMPLEMENTED,
    /* cb2b */ operation::UNIMPLEMENTED,
    /* cb2c */ operation::UNIMPLEMENTED,
    /* cb2d */ operation::UNIMPLEMENTED,
    /* cb2e */ operation::UNIMPLEMENTED,
    /* cb2f */ operation::UNIMPLEMENTED,
    /* cb30 */ operation::UNIMPLEMENTED,
    /* cb31 */ operation::UNIMPLEMENTED,
    /* cb32 */ operation::UNIMPLEMENTED,
    /* cb33 */ operation::UNIMPLEMENTED,
    /* cb34 */ operation::UNIMPLEMENTED,
    /* cb35 */ operation::UNIMPLEMENTED,
    /* cb36 */ operation::UNIMPLEMENTED,
    /* cb37 */ operation::UNIMPLEMENTED,
    /* cb38 */ operation::UNIMPLEMENTED,
    /* cb39 */ operation::UNIMPLEMENTED,
    /* cb3a */ operation::UNIMPLEMENTED,
    /* cb3b */ operation::UNIMPLEMENTED,
    /* cb3c */ operation::UNIMPLEMENTED,
    /* cb3d */ operation::UNIMPLEMENTED,
    /* cb3e */ operation::UNIMPLEMENTED,
    /* cb3f */ operation::UNIMPLEMENTED,
    /* cb40 */ operation::UNIMPLEMENTED,
    /* cb41 */ operation::UNIMPLEMENTED,
    /* cb42 */ operation::UNIMPLEMENTED,
    /* cb43 */ operation::UNIMPLEMENTED,
    /* cb44 */ operation::UNIMPLEMENTED,
    /* cb45 */ operation::UNIMPLEMENTED,
    /* cb46 */ operation::UNIMPLEMENTED,
    /* cb47 */ operation::UNIMPLEMENTED,
    /* cb48 */ operation::UNIMPLEMENTED,
    /* cb49 */ operation::UNIMPLEMENTED,
    /* cb4a */ operation::UNIMPLEMENTED,
    /* cb4b */ operation::UNIMPLEMENTED,
    /* cb4c */ operation::UNIMPLEMENTED,
    /* cb4d */ operation::UNIMPLEMENTED,
    /* cb4e */ operation::UNIMPLEMENTED,
    /* cb4f */ operation::UNIMPLEMENTED,
    /* cb50 */ operation::UNIMPLEMENTED,
    /* cb51 */ operation::UNIMPLEMENTED,
    /* cb52 */ operation::UNIMPLEMENTED,
    /* cb53 */ operation::UNIMPLEMENTED,
    /* cb54 */ operation::UNIMPLEMENTED,
    /* cb55 */ operation::UNIMPLEMENTED,
    /* cb56 */ operation::UNIMPLEMENTED,
    /* cb57 */ operation::UNIMPLEMENTED,
    /* cb58 */ operation::UNIMPLEMENTED,
    /* cb59 */ operation::UNIMPLEMENTED,
    /* cb5a */ operation::UNIMPLEMENTED,
    /* cb5b */ operation::UNIMPLEMENTED,
    /* cb5c */ operation::UNIMPLEMENTED,
    /* cb5d */ operation::UNIMPLEMENTED,
    /* cb5e */ operation::UNIMPLEMENTED,
    /* cb5f */ operation::UNIMPLEMENTED,
    /* cb60 */ operation::UNIMPLEMENTED,
    /* cb61 */ operation::UNIMPLEMENTED,
    /* cb62 */ operation::UNIMPLEMENTED,
    /* cb63 */ operation::UNIMPLEMENTED,
    /* cb64 */ operation::UNIMPLEMENTED,
    /* cb65 */ operation::UNIMPLEMENTED,
    /* cb66 */ operation::UNIMPLEMENTED,
    /* cb67 */ operation::UNIMPLEMENTED,
    /* cb68 */ operation::UNIMPLEMENTED,
    /* cb69 */ operation::UNIMPLEMENTED,
    /* cb6a */ operation::UNIMPLEMENTED,
    /* cb6b */ operation::UNIMPLEMENTED,
    /* cb6c */ operation::UNIMPLEMENTED,
    /* cb6d */ operation::UNIMPLEMENTED,
    /* cb6e */ operation::UNIMPLEMENTED,
    /* cb6f */ operation::UNIMPLEMENTED,
    /* cb70 */ operation::UNIMPLEMENTED,
    /* cb71 */ operation::UNIMPLEMENTED,
    /* cb72 */ operation::UNIMPLEMENTED,
    /* cb73 */ operation::UNIMPLEMENTED,
    /* cb74 */ operation::UNIMPLEMENTED,
    /* cb75 */ operation::UNIMPLEMENTED,
    /* cb76 */ operation::UNIMPLEMENTED,
    /* cb77 */ operation::UNIMPLEMENTED,
    /* cb78 */ operation::UNIMPLEMENTED,
    /* cb79 */ operation::UNIMPLEMENTED,
    /* cb7a */ operation::UNIMPLEMENTED,
    /* cb7b */ operation::UNIMPLEMENTED,
    |_7c, gb| {
        let result = !u8_get_bit(gb.cpu.h, 7);
        gb.set_z_flag(result);
        gb.set_n_flag(false);
        gb.set_h_flag(true);
        op_execution!{
            cycles: 2;
            asm: "BIT 7, H";
            trace: "Z₁ = {}", result;
        }
    },
    /* cb7d */ operation::UNIMPLEMENTED,
    /* cb7e */ operation::UNIMPLEMENTED,
    /* cb7f */ operation::UNIMPLEMENTED,
    /* cb80 */ operation::UNIMPLEMENTED,
    /* cb81 */ operation::UNIMPLEMENTED,
    /* cb82 */ operation::UNIMPLEMENTED,
    /* cb83 */ operation::UNIMPLEMENTED,
    /* cb84 */ operation::UNIMPLEMENTED,
    /* cb85 */ operation::UNIMPLEMENTED,
    /* cb86 */ operation::UNIMPLEMENTED,
    /* cb87 */ operation::UNIMPLEMENTED,
    /* cb88 */ operation::UNIMPLEMENTED,
    /* cb89 */ operation::UNIMPLEMENTED,
    /* cb8a */ operation::UNIMPLEMENTED,
    /* cb8b */ operation::UNIMPLEMENTED,
    /* cb8c */ operation::UNIMPLEMENTED,
    /* cb8d */ operation::UNIMPLEMENTED,
    /* cb8e */ operation::UNIMPLEMENTED,
    /* cb8f */ operation::UNIMPLEMENTED,
    /* cb90 */ operation::UNIMPLEMENTED,
    /* cb91 */ operation::UNIMPLEMENTED,
    /* cb92 */ operation::UNIMPLEMENTED,
    /* cb93 */ operation::UNIMPLEMENTED,
    /* cb94 */ operation::UNIMPLEMENTED,
    /* cb95 */ operation::UNIMPLEMENTED,
    /* cb96 */ operation::UNIMPLEMENTED,
    /* cb97 */ operation::UNIMPLEMENTED,
    /* cb98 */ operation::UNIMPLEMENTED,
    /* cb99 */ operation::UNIMPLEMENTED,
    /* cb9a */ operation::UNIMPLEMENTED,
    /* cb9b */ operation::UNIMPLEMENTED,
    /* cb9c */ operation::UNIMPLEMENTED,
    /* cb9d */ operation::UNIMPLEMENTED,
    /* cb9e */ operation::UNIMPLEMENTED,
    /* cb9f */ operation::UNIMPLEMENTED,
    /* cba0 */ operation::UNIMPLEMENTED,
    /* cba1 */ operation::UNIMPLEMENTED,
    /* cba2 */ operation::UNIMPLEMENTED,
    /* cba3 */ operation::UNIMPLEMENTED,
    /* cba4 */ operation::UNIMPLEMENTED,
    /* cba5 */ operation::UNIMPLEMENTED,
    /* cba6 */ operation::UNIMPLEMENTED,
    /* cba7 */ operation::UNIMPLEMENTED,
    /* cba8 */ operation::UNIMPLEMENTED,
    /* cba9 */ operation::UNIMPLEMENTED,
    /* cbaa */ operation::UNIMPLEMENTED,
    /* cbab */ operation::UNIMPLEMENTED,
    /* cbac */ operation::UNIMPLEMENTED,
    /* cbad */ operation::UNIMPLEMENTED,
    /* cbae */ operation::UNIMPLEMENTED,
    /* cbaf */ operation::UNIMPLEMENTED,
    /* cbb0 */ operation::UNIMPLEMENTED,
    /* cbb1 */ operation::UNIMPLEMENTED,
    /* cbb2 */ operation::UNIMPLEMENTED,
    /* cbb3 */ operation::UNIMPLEMENTED,
    /* cbb4 */ operation::UNIMPLEMENTED,
    /* cbb5 */ operation::UNIMPLEMENTED,
    /* cbb6 */ operation::UNIMPLEMENTED,
    /* cbb7 */ operation::UNIMPLEMENTED,
    /* cbb8 */ operation::UNIMPLEMENTED,
    /* cbb9 */ operation::UNIMPLEMENTED,
    /* cbba */ operation::UNIMPLEMENTED,
    /* cbbb */ operation::UNIMPLEMENTED,
    /* cbbc */ operation::UNIMPLEMENTED,
    /* cbbd */ operation::UNIMPLEMENTED,
    /* cbbe */ operation::UNIMPLEMENTED,
    /* cbbf */ operation::UNIMPLEMENTED,
    /* cbc0 */ operation::UNIMPLEMENTED,
    /* cbc1 */ operation::UNIMPLEMENTED,
    /* cbc2 */ operation::UNIMPLEMENTED,
    /* cbc3 */ operation::UNIMPLEMENTED,
    /* cbc4 */ operation::UNIMPLEMENTED,
    /* cbc5 */ operation::UNIMPLEMENTED,
    /* cbc6 */ operation::UNIMPLEMENTED,
    /* cbc7 */ operation::UNIMPLEMENTED,
    /* cbc8 */ operation::UNIMPLEMENTED,
    /* cbc9 */ operation::UNIMPLEMENTED,
    /* cbca */ operation::UNIMPLEMENTED,
    /* cbcb */ operation::UNIMPLEMENTED,
    /* cbcc */ operation::UNIMPLEMENTED,
    /* cbcd */ operation::UNIMPLEMENTED,
    /* cbce */ operation::UNIMPLEMENTED,
    /* cbcf */ operation::UNIMPLEMENTED,
    /* cbd0 */ operation::UNIMPLEMENTED,
    /* cbd1 */ operation::UNIMPLEMENTED,
    /* cbd2 */ operation::UNIMPLEMENTED,
    /* cbd3 */ operation::UNIMPLEMENTED,
    /* cbd4 */ operation::UNIMPLEMENTED,
    /* cbd5 */ operation::UNIMPLEMENTED,
    /* cbd6 */ operation::UNIMPLEMENTED,
    /* cbd7 */ operation::UNIMPLEMENTED,
    /* cbd8 */ operation::UNIMPLEMENTED,
    /* cbd9 */ operation::UNIMPLEMENTED,
    /* cbda */ operation::UNIMPLEMENTED,
    /* cbdb */ operation::UNIMPLEMENTED,
    /* cbdc */ operation::UNIMPLEMENTED,
    /* cbdd */ operation::UNIMPLEMENTED,
    /* cbde */ operation::UNIMPLEMENTED,
    /* cbdf */ operation::UNIMPLEMENTED,
    /* cbe0 */ operation::UNIMPLEMENTED,
    /* cbe1 */ operation::UNIMPLEMENTED,
    /* cbe2 */ operation::UNIMPLEMENTED,
    /* cbe3 */ operation::UNIMPLEMENTED,
    /* cbe4 */ operation::UNIMPLEMENTED,
    /* cbe5 */ operation::UNIMPLEMENTED,
    /* cbe6 */ operation::UNIMPLEMENTED,
    /* cbe7 */ operation::UNIMPLEMENTED,
    /* cbe8 */ operation::UNIMPLEMENTED,
    /* cbe9 */ operation::UNIMPLEMENTED,
    /* cbea */ operation::UNIMPLEMENTED,
    /* cbeb */ operation::UNIMPLEMENTED,
    /* cbec */ operation::UNIMPLEMENTED,
    /* cbed */ operation::UNIMPLEMENTED,
    /* cbee */ operation::UNIMPLEMENTED,
    /* cbef */ operation::UNIMPLEMENTED,
    /* cbf0 */ operation::UNIMPLEMENTED,
    /* cbf1 */ operation::UNIMPLEMENTED,
    /* cbf2 */ operation::UNIMPLEMENTED,
    /* cbf3 */ operation::UNIMPLEMENTED,
    /* cbf4 */ operation::UNIMPLEMENTED,
    /* cbf5 */ operation::UNIMPLEMENTED,
    /* cbf6 */ operation::UNIMPLEMENTED,
    /* cbf7 */ operation::UNIMPLEMENTED,
    /* cbf8 */ operation::UNIMPLEMENTED,
    /* cbf9 */ operation::UNIMPLEMENTED,
    /* cbfa */ operation::UNIMPLEMENTED,
    /* cbfb */ operation::UNIMPLEMENTED,
    /* cbfc */ operation::UNIMPLEMENTED,
    /* cbfd */ operation::UNIMPLEMENTED,
    /* cbfe */ operation::UNIMPLEMENTED,
    /* cbff */ operation::UNIMPLEMENTED,
];
