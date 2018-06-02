use super::operation;

use emulator::cpu::CPU;

// 0xCB-prefixed two-byte opcodes
pub static OPCODES: [operation::OpFn; 0xFF] = [
    |_00, gb| unimplemented!("opcode 0xCB00 not implemented"),
    |_01, gb| unimplemented!("opcode 0xCB01 not implemented"),
    |_02, gb| unimplemented!("opcode 0xCB02 not implemented"),
    |_03, gb| unimplemented!("opcode 0xCB03 not implemented"),
    |_04, gb| unimplemented!("opcode 0xCB04 not implemented"),
    |_05, gb| unimplemented!("opcode 0xCB05 not implemented"),
    |_06, gb| unimplemented!("opcode 0xCB06 not implemented"),
    |_07, gb| unimplemented!("opcode 0xCB07 not implemented"),
    |_08, gb| unimplemented!("opcode 0xCB08 not implemented"),
    |_09, gb| unimplemented!("opcode 0xCB09 not implemented"),
    |_0a, gb| unimplemented!("opcode 0xCB0A not implemented"),
    |_0b, gb| unimplemented!("opcode 0xCB0B not implemented"),
    |_0c, gb| unimplemented!("opcode 0xCB0C not implemented"),
    |_0d, gb| unimplemented!("opcode 0xCB0D not implemented"),
    |_0e, gb| unimplemented!("opcode 0xCB0E not implemented"),
    |_0f, gb| unimplemented!("opcode 0xCB0F not implemented"),
    |_10, gb| {
        let b0 = gb.b;
        let b1 = b0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.b = b1;
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
        let c0 = gb.c;
        let c1 = c0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.c = c1;
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
        let d0 = gb.d;
        let d1 = d0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.d = d1;
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
        let e0 = gb.e;
        let e1 = e0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.e = e1;
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
        let h0 = gb.h;
        let h1 = h0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.h = h1;
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
        let l0 = gb.l;
        let l1 = l0 << 1 + if gb.c_flag() { 1 } else { 0 };
        gb.l = l1;
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
    |_16, gb| unimplemented!("opcode 0xCB16 not implemented"),
    |_17, gb| unimplemented!("opcode 0xCB17 not implemented"),
    |_18, gb| unimplemented!("opcode 0xCB18 not implemented"),
    |_19, gb| unimplemented!("opcode 0xCB19 not implemented"),
    |_1a, gb| unimplemented!("opcode 0xCB1A not implemented"),
    |_1b, gb| unimplemented!("opcode 0xCB1B not implemented"),
    |_1c, gb| unimplemented!("opcode 0xCB1C not implemented"),
    |_1d, gb| unimplemented!("opcode 0xCB1D not implemented"),
    |_1e, gb| unimplemented!("opcode 0xCB1E not implemented"),
    |_1f, gb| unimplemented!("opcode 0xCB1F not implemented"),
    |_20, gb| unimplemented!("opcode 0xCB20 not implemented"),
    |_21, gb| unimplemented!("opcode 0xCB21 not implemented"),
    |_22, gb| unimplemented!("opcode 0xCB22 not implemented"),
    |_23, gb| unimplemented!("opcode 0xCB23 not implemented"),
    |_24, gb| unimplemented!("opcode 0xCB24 not implemented"),
    |_25, gb| unimplemented!("opcode 0xCB25 not implemented"),
    |_26, gb| unimplemented!("opcode 0xCB26 not implemented"),
    |_27, gb| unimplemented!("opcode 0xCB27 not implemented"),
    |_28, gb| unimplemented!("opcode 0xCB28 not implemented"),
    |_29, gb| unimplemented!("opcode 0xCB29 not implemented"),
    |_2a, gb| unimplemented!("opcode 0xCB2A not implemented"),
    |_2b, gb| unimplemented!("opcode 0xCB2B not implemented"),
    |_2c, gb| unimplemented!("opcode 0xCB2C not implemented"),
    |_2d, gb| unimplemented!("opcode 0xCB2D not implemented"),
    |_2e, gb| unimplemented!("opcode 0xCB2E not implemented"),
    |_2f, gb| unimplemented!("opcode 0xCB2F not implemented"),
    |_30, gb| unimplemented!("opcode 0xCB30 not implemented"),
    |_31, gb| unimplemented!("opcode 0xCB31 not implemented"),
    |_32, gb| unimplemented!("opcode 0xCB32 not implemented"),
    |_33, gb| unimplemented!("opcode 0xCB33 not implemented"),
    |_34, gb| unimplemented!("opcode 0xCB34 not implemented"),
    |_35, gb| unimplemented!("opcode 0xCB35 not implemented"),
    |_36, gb| unimplemented!("opcode 0xCB36 not implemented"),
    |_37, gb| unimplemented!("opcode 0xCB37 not implemented"),
    |_38, gb| unimplemented!("opcode 0xCB38 not implemented"),
    |_39, gb| unimplemented!("opcode 0xCB39 not implemented"),
    |_3a, gb| unimplemented!("opcode 0xCB3A not implemented"),
    |_3b, gb| unimplemented!("opcode 0xCB3B not implemented"),
    |_3c, gb| unimplemented!("opcode 0xCB3C not implemented"),
    |_3d, gb| unimplemented!("opcode 0xCB3D not implemented"),
    |_3e, gb| unimplemented!("opcode 0xCB3E not implemented"),
    |_3f, gb| unimplemented!("opcode 0xCB3F not implemented"),
    |_40, gb| unimplemented!("opcode 0xCB40 not implemented"),
    |_41, gb| unimplemented!("opcode 0xCB41 not implemented"),
    |_42, gb| unimplemented!("opcode 0xCB42 not implemented"),
    |_43, gb| unimplemented!("opcode 0xCB43 not implemented"),
    |_44, gb| unimplemented!("opcode 0xCB44 not implemented"),
    |_45, gb| unimplemented!("opcode 0xCB45 not implemented"),
    |_46, gb| unimplemented!("opcode 0xCB46 not implemented"),
    |_47, gb| unimplemented!("opcode 0xCB47 not implemented"),
    |_48, gb| unimplemented!("opcode 0xCB48 not implemented"),
    |_49, gb| unimplemented!("opcode 0xCB49 not implemented"),
    |_4a, gb| unimplemented!("opcode 0xCB4A not implemented"),
    |_4b, gb| unimplemented!("opcode 0xCB4B not implemented"),
    |_4c, gb| unimplemented!("opcode 0xCB4C not implemented"),
    |_4d, gb| unimplemented!("opcode 0xCB4D not implemented"),
    |_4e, gb| unimplemented!("opcode 0xCB4E not implemented"),
    |_4f, gb| unimplemented!("opcode 0xCB4F not implemented"),
    |_50, gb| unimplemented!("opcode 0xCB50 not implemented"),
    |_51, gb| unimplemented!("opcode 0xCB51 not implemented"),
    |_52, gb| unimplemented!("opcode 0xCB52 not implemented"),
    |_53, gb| unimplemented!("opcode 0xCB53 not implemented"),
    |_54, gb| unimplemented!("opcode 0xCB54 not implemented"),
    |_55, gb| unimplemented!("opcode 0xCB55 not implemented"),
    |_56, gb| unimplemented!("opcode 0xCB56 not implemented"),
    |_57, gb| unimplemented!("opcode 0xCB57 not implemented"),
    |_58, gb| unimplemented!("opcode 0xCB58 not implemented"),
    |_59, gb| unimplemented!("opcode 0xCB59 not implemented"),
    |_5a, gb| unimplemented!("opcode 0xCB5A not implemented"),
    |_5b, gb| unimplemented!("opcode 0xCB5B not implemented"),
    |_5c, gb| unimplemented!("opcode 0xCB5C not implemented"),
    |_5d, gb| unimplemented!("opcode 0xCB5D not implemented"),
    |_5e, gb| unimplemented!("opcode 0xCB5E not implemented"),
    |_5f, gb| unimplemented!("opcode 0xCB5F not implemented"),
    |_60, gb| unimplemented!("opcode 0xCB60 not implemented"),
    |_61, gb| unimplemented!("opcode 0xCB61 not implemented"),
    |_62, gb| unimplemented!("opcode 0xCB62 not implemented"),
    |_63, gb| unimplemented!("opcode 0xCB63 not implemented"),
    |_64, gb| unimplemented!("opcode 0xCB64 not implemented"),
    |_65, gb| unimplemented!("opcode 0xCB65 not implemented"),
    |_66, gb| unimplemented!("opcode 0xCB66 not implemented"),
    |_67, gb| unimplemented!("opcode 0xCB67 not implemented"),
    |_68, gb| unimplemented!("opcode 0xCB68 not implemented"),
    |_69, gb| unimplemented!("opcode 0xCB69 not implemented"),
    |_6a, gb| unimplemented!("opcode 0xCB6A not implemented"),
    |_6b, gb| unimplemented!("opcode 0xCB6B not implemented"),
    |_6c, gb| unimplemented!("opcode 0xCB6C not implemented"),
    |_6d, gb| unimplemented!("opcode 0xCB6D not implemented"),
    |_6e, gb| unimplemented!("opcode 0xCB6E not implemented"),
    |_6f, gb| unimplemented!("opcode 0xCB6F not implemented"),
    |_70, gb| unimplemented!("opcode 0xCB70 not implemented"),
    |_71, gb| unimplemented!("opcode 0xCB71 not implemented"),
    |_72, gb| unimplemented!("opcode 0xCB72 not implemented"),
    |_73, gb| unimplemented!("opcode 0xCB73 not implemented"),
    |_74, gb| unimplemented!("opcode 0xCB74 not implemented"),
    |_75, gb| unimplemented!("opcode 0xCB75 not implemented"),
    |_76, gb| unimplemented!("opcode 0xCB76 not implemented"),
    |_77, gb| unimplemented!("opcode 0xCB77 not implemented"),
    |_78, gb| unimplemented!("opcode 0xCB78 not implemented"),
    |_79, gb| unimplemented!("opcode 0xCB79 not implemented"),
    |_7a, gb| unimplemented!("opcode 0xCB7A not implemented"),
    |_7b, gb| unimplemented!("opcode 0xCB7B not implemented"),
    |_7c, gb| {
        let result = !u8_get_bit(gb.h, 7);
        gb.set_z_flag(result);
        gb.set_n_flag(false);
        gb.set_h_flag(true);
        op_execution!{
            cycles: 2;
            asm: "BIT 7, H";
            trace: "Z₁ = {}", result;
        }
    },
    |_7d, gb| unimplemented!("opcode 0xCB7D not implemented"),
    |_7e, gb| unimplemented!("opcode 0xCB7E not implemented"),
    |_7f, gb| unimplemented!("opcode 0xCB7F not implemented"),
    |_80, gb| unimplemented!("opcode 0xCB80 not implemented"),
    |_81, gb| unimplemented!("opcode 0xCB81 not implemented"),
    |_82, gb| unimplemented!("opcode 0xCB82 not implemented"),
    |_83, gb| unimplemented!("opcode 0xCB83 not implemented"),
    |_84, gb| unimplemented!("opcode 0xCB84 not implemented"),
    |_85, gb| unimplemented!("opcode 0xCB85 not implemented"),
    |_86, gb| unimplemented!("opcode 0xCB86 not implemented"),
    |_87, gb| unimplemented!("opcode 0xCB87 not implemented"),
    |_88, gb| unimplemented!("opcode 0xCB88 not implemented"),
    |_89, gb| unimplemented!("opcode 0xCB89 not implemented"),
    |_8a, gb| unimplemented!("opcode 0xCB8A not implemented"),
    |_8b, gb| unimplemented!("opcode 0xCB8B not implemented"),
    |_8c, gb| unimplemented!("opcode 0xCB8C not implemented"),
    |_8d, gb| unimplemented!("opcode 0xCB8D not implemented"),
    |_8e, gb| unimplemented!("opcode 0xCB8E not implemented"),
    |_8f, gb| unimplemented!("opcode 0xCB8F not implemented"),
    |_90, gb| unimplemented!("opcode 0xCB90 not implemented"),
    |_91, gb| unimplemented!("opcode 0xCB91 not implemented"),
    |_92, gb| unimplemented!("opcode 0xCB92 not implemented"),
    |_93, gb| unimplemented!("opcode 0xCB93 not implemented"),
    |_94, gb| unimplemented!("opcode 0xCB94 not implemented"),
    |_95, gb| unimplemented!("opcode 0xCB95 not implemented"),
    |_96, gb| unimplemented!("opcode 0xCB96 not implemented"),
    |_97, gb| unimplemented!("opcode 0xCB97 not implemented"),
    |_98, gb| unimplemented!("opcode 0xCB98 not implemented"),
    |_99, gb| unimplemented!("opcode 0xCB99 not implemented"),
    |_9a, gb| unimplemented!("opcode 0xCB9A not implemented"),
    |_9b, gb| unimplemented!("opcode 0xCB9B not implemented"),
    |_9c, gb| unimplemented!("opcode 0xCB9C not implemented"),
    |_9d, gb| unimplemented!("opcode 0xCB9D not implemented"),
    |_9e, gb| unimplemented!("opcode 0xCB9E not implemented"),
    |_9f, gb| unimplemented!("opcode 0xCB9F not implemented"),
    |_a0, gb| unimplemented!("opcode 0xCBA0 not implemented"),
    |_a1, gb| unimplemented!("opcode 0xCBA1 not implemented"),
    |_a2, gb| unimplemented!("opcode 0xCBA2 not implemented"),
    |_a3, gb| unimplemented!("opcode 0xCBA3 not implemented"),
    |_a4, gb| unimplemented!("opcode 0xCBA4 not implemented"),
    |_a5, gb| unimplemented!("opcode 0xCBA5 not implemented"),
    |_a6, gb| unimplemented!("opcode 0xCBA6 not implemented"),
    |_a7, gb| unimplemented!("opcode 0xCBA7 not implemented"),
    |_a8, gb| unimplemented!("opcode 0xCBA8 not implemented"),
    |_a9, gb| unimplemented!("opcode 0xCBA9 not implemented"),
    |_aa, gb| unimplemented!("opcode 0xCBAA not implemented"),
    |_ab, gb| unimplemented!("opcode 0xCBAB not implemented"),
    |_ac, gb| unimplemented!("opcode 0xCBAC not implemented"),
    |_ad, gb| unimplemented!("opcode 0xCBAD not implemented"),
    |_ae, gb| unimplemented!("opcode 0xCBAE not implemented"),
    |_af, gb| unimplemented!("opcode 0xCBAF not implemented"),
    |_b0, gb| unimplemented!("opcode 0xCBB0 not implemented"),
    |_b1, gb| unimplemented!("opcode 0xCBB1 not implemented"),
    |_b2, gb| unimplemented!("opcode 0xCBB2 not implemented"),
    |_b3, gb| unimplemented!("opcode 0xCBB3 not implemented"),
    |_b4, gb| unimplemented!("opcode 0xCBB4 not implemented"),
    |_b5, gb| unimplemented!("opcode 0xCBB5 not implemented"),
    |_b6, gb| unimplemented!("opcode 0xCBB6 not implemented"),
    |_b7, gb| unimplemented!("opcode 0xCBB7 not implemented"),
    |_b8, gb| unimplemented!("opcode 0xCBB8 not implemented"),
    |_b9, gb| unimplemented!("opcode 0xCBB9 not implemented"),
    |_ba, gb| unimplemented!("opcode 0xCBBA not implemented"),
    |_bb, gb| unimplemented!("opcode 0xCBBB not implemented"),
    |_bc, gb| unimplemented!("opcode 0xCBBC not implemented"),
    |_bd, gb| unimplemented!("opcode 0xCBBD not implemented"),
    |_be, gb| unimplemented!("opcode 0xCBBE not implemented"),
    |_bf, gb| unimplemented!("opcode 0xCBBF not implemented"),
    |_c0, gb| unimplemented!("opcode 0xCBC0 not implemented"),
    |_c1, gb| unimplemented!("opcode 0xCBC1 not implemented"),
    |_c2, gb| unimplemented!("opcode 0xCBC2 not implemented"),
    |_c3, gb| unimplemented!("opcode 0xCBC3 not implemented"),
    |_c4, gb| unimplemented!("opcode 0xCBC4 not implemented"),
    |_c5, gb| unimplemented!("opcode 0xCBC5 not implemented"),
    |_c6, gb| unimplemented!("opcode 0xCBC6 not implemented"),
    |_c7, gb| unimplemented!("opcode 0xCBC7 not implemented"),
    |_c8, gb| unimplemented!("opcode 0xCBC8 not implemented"),
    |_c9, gb| unimplemented!("opcode 0xCBC9 not implemented"),
    |_ca, gb| unimplemented!("opcode 0xCBCA not implemented"),
    |_cb, gb| unimplemented!("opcode 0xCBCB not implemented"),
    |_cc, gb| unimplemented!("opcode 0xCBCC not implemented"),
    |_cd, gb| unimplemented!("opcode 0xCBCD not implemented"),
    |_ce, gb| unimplemented!("opcode 0xCBCE not implemented"),
    |_cf, gb| unimplemented!("opcode 0xCBCF not implemented"),
    |_d0, gb| unimplemented!("opcode 0xCBD0 not implemented"),
    |_d1, gb| unimplemented!("opcode 0xCBD1 not implemented"),
    |_d2, gb| unimplemented!("opcode 0xCBD2 not implemented"),
    |_d3, gb| unimplemented!("opcode 0xCBD3 not implemented"),
    |_d4, gb| unimplemented!("opcode 0xCBD4 not implemented"),
    |_d5, gb| unimplemented!("opcode 0xCBD5 not implemented"),
    |_d6, gb| unimplemented!("opcode 0xCBD6 not implemented"),
    |_d7, gb| unimplemented!("opcode 0xCBD7 not implemented"),
    |_d8, gb| unimplemented!("opcode 0xCBD8 not implemented"),
    |_d9, gb| unimplemented!("opcode 0xCBD9 not implemented"),
    |_da, gb| unimplemented!("opcode 0xCBDA not implemented"),
    |_db, gb| unimplemented!("opcode 0xCBDB not implemented"),
    |_dc, gb| unimplemented!("opcode 0xCBDC not implemented"),
    |_dd, gb| unimplemented!("opcode 0xCBDD not implemented"),
    |_de, gb| unimplemented!("opcode 0xCBDE not implemented"),
    |_df, gb| unimplemented!("opcode 0xCBDF not implemented"),
    |_e0, gb| unimplemented!("opcode 0xCBE0 not implemented"),
    |_e1, gb| unimplemented!("opcode 0xCBE1 not implemented"),
    |_e2, gb| unimplemented!("opcode 0xCBE2 not implemented"),
    |_e3, gb| unimplemented!("opcode 0xCBE3 not implemented"),
    |_e4, gb| unimplemented!("opcode 0xCBE4 not implemented"),
    |_e5, gb| unimplemented!("opcode 0xCBE5 not implemented"),
    |_e6, gb| unimplemented!("opcode 0xCBE6 not implemented"),
    |_e7, gb| unimplemented!("opcode 0xCBE7 not implemented"),
    |_e8, gb| unimplemented!("opcode 0xCBE8 not implemented"),
    |_e9, gb| unimplemented!("opcode 0xCBE9 not implemented"),
    |_ea, gb| unimplemented!("opcode 0xCBEA not implemented"),
    |_eb, gb| unimplemented!("opcode 0xCBEB not implemented"),
    |_ec, gb| unimplemented!("opcode 0xCBEC not implemented"),
    |_ed, gb| unimplemented!("opcode 0xCBED not implemented"),
    |_ee, gb| unimplemented!("opcode 0xCBEE not implemented"),
    |_ef, gb| unimplemented!("opcode 0xCBEF not implemented"),
    |_f0, gb| unimplemented!("opcode 0xCBF0 not implemented"),
    |_f1, gb| unimplemented!("opcode 0xCBF1 not implemented"),
    |_f2, gb| unimplemented!("opcode 0xCBF2 not implemented"),
    |_f3, gb| unimplemented!("opcode 0xCBF3 not implemented"),
    |_f4, gb| unimplemented!("opcode 0xCBF4 not implemented"),
    |_f5, gb| unimplemented!("opcode 0xCBF5 not implemented"),
    |_f6, gb| unimplemented!("opcode 0xCBF6 not implemented"),
    |_f7, gb| unimplemented!("opcode 0xCBF7 not implemented"),
    |_f8, gb| unimplemented!("opcode 0xCBF8 not implemented"),
    |_f9, gb| unimplemented!("opcode 0xCBF9 not implemented"),
    |_fa, gb| unimplemented!("opcode 0xCBFA not implemented"),
    |_fb, gb| unimplemented!("opcode 0xCBFB not implemented"),
    |_fc, gb| unimplemented!("opcode 0xCBFC not implemented"),
    |_fd, gb| unimplemented!("opcode 0xCBFD not implemented"),
    |_fe, gb| unimplemented!("opcode 0xCBFE not implemented"),
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
