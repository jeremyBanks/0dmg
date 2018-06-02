use super::operation;

use emulator::cpu::CPUController;

// 0xCB-prefixed two-byte opcodes
pub static OPCODES: [operation::OpFn; 0xFF] = [
    |_00, _gb| unimplemented!("opcode 0xCB00 not implemented"),
    |_01, _gb| unimplemented!("opcode 0xCB01 not implemented"),
    |_02, _gb| unimplemented!("opcode 0xCB02 not implemented"),
    |_03, _gb| unimplemented!("opcode 0xCB03 not implemented"),
    |_04, _gb| unimplemented!("opcode 0xCB04 not implemented"),
    |_05, _gb| unimplemented!("opcode 0xCB05 not implemented"),
    |_06, _gb| unimplemented!("opcode 0xCB06 not implemented"),
    |_07, _gb| unimplemented!("opcode 0xCB07 not implemented"),
    |_08, _gb| unimplemented!("opcode 0xCB08 not implemented"),
    |_09, _gb| unimplemented!("opcode 0xCB09 not implemented"),
    |_0a, _gb| unimplemented!("opcode 0xCB0A not implemented"),
    |_0b, _gb| unimplemented!("opcode 0xCB0B not implemented"),
    |_0c, _gb| unimplemented!("opcode 0xCB0C not implemented"),
    |_0d, _gb| unimplemented!("opcode 0xCB0D not implemented"),
    |_0e, _gb| unimplemented!("opcode 0xCB0E not implemented"),
    |_0f, _gb| unimplemented!("opcode 0xCB0F not implemented"),
    |_10, gb| {
        let b0 = gb.cpu.b;
        let b1 = b0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.cpu.b = b1;
        gb.set_z_flag(b1 == 0);
        gb.set_c_flag(b0 & 0b10000000 > 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL B";
            trace: "B₀ = {}", b0;
        }
    },
    |_11, gb| {
        let c0 = gb.cpu.c;
        let c1 = c0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.cpu.c = c1;
        gb.set_z_flag(c1 == 0);
        gb.set_c_flag(c0 & 0b10000000 > 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL C";
            trace: "C₀ = {}", c0;
        }
    },
    |_12, gb| {
        let d0 = gb.cpu.d;
        let d1 = d0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.cpu.d = d1;
        gb.set_z_flag(d1 == 0);
        gb.set_c_flag(d0 & 0b10000000 > 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL D";
            trace: "D₀ = {}", d0;
        }
    },
    |_13, gb| {
        let e0 = gb.cpu.e;
        let e1 = e0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.cpu.e = e1;
        gb.set_z_flag(e1 == 0);
        gb.set_c_flag(e0 & 0b10000000 > 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL E";
            trace: "E₀ = {}", e0;
        }
    },
    |_14, gb| {
        let h0 = gb.cpu.h;
        let h1 = h0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.cpu.h = h1;
        gb.set_z_flag(h1 == 0);
        gb.set_c_flag(h0 & 0b10000000 > 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL H";
            trace: "H₀ = {}", h0;
        }
    },
    |_15, gb| {
        let l0 = gb.cpu.l;
        let l1 = l0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.cpu.l = l1;
        gb.set_z_flag(l1 == 0);
        gb.set_c_flag(l0 & 0b10000000 > 0);
        gb.set_n_flag(false);
        gb.set_h_flag(false);
        op_execution!{
            cycles: 2;
            asm: "RL L";
            trace: "L₀ = {}", l0;
        }
    },
    |_16, _gb| unimplemented!("opcode 0xCB16 not implemented"),
    |_17, _gb| unimplemented!("opcode 0xCB17 not implemented"),
    |_18, _gb| unimplemented!("opcode 0xCB18 not implemented"),
    |_19, _gb| unimplemented!("opcode 0xCB19 not implemented"),
    |_1a, _gb| unimplemented!("opcode 0xCB1A not implemented"),
    |_1b, _gb| unimplemented!("opcode 0xCB1B not implemented"),
    |_1c, _gb| unimplemented!("opcode 0xCB1C not implemented"),
    |_1d, _gb| unimplemented!("opcode 0xCB1D not implemented"),
    |_1e, _gb| unimplemented!("opcode 0xCB1E not implemented"),
    |_1f, _gb| unimplemented!("opcode 0xCB1F not implemented"),
    |_20, _gb| unimplemented!("opcode 0xCB20 not implemented"),
    |_21, _gb| unimplemented!("opcode 0xCB21 not implemented"),
    |_22, _gb| unimplemented!("opcode 0xCB22 not implemented"),
    |_23, _gb| unimplemented!("opcode 0xCB23 not implemented"),
    |_24, _gb| unimplemented!("opcode 0xCB24 not implemented"),
    |_25, _gb| unimplemented!("opcode 0xCB25 not implemented"),
    |_26, _gb| unimplemented!("opcode 0xCB26 not implemented"),
    |_27, _gb| unimplemented!("opcode 0xCB27 not implemented"),
    |_28, _gb| unimplemented!("opcode 0xCB28 not implemented"),
    |_29, _gb| unimplemented!("opcode 0xCB29 not implemented"),
    |_2a, _gb| unimplemented!("opcode 0xCB2A not implemented"),
    |_2b, _gb| unimplemented!("opcode 0xCB2B not implemented"),
    |_2c, _gb| unimplemented!("opcode 0xCB2C not implemented"),
    |_2d, _gb| unimplemented!("opcode 0xCB2D not implemented"),
    |_2e, _gb| unimplemented!("opcode 0xCB2E not implemented"),
    |_2f, _gb| unimplemented!("opcode 0xCB2F not implemented"),
    |_30, _gb| unimplemented!("opcode 0xCB30 not implemented"),
    |_31, _gb| unimplemented!("opcode 0xCB31 not implemented"),
    |_32, _gb| unimplemented!("opcode 0xCB32 not implemented"),
    |_33, _gb| unimplemented!("opcode 0xCB33 not implemented"),
    |_34, _gb| unimplemented!("opcode 0xCB34 not implemented"),
    |_35, _gb| unimplemented!("opcode 0xCB35 not implemented"),
    |_36, _gb| unimplemented!("opcode 0xCB36 not implemented"),
    |_37, _gb| unimplemented!("opcode 0xCB37 not implemented"),
    |_38, _gb| unimplemented!("opcode 0xCB38 not implemented"),
    |_39, _gb| unimplemented!("opcode 0xCB39 not implemented"),
    |_3a, _gb| unimplemented!("opcode 0xCB3A not implemented"),
    |_3b, _gb| unimplemented!("opcode 0xCB3B not implemented"),
    |_3c, _gb| unimplemented!("opcode 0xCB3C not implemented"),
    |_3d, _gb| unimplemented!("opcode 0xCB3D not implemented"),
    |_3e, _gb| unimplemented!("opcode 0xCB3E not implemented"),
    |_3f, _gb| unimplemented!("opcode 0xCB3F not implemented"),
    |_40, _gb| unimplemented!("opcode 0xCB40 not implemented"),
    |_41, _gb| unimplemented!("opcode 0xCB41 not implemented"),
    |_42, _gb| unimplemented!("opcode 0xCB42 not implemented"),
    |_43, _gb| unimplemented!("opcode 0xCB43 not implemented"),
    |_44, _gb| unimplemented!("opcode 0xCB44 not implemented"),
    |_45, _gb| unimplemented!("opcode 0xCB45 not implemented"),
    |_46, _gb| unimplemented!("opcode 0xCB46 not implemented"),
    |_47, _gb| unimplemented!("opcode 0xCB47 not implemented"),
    |_48, _gb| unimplemented!("opcode 0xCB48 not implemented"),
    |_49, _gb| unimplemented!("opcode 0xCB49 not implemented"),
    |_4a, _gb| unimplemented!("opcode 0xCB4A not implemented"),
    |_4b, _gb| unimplemented!("opcode 0xCB4B not implemented"),
    |_4c, _gb| unimplemented!("opcode 0xCB4C not implemented"),
    |_4d, _gb| unimplemented!("opcode 0xCB4D not implemented"),
    |_4e, _gb| unimplemented!("opcode 0xCB4E not implemented"),
    |_4f, _gb| unimplemented!("opcode 0xCB4F not implemented"),
    |_50, _gb| unimplemented!("opcode 0xCB50 not implemented"),
    |_51, _gb| unimplemented!("opcode 0xCB51 not implemented"),
    |_52, _gb| unimplemented!("opcode 0xCB52 not implemented"),
    |_53, _gb| unimplemented!("opcode 0xCB53 not implemented"),
    |_54, _gb| unimplemented!("opcode 0xCB54 not implemented"),
    |_55, _gb| unimplemented!("opcode 0xCB55 not implemented"),
    |_56, _gb| unimplemented!("opcode 0xCB56 not implemented"),
    |_57, _gb| unimplemented!("opcode 0xCB57 not implemented"),
    |_58, _gb| unimplemented!("opcode 0xCB58 not implemented"),
    |_59, _gb| unimplemented!("opcode 0xCB59 not implemented"),
    |_5a, _gb| unimplemented!("opcode 0xCB5A not implemented"),
    |_5b, _gb| unimplemented!("opcode 0xCB5B not implemented"),
    |_5c, _gb| unimplemented!("opcode 0xCB5C not implemented"),
    |_5d, _gb| unimplemented!("opcode 0xCB5D not implemented"),
    |_5e, _gb| unimplemented!("opcode 0xCB5E not implemented"),
    |_5f, _gb| unimplemented!("opcode 0xCB5F not implemented"),
    |_60, _gb| unimplemented!("opcode 0xCB60 not implemented"),
    |_61, _gb| unimplemented!("opcode 0xCB61 not implemented"),
    |_62, _gb| unimplemented!("opcode 0xCB62 not implemented"),
    |_63, _gb| unimplemented!("opcode 0xCB63 not implemented"),
    |_64, _gb| unimplemented!("opcode 0xCB64 not implemented"),
    |_65, _gb| unimplemented!("opcode 0xCB65 not implemented"),
    |_66, _gb| unimplemented!("opcode 0xCB66 not implemented"),
    |_67, _gb| unimplemented!("opcode 0xCB67 not implemented"),
    |_68, _gb| unimplemented!("opcode 0xCB68 not implemented"),
    |_69, _gb| unimplemented!("opcode 0xCB69 not implemented"),
    |_6a, _gb| unimplemented!("opcode 0xCB6A not implemented"),
    |_6b, _gb| unimplemented!("opcode 0xCB6B not implemented"),
    |_6c, _gb| unimplemented!("opcode 0xCB6C not implemented"),
    |_6d, _gb| unimplemented!("opcode 0xCB6D not implemented"),
    |_6e, _gb| unimplemented!("opcode 0xCB6E not implemented"),
    |_6f, _gb| unimplemented!("opcode 0xCB6F not implemented"),
    |_70, _gb| unimplemented!("opcode 0xCB70 not implemented"),
    |_71, _gb| unimplemented!("opcode 0xCB71 not implemented"),
    |_72, _gb| unimplemented!("opcode 0xCB72 not implemented"),
    |_73, _gb| unimplemented!("opcode 0xCB73 not implemented"),
    |_74, _gb| unimplemented!("opcode 0xCB74 not implemented"),
    |_75, _gb| unimplemented!("opcode 0xCB75 not implemented"),
    |_76, _gb| unimplemented!("opcode 0xCB76 not implemented"),
    |_77, _gb| unimplemented!("opcode 0xCB77 not implemented"),
    |_78, _gb| unimplemented!("opcode 0xCB78 not implemented"),
    |_79, _gb| unimplemented!("opcode 0xCB79 not implemented"),
    |_7a, _gb| unimplemented!("opcode 0xCB7A not implemented"),
    |_7b, _gb| unimplemented!("opcode 0xCB7B not implemented"),
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
    |_7d, _gb| unimplemented!("opcode 0xCB7D not implemented"),
    |_7e, _gb| unimplemented!("opcode 0xCB7E not implemented"),
    |_7f, _gb| unimplemented!("opcode 0xCB7F not implemented"),
    |_80, _gb| unimplemented!("opcode 0xCB80 not implemented"),
    |_81, _gb| unimplemented!("opcode 0xCB81 not implemented"),
    |_82, _gb| unimplemented!("opcode 0xCB82 not implemented"),
    |_83, _gb| unimplemented!("opcode 0xCB83 not implemented"),
    |_84, _gb| unimplemented!("opcode 0xCB84 not implemented"),
    |_85, _gb| unimplemented!("opcode 0xCB85 not implemented"),
    |_86, _gb| unimplemented!("opcode 0xCB86 not implemented"),
    |_87, _gb| unimplemented!("opcode 0xCB87 not implemented"),
    |_88, _gb| unimplemented!("opcode 0xCB88 not implemented"),
    |_89, _gb| unimplemented!("opcode 0xCB89 not implemented"),
    |_8a, _gb| unimplemented!("opcode 0xCB8A not implemented"),
    |_8b, _gb| unimplemented!("opcode 0xCB8B not implemented"),
    |_8c, _gb| unimplemented!("opcode 0xCB8C not implemented"),
    |_8d, _gb| unimplemented!("opcode 0xCB8D not implemented"),
    |_8e, _gb| unimplemented!("opcode 0xCB8E not implemented"),
    |_8f, _gb| unimplemented!("opcode 0xCB8F not implemented"),
    |_90, _gb| unimplemented!("opcode 0xCB90 not implemented"),
    |_91, _gb| unimplemented!("opcode 0xCB91 not implemented"),
    |_92, _gb| unimplemented!("opcode 0xCB92 not implemented"),
    |_93, _gb| unimplemented!("opcode 0xCB93 not implemented"),
    |_94, _gb| unimplemented!("opcode 0xCB94 not implemented"),
    |_95, _gb| unimplemented!("opcode 0xCB95 not implemented"),
    |_96, _gb| unimplemented!("opcode 0xCB96 not implemented"),
    |_97, _gb| unimplemented!("opcode 0xCB97 not implemented"),
    |_98, _gb| unimplemented!("opcode 0xCB98 not implemented"),
    |_99, _gb| unimplemented!("opcode 0xCB99 not implemented"),
    |_9a, _gb| unimplemented!("opcode 0xCB9A not implemented"),
    |_9b, _gb| unimplemented!("opcode 0xCB9B not implemented"),
    |_9c, _gb| unimplemented!("opcode 0xCB9C not implemented"),
    |_9d, _gb| unimplemented!("opcode 0xCB9D not implemented"),
    |_9e, _gb| unimplemented!("opcode 0xCB9E not implemented"),
    |_9f, _gb| unimplemented!("opcode 0xCB9F not implemented"),
    |_a0, _gb| unimplemented!("opcode 0xCBA0 not implemented"),
    |_a1, _gb| unimplemented!("opcode 0xCBA1 not implemented"),
    |_a2, _gb| unimplemented!("opcode 0xCBA2 not implemented"),
    |_a3, _gb| unimplemented!("opcode 0xCBA3 not implemented"),
    |_a4, _gb| unimplemented!("opcode 0xCBA4 not implemented"),
    |_a5, _gb| unimplemented!("opcode 0xCBA5 not implemented"),
    |_a6, _gb| unimplemented!("opcode 0xCBA6 not implemented"),
    |_a7, _gb| unimplemented!("opcode 0xCBA7 not implemented"),
    |_a8, _gb| unimplemented!("opcode 0xCBA8 not implemented"),
    |_a9, _gb| unimplemented!("opcode 0xCBA9 not implemented"),
    |_aa, _gb| unimplemented!("opcode 0xCBAA not implemented"),
    |_ab, _gb| unimplemented!("opcode 0xCBAB not implemented"),
    |_ac, _gb| unimplemented!("opcode 0xCBAC not implemented"),
    |_ad, _gb| unimplemented!("opcode 0xCBAD not implemented"),
    |_ae, _gb| unimplemented!("opcode 0xCBAE not implemented"),
    |_af, _gb| unimplemented!("opcode 0xCBAF not implemented"),
    |_b0, _gb| unimplemented!("opcode 0xCBB0 not implemented"),
    |_b1, _gb| unimplemented!("opcode 0xCBB1 not implemented"),
    |_b2, _gb| unimplemented!("opcode 0xCBB2 not implemented"),
    |_b3, _gb| unimplemented!("opcode 0xCBB3 not implemented"),
    |_b4, _gb| unimplemented!("opcode 0xCBB4 not implemented"),
    |_b5, _gb| unimplemented!("opcode 0xCBB5 not implemented"),
    |_b6, _gb| unimplemented!("opcode 0xCBB6 not implemented"),
    |_b7, _gb| unimplemented!("opcode 0xCBB7 not implemented"),
    |_b8, _gb| unimplemented!("opcode 0xCBB8 not implemented"),
    |_b9, _gb| unimplemented!("opcode 0xCBB9 not implemented"),
    |_ba, _gb| unimplemented!("opcode 0xCBBA not implemented"),
    |_bb, _gb| unimplemented!("opcode 0xCBBB not implemented"),
    |_bc, _gb| unimplemented!("opcode 0xCBBC not implemented"),
    |_bd, _gb| unimplemented!("opcode 0xCBBD not implemented"),
    |_be, _gb| unimplemented!("opcode 0xCBBE not implemented"),
    |_bf, _gb| unimplemented!("opcode 0xCBBF not implemented"),
    |_c0, _gb| unimplemented!("opcode 0xCBC0 not implemented"),
    |_c1, _gb| unimplemented!("opcode 0xCBC1 not implemented"),
    |_c2, _gb| unimplemented!("opcode 0xCBC2 not implemented"),
    |_c3, _gb| unimplemented!("opcode 0xCBC3 not implemented"),
    |_c4, _gb| unimplemented!("opcode 0xCBC4 not implemented"),
    |_c5, _gb| unimplemented!("opcode 0xCBC5 not implemented"),
    |_c6, _gb| unimplemented!("opcode 0xCBC6 not implemented"),
    |_c7, _gb| unimplemented!("opcode 0xCBC7 not implemented"),
    |_c8, _gb| unimplemented!("opcode 0xCBC8 not implemented"),
    |_c9, _gb| unimplemented!("opcode 0xCBC9 not implemented"),
    |_ca, _gb| unimplemented!("opcode 0xCBCA not implemented"),
    |_cb, _gb| unimplemented!("opcode 0xCBCB not implemented"),
    |_cc, _gb| unimplemented!("opcode 0xCBCC not implemented"),
    |_cd, _gb| unimplemented!("opcode 0xCBCD not implemented"),
    |_ce, _gb| unimplemented!("opcode 0xCBCE not implemented"),
    |_cf, _gb| unimplemented!("opcode 0xCBCF not implemented"),
    |_d0, _gb| unimplemented!("opcode 0xCBD0 not implemented"),
    |_d1, _gb| unimplemented!("opcode 0xCBD1 not implemented"),
    |_d2, _gb| unimplemented!("opcode 0xCBD2 not implemented"),
    |_d3, _gb| unimplemented!("opcode 0xCBD3 not implemented"),
    |_d4, _gb| unimplemented!("opcode 0xCBD4 not implemented"),
    |_d5, _gb| unimplemented!("opcode 0xCBD5 not implemented"),
    |_d6, _gb| unimplemented!("opcode 0xCBD6 not implemented"),
    |_d7, _gb| unimplemented!("opcode 0xCBD7 not implemented"),
    |_d8, _gb| unimplemented!("opcode 0xCBD8 not implemented"),
    |_d9, _gb| unimplemented!("opcode 0xCBD9 not implemented"),
    |_da, _gb| unimplemented!("opcode 0xCBDA not implemented"),
    |_db, _gb| unimplemented!("opcode 0xCBDB not implemented"),
    |_dc, _gb| unimplemented!("opcode 0xCBDC not implemented"),
    |_dd, _gb| unimplemented!("opcode 0xCBDD not implemented"),
    |_de, _gb| unimplemented!("opcode 0xCBDE not implemented"),
    |_df, _gb| unimplemented!("opcode 0xCBDF not implemented"),
    |_e0, _gb| unimplemented!("opcode 0xCBE0 not implemented"),
    |_e1, _gb| unimplemented!("opcode 0xCBE1 not implemented"),
    |_e2, _gb| unimplemented!("opcode 0xCBE2 not implemented"),
    |_e3, _gb| unimplemented!("opcode 0xCBE3 not implemented"),
    |_e4, _gb| unimplemented!("opcode 0xCBE4 not implemented"),
    |_e5, _gb| unimplemented!("opcode 0xCBE5 not implemented"),
    |_e6, _gb| unimplemented!("opcode 0xCBE6 not implemented"),
    |_e7, _gb| unimplemented!("opcode 0xCBE7 not implemented"),
    |_e8, _gb| unimplemented!("opcode 0xCBE8 not implemented"),
    |_e9, _gb| unimplemented!("opcode 0xCBE9 not implemented"),
    |_ea, _gb| unimplemented!("opcode 0xCBEA not implemented"),
    |_eb, _gb| unimplemented!("opcode 0xCBEB not implemented"),
    |_ec, _gb| unimplemented!("opcode 0xCBEC not implemented"),
    |_ed, _gb| unimplemented!("opcode 0xCBED not implemented"),
    |_ee, _gb| unimplemented!("opcode 0xCBEE not implemented"),
    |_ef, _gb| unimplemented!("opcode 0xCBEF not implemented"),
    |_f0, _gb| unimplemented!("opcode 0xCBF0 not implemented"),
    |_f1, _gb| unimplemented!("opcode 0xCBF1 not implemented"),
    |_f2, _gb| unimplemented!("opcode 0xCBF2 not implemented"),
    |_f3, _gb| unimplemented!("opcode 0xCBF3 not implemented"),
    |_f4, _gb| unimplemented!("opcode 0xCBF4 not implemented"),
    |_f5, _gb| unimplemented!("opcode 0xCBF5 not implemented"),
    |_f6, _gb| unimplemented!("opcode 0xCBF6 not implemented"),
    |_f7, _gb| unimplemented!("opcode 0xCBF7 not implemented"),
    |_f8, _gb| unimplemented!("opcode 0xCBF8 not implemented"),
    |_f9, _gb| unimplemented!("opcode 0xCBF9 not implemented"),
    |_fa, _gb| unimplemented!("opcode 0xCBFA not implemented"),
    |_fb, _gb| unimplemented!("opcode 0xCBFB not implemented"),
    |_fc, _gb| unimplemented!("opcode 0xCBFC not implemented"),
    |_fd, _gb| unimplemented!("opcode 0xCBFD not implemented"),
    |_fe, _gb| unimplemented!("opcode 0xCBFE not implemented"),
];

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
