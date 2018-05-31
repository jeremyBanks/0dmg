use std::collections::HashMap;

#[derive(Debug)]
pub struct CPU {
    // clock ticks
    t: u64,
    // instruction pointer
    i: u16,

    // A accumulator register
    a: u8,
    // F flags register
    f: u8,
    // BC register/B and C registers
    b: u8,
    c: u8,
    // DE register/D and E registers
    d: u8,
    e: u8,
    // HL register/H and L registers
    h: u8,
    l: u8,
    // SP stack pointer register
    sp: u16,
    // PC program counter register
    pc: u16,
}

type OperationFn = fn(
    opcode: u8,
    cpu: &mut CPU,
    mem: &mut MemoryMap,
) -> Result<OperationExecution, ()>;

#[derive(Debug)]
struct OperationExecution {
    dt: u64, // number of cycles elapsed
    asm: str, // generated pseudo-asm
    debug: Optional<str>, // some readable debug data
}

// supported one-byte opcodes
static opcodes: [OperationFn; 0xFF] = [
    |_00, cpu, mem| { 
        OperationExecution {
            dt: 1,
            asm: "NOP",
            debug: None
        }
    },
    |_01, cpu, mem| { unimplemented!() },
    |_02, cpu, mem| { unimplemented!() },
    |_03, cpu, mem| { unimplemented!() },
    |_04, cpu, mem| { unimplemented!() },
    |_05, cpu, mem| { unimplemented!() },
    |_06, cpu, mem| { 
        let b0 = cpu.b;
        let b1 = read_immediate_u8(cpu, mem);
        cpu.b = b1;
        OperationExecution {
            dt: 2,
            asm: &format!("LD B, ${:02x}", b1),
            debug: &format!("B₀ = ${:02x}, B₁ = ${:02x}", b0, b1)
        }
    },
    |_07, cpu, mem| { unimplemented!() },
    |_08, cpu, mem| { unimplemented!() },
    |_09, cpu, mem| { unimplemented!() },
    |_0A, cpu, mem| { unimplemented!() },
    |_0B, cpu, mem| { unimplemented!() },
    |_0C, cpu, mem| { unimplemented!() },
    |_0D, cpu, mem| { unimplemented!() },
    |_0E, cpu, mem| { unimplemented!() },
    |_0F, cpu, mem| { unimplemented!() },
    |_10, cpu, mem| { unimplemented!() },
    |_11, cpu, mem| { unimplemented!() },
    |_12, cpu, mem| { unimplemented!() },
    |_13, cpu, mem| { unimplemented!() },
    |_14, cpu, mem| { unimplemented!() },
    |_15, cpu, mem| { unimplemented!() },
    |_16, cpu, mem| { unimplemented!() },
    |_17, cpu, mem| { unimplemented!() },
    |_18, cpu, mem| { unimplemented!() },
    |_19, cpu, mem| { unimplemented!() },
    |_1A, cpu, mem| { unimplemented!() },
    |_1B, cpu, mem| { unimplemented!() },
    |_1C, cpu, mem| { unimplemented!() },
    |_1D, cpu, mem| { unimplemented!() },
    |_1E, cpu, mem| { unimplemented!() },
    |_1F, cpu, mem| { unimplemented!() },
    |_20, cpu, mem| { unimplemented!() },
    |_21, cpu, mem| { unimplemented!() },
    |_22, cpu, mem| { unimplemented!() },
    |_23, cpu, mem| { unimplemented!() },
    |_24, cpu, mem| { unimplemented!() },
    |_25, cpu, mem| { unimplemented!() },
    |_26, cpu, mem| { unimplemented!() },
    |_27, cpu, mem| { unimplemented!() },
    |_28, cpu, mem| { unimplemented!() },
    |_29, cpu, mem| { unimplemented!() },
    |_2A, cpu, mem| { unimplemented!() },
    |_2B, cpu, mem| { unimplemented!() },
    |_2C, cpu, mem| { unimplemented!() },
    |_2D, cpu, mem| { unimplemented!() },
    |_2E, cpu, mem| { unimplemented!() },
    |_2F, cpu, mem| { unimplemented!() },
    |_30, cpu, mem| { unimplemented!() },
    |_31, cpu, mem| { unimplemented!() },
    |_32, cpu, mem| { unimplemented!() },
    |_33, cpu, mem| { unimplemented!() },
    |_34, cpu, mem| { unimplemented!() },
    |_35, cpu, mem| { unimplemented!() },
    |_36, cpu, mem| { unimplemented!() },
    |_37, cpu, mem| { unimplemented!() },
    |_38, cpu, mem| { unimplemented!() },
    |_39, cpu, mem| { unimplemented!() },
    |_3A, cpu, mem| { unimplemented!() },
    |_3B, cpu, mem| { unimplemented!() },
    |_3C, cpu, mem| { unimplemented!() },
    |_3D, cpu, mem| { unimplemented!() },
    |_3E, cpu, mem| { unimplemented!() },
    |_3F, cpu, mem| { unimplemented!() },
    |_40, cpu, mem| { unimplemented!() },
    |_41, cpu, mem| { unimplemented!() },
    |_42, cpu, mem| { unimplemented!() },
    |_43, cpu, mem| { unimplemented!() },
    |_44, cpu, mem| { unimplemented!() },
    |_45, cpu, mem| { unimplemented!() },
    |_46, cpu, mem| { unimplemented!() },
    |_47, cpu, mem| { unimplemented!() },
    |_48, cpu, mem| { unimplemented!() },
    |_49, cpu, mem| { unimplemented!() },
    |_4A, cpu, mem| { unimplemented!() },
    |_4B, cpu, mem| { unimplemented!() },
    |_4C, cpu, mem| { unimplemented!() },
    |_4D, cpu, mem| { unimplemented!() },
    |_4E, cpu, mem| { unimplemented!() },
    |_4F, cpu, mem| { unimplemented!() },
    |_50, cpu, mem| { unimplemented!() },
    |_51, cpu, mem| { unimplemented!() },
    |_52, cpu, mem| { unimplemented!() },
    |_53, cpu, mem| { unimplemented!() },
    |_54, cpu, mem| { unimplemented!() },
    |_55, cpu, mem| { unimplemented!() },
    |_56, cpu, mem| { unimplemented!() },
    |_57, cpu, mem| { unimplemented!() },
    |_58, cpu, mem| { unimplemented!() },
    |_59, cpu, mem| { unimplemented!() },
    |_5A, cpu, mem| { unimplemented!() },
    |_5B, cpu, mem| { unimplemented!() },
    |_5C, cpu, mem| { unimplemented!() },
    |_5D, cpu, mem| { unimplemented!() },
    |_5E, cpu, mem| { unimplemented!() },
    |_5F, cpu, mem| { unimplemented!() },
    |_60, cpu, mem| { unimplemented!() },
    |_61, cpu, mem| { unimplemented!() },
    |_62, cpu, mem| { unimplemented!() },
    |_63, cpu, mem| { unimplemented!() },
    |_64, cpu, mem| { unimplemented!() },
    |_65, cpu, mem| { unimplemented!() },
    |_66, cpu, mem| { unimplemented!() },
    |_67, cpu, mem| { unimplemented!() },
    |_68, cpu, mem| { unimplemented!() },
    |_69, cpu, mem| { unimplemented!() },
    |_6A, cpu, mem| { unimplemented!() },
    |_6B, cpu, mem| { unimplemented!() },
    |_6C, cpu, mem| { unimplemented!() },
    |_6D, cpu, mem| { unimplemented!() },
    |_6E, cpu, mem| { unimplemented!() },
    |_6F, cpu, mem| { unimplemented!() },
    |_70, cpu, mem| { unimplemented!() },
    |_71, cpu, mem| { unimplemented!() },
    |_72, cpu, mem| { unimplemented!() },
    |_73, cpu, mem| { unimplemented!() },
    |_74, cpu, mem| { unimplemented!() },
    |_75, cpu, mem| { unimplemented!() },
    |_76, cpu, mem| { unimplemented!() },
    |_77, cpu, mem| { unimplemented!() },
    |_78, cpu, mem| { unimplemented!() },
    |_79, cpu, mem| { unimplemented!() },
    |_7A, cpu, mem| { unimplemented!() },
    |_7B, cpu, mem| { unimplemented!() },
    |_7C, cpu, mem| { unimplemented!() },
    |_7D, cpu, mem| { unimplemented!() },
    |_7E, cpu, mem| { unimplemented!() },
    |_7F, cpu, mem| { unimplemented!() },
    |_80, cpu, mem| { unimplemented!() },
    |_81, cpu, mem| { unimplemented!() },
    |_82, cpu, mem| { unimplemented!() },
    |_83, cpu, mem| { unimplemented!() },
    |_84, cpu, mem| { unimplemented!() },
    |_85, cpu, mem| { unimplemented!() },
    |_86, cpu, mem| { unimplemented!() },
    |_87, cpu, mem| { unimplemented!() },
    |_88, cpu, mem| { unimplemented!() },
    |_89, cpu, mem| { unimplemented!() },
    |_8A, cpu, mem| { unimplemented!() },
    |_8B, cpu, mem| { unimplemented!() },
    |_8C, cpu, mem| { unimplemented!() },
    |_8D, cpu, mem| { unimplemented!() },
    |_8E, cpu, mem| { unimplemented!() },
    |_8F, cpu, mem| { unimplemented!() },
    |_90, cpu, mem| { unimplemented!() },
    |_91, cpu, mem| { unimplemented!() },
    |_92, cpu, mem| { unimplemented!() },
    |_93, cpu, mem| { unimplemented!() },
    |_94, cpu, mem| { unimplemented!() },
    |_95, cpu, mem| { unimplemented!() },
    |_96, cpu, mem| { unimplemented!() },
    |_97, cpu, mem| { unimplemented!() },
    |_98, cpu, mem| { unimplemented!() },
    |_99, cpu, mem| { unimplemented!() },
    |_9A, cpu, mem| { unimplemented!() },
    |_9B, cpu, mem| { unimplemented!() },
    |_9C, cpu, mem| { unimplemented!() },
    |_9D, cpu, mem| { unimplemented!() },
    |_9E, cpu, mem| { unimplemented!() },
    |_9F, cpu, mem| { unimplemented!() },
    |_A0, cpu, mem| { unimplemented!() },
    |_A1, cpu, mem| { unimplemented!() },
    |_A2, cpu, mem| { unimplemented!() },
    |_A3, cpu, mem| { unimplemented!() },
    |_A4, cpu, mem| { unimplemented!() },
    |_A5, cpu, mem| { unimplemented!() },
    |_A6, cpu, mem| { unimplemented!() },
    |_A7, cpu, mem| { unimplemented!() },
    |_A8, cpu, mem| { unimplemented!() },
    |_A9, cpu, mem| { unimplemented!() },
    |_AA, cpu, mem| { unimplemented!() },
    |_AB, cpu, mem| { unimplemented!() },
    |_AC, cpu, mem| { unimplemented!() },
    |_AD, cpu, mem| { unimplemented!() },
    |_AE, cpu, mem| { unimplemented!() },
    |_AF, cpu, mem| { unimplemented!() },
    |_B0, cpu, mem| { unimplemented!() },
    |_B1, cpu, mem| { unimplemented!() },
    |_B2, cpu, mem| { unimplemented!() },
    |_B3, cpu, mem| { unimplemented!() },
    |_B4, cpu, mem| { unimplemented!() },
    |_B5, cpu, mem| { unimplemented!() },
    |_B6, cpu, mem| { unimplemented!() },
    |_B7, cpu, mem| { unimplemented!() },
    |_B8, cpu, mem| { unimplemented!() },
    |_B9, cpu, mem| { unimplemented!() },
    |_BA, cpu, mem| { unimplemented!() },
    |_BB, cpu, mem| { unimplemented!() },
    |_BC, cpu, mem| { unimplemented!() },
    |_BD, cpu, mem| { unimplemented!() },
    |_BE, cpu, mem| { unimplemented!() },
    |_BF, cpu, mem| { unimplemented!() },
    |_C0, cpu, mem| { unimplemented!() },
    |_C1, cpu, mem| { unimplemented!() },
    |_C2, cpu, mem| { unimplemented!() },
    |_C3, cpu, mem| { unimplemented!() },
    |_C4, cpu, mem| { unimplemented!() },
    |_C5, cpu, mem| { unimplemented!() },
    |_C6, cpu, mem| { unimplemented!() },
    |_C7, cpu, mem| { unimplemented!() },
    |_C8, cpu, mem| { unimplemented!() },
    |_C9, cpu, mem| { unimplemented!() },
    |_CA, cpu, mem| { unimplemented!() },
    |_CB, cpu, mem| {
        panic!("0xCB prefix is not a complete opcode");
    },
    |_CC, cpu, mem| { unimplemented!() },
    |_CD, cpu, mem| { unimplemented!() },
    |_CE, cpu, mem| { unimplemented!() },
    |_CF, cpu, mem| { unimplemented!() },
    |_D0, cpu, mem| { unimplemented!() },
    |_D1, cpu, mem| { unimplemented!() },
    |_D2, cpu, mem| { unimplemented!() },
    |_D3, cpu, mem| {
        panic!("0xD3 is not a valid opcode");
    },
    |_D4, cpu, mem| { unimplemented!() },
    |_D5, cpu, mem| { unimplemented!() },
    |_D6, cpu, mem| { unimplemented!() },
    |_D7, cpu, mem| { unimplemented!() },
    |_D8, cpu, mem| { unimplemented!() },
    |_D9, cpu, mem| { unimplemented!() },
    |_Da, cpu, mem| { unimplemented!() },
    |_Db, cpu, mem| { 
        panic!("0xDB is not a valid opcode");
     },
    |_DC, cpu, mem| { unimplemented!() },
    |_DD, cpu, mem| { 
        panic!("0xDD is not a valid opcode");
     },
    |_DE, cpu, mem| { unimplemented!() },
    |_DF, cpu, mem| { unimplemented!() },
    |_E0, cpu, mem| { unimplemented!() },
    |_E1, cpu, mem| { unimplemented!() },
    |_E2, cpu, mem| { unimplemented!() },
    |_E3, cpu, mem| {
        panic!("0xE3 is not a valid opcode");
    },
    |_E4, cpu, mem| {
        panic!("0xE4 is not a valid opcode");
    },
    |_E5, cpu, mem| { unimplemented!() },
    |_E6, cpu, mem| { unimplemented!() },
    |_E7, cpu, mem| { unimplemented!() },
    |_E8, cpu, mem| { unimplemented!() },
    |_E9, cpu, mem| { unimplemented!() },
    |_Ea, cpu, mem| { unimplemented!() },
    |_Eb, cpu, mem| { 
        panic!("0xEB is not a valid opcode");
     },
    |_EC, cpu, mem| { 
        panic!("0xEC is not a valid opcode");
     },
    |_ED, cpu, mem| { 
        panic!("0xED is not a valid opcode");
     },
    |_EE, cpu, mem| { unimplemented!() },
    |_EF, cpu, mem| { unimplemented!() },
    |_F0, cpu, mem| { unimplemented!() },
    |_F1, cpu, mem| { unimplemented!() },
    |_F2, cpu, mem| { unimplemented!() },
    |_F3, cpu, mem| { unimplemented!() },
    |_F4, cpu, mem| {
        panic!("0xF4 is not a valid opcode");
    },
    |_F5, cpu, mem| { unimplemented!() },
    |_F6, cpu, mem| { unimplemented!() },
    |_F7, cpu, mem| { unimplemented!() },
    |_F8, cpu, mem| { unimplemented!() },
    |_F9, cpu, mem| { unimplemented!() },
    |_FA, cpu, mem| { unimplemented!() },
    |_FB, cpu, mem| { unimplemented!() },
    |_FC, cpu, mem| { 
        panic!("0xFC is not a valid opcode");
     },
    |_FD, cpu, mem| { 
        panic!("0xFD is not a valid opcode");
     },
    |_FE, cpu, mem| { unimplemented!() },
];

// supported 0xCB-prefixed two-byte opcodes
static opcodes_cb: [OperationFn; 0xFF] =  [
    |_00, cpu, mem| { unimplemented!() },
    |_01, cpu, mem| { unimplemented!() },
    |_02, cpu, mem| { unimplemented!() },
    |_03, cpu, mem| { unimplemented!() },
    |_04, cpu, mem| { unimplemented!() },
    |_05, cpu, mem| { unimplemented!() },
    |_06, cpu, mem| { unimplemented!() },
    |_07, cpu, mem| { unimplemented!() },
    |_08, cpu, mem| { unimplemented!() },
    |_09, cpu, mem| { unimplemented!() },
    |_0A, cpu, mem| { unimplemented!() },
    |_0B, cpu, mem| { unimplemented!() },
    |_0C, cpu, mem| { unimplemented!() },
    |_0D, cpu, mem| { unimplemented!() },
    |_0E, cpu, mem| { unimplemented!() },
    |_0F, cpu, mem| { unimplemented!() },
    |_10, cpu, mem| { unimplemented!() },
    |_11, cpu, mem| { unimplemented!() },
    |_12, cpu, mem| { unimplemented!() },
    |_13, cpu, mem| { unimplemented!() },
    |_14, cpu, mem| { unimplemented!() },
    |_15, cpu, mem| { unimplemented!() },
    |_16, cpu, mem| { unimplemented!() },
    |_17, cpu, mem| { unimplemented!() },
    |_18, cpu, mem| { unimplemented!() },
    |_19, cpu, mem| { unimplemented!() },
    |_1A, cpu, mem| { unimplemented!() },
    |_1B, cpu, mem| { unimplemented!() },
    |_1C, cpu, mem| { unimplemented!() },
    |_1D, cpu, mem| { unimplemented!() },
    |_1E, cpu, mem| { unimplemented!() },
    |_1F, cpu, mem| { unimplemented!() },
    |_20, cpu, mem| { unimplemented!() },
    |_21, cpu, mem| { unimplemented!() },
    |_22, cpu, mem| { unimplemented!() },
    |_23, cpu, mem| { unimplemented!() },
    |_24, cpu, mem| { unimplemented!() },
    |_25, cpu, mem| { unimplemented!() },
    |_26, cpu, mem| { unimplemented!() },
    |_27, cpu, mem| { unimplemented!() },
    |_28, cpu, mem| { unimplemented!() },
    |_29, cpu, mem| { unimplemented!() },
    |_2A, cpu, mem| { unimplemented!() },
    |_2B, cpu, mem| { unimplemented!() },
    |_2C, cpu, mem| { unimplemented!() },
    |_2D, cpu, mem| { unimplemented!() },
    |_2E, cpu, mem| { unimplemented!() },
    |_2F, cpu, mem| { unimplemented!() },
    |_30, cpu, mem| { unimplemented!() },
    |_31, cpu, mem| { unimplemented!() },
    |_32, cpu, mem| { unimplemented!() },
    |_33, cpu, mem| { unimplemented!() },
    |_34, cpu, mem| { unimplemented!() },
    |_35, cpu, mem| { unimplemented!() },
    |_36, cpu, mem| { unimplemented!() },
    |_37, cpu, mem| { unimplemented!() },
    |_38, cpu, mem| { unimplemented!() },
    |_39, cpu, mem| { unimplemented!() },
    |_3A, cpu, mem| { unimplemented!() },
    |_3B, cpu, mem| { unimplemented!() },
    |_3C, cpu, mem| { unimplemented!() },
    |_3D, cpu, mem| { unimplemented!() },
    |_3E, cpu, mem| { unimplemented!() },
    |_3F, cpu, mem| { unimplemented!() },
    |_40, cpu, mem| { unimplemented!() },
    |_41, cpu, mem| { unimplemented!() },
    |_42, cpu, mem| { unimplemented!() },
    |_43, cpu, mem| { unimplemented!() },
    |_44, cpu, mem| { unimplemented!() },
    |_45, cpu, mem| { unimplemented!() },
    |_46, cpu, mem| { unimplemented!() },
    |_47, cpu, mem| { unimplemented!() },
    |_48, cpu, mem| { unimplemented!() },
    |_49, cpu, mem| { unimplemented!() },
    |_4A, cpu, mem| { unimplemented!() },
    |_4B, cpu, mem| { unimplemented!() },
    |_4C, cpu, mem| { unimplemented!() },
    |_4D, cpu, mem| { unimplemented!() },
    |_4E, cpu, mem| { unimplemented!() },
    |_4F, cpu, mem| { unimplemented!() },
    |_50, cpu, mem| { unimplemented!() },
    |_51, cpu, mem| { unimplemented!() },
    |_52, cpu, mem| { unimplemented!() },
    |_53, cpu, mem| { unimplemented!() },
    |_54, cpu, mem| { unimplemented!() },
    |_55, cpu, mem| { unimplemented!() },
    |_56, cpu, mem| { unimplemented!() },
    |_57, cpu, mem| { unimplemented!() },
    |_58, cpu, mem| { unimplemented!() },
    |_59, cpu, mem| { unimplemented!() },
    |_5A, cpu, mem| { unimplemented!() },
    |_5B, cpu, mem| { unimplemented!() },
    |_5C, cpu, mem| { unimplemented!() },
    |_5D, cpu, mem| { unimplemented!() },
    |_5E, cpu, mem| { unimplemented!() },
    |_5F, cpu, mem| { unimplemented!() },
    |_60, cpu, mem| { unimplemented!() },
    |_61, cpu, mem| { unimplemented!() },
    |_62, cpu, mem| { unimplemented!() },
    |_63, cpu, mem| { unimplemented!() },
    |_64, cpu, mem| { unimplemented!() },
    |_65, cpu, mem| { unimplemented!() },
    |_66, cpu, mem| { unimplemented!() },
    |_67, cpu, mem| { unimplemented!() },
    |_68, cpu, mem| { unimplemented!() },
    |_69, cpu, mem| { unimplemented!() },
    |_6A, cpu, mem| { unimplemented!() },
    |_6B, cpu, mem| { unimplemented!() },
    |_6C, cpu, mem| { unimplemented!() },
    |_6D, cpu, mem| { unimplemented!() },
    |_6E, cpu, mem| { unimplemented!() },
    |_6F, cpu, mem| { unimplemented!() },
    |_70, cpu, mem| { unimplemented!() },
    |_71, cpu, mem| { unimplemented!() },
    |_72, cpu, mem| { unimplemented!() },
    |_73, cpu, mem| { unimplemented!() },
    |_74, cpu, mem| { unimplemented!() },
    |_75, cpu, mem| { unimplemented!() },
    |_76, cpu, mem| { unimplemented!() },
    |_77, cpu, mem| { unimplemented!() },
    |_78, cpu, mem| { unimplemented!() },
    |_79, cpu, mem| { unimplemented!() },
    |_7A, cpu, mem| { unimplemented!() },
    |_7B, cpu, mem| { unimplemented!() },
    |_7C, cpu, mem| { unimplemented!() },
    |_7D, cpu, mem| { unimplemented!() },
    |_7E, cpu, mem| { unimplemented!() },
    |_7F, cpu, mem| { unimplemented!() },
    |_80, cpu, mem| { unimplemented!() },
    |_81, cpu, mem| { unimplemented!() },
    |_82, cpu, mem| { unimplemented!() },
    |_83, cpu, mem| { unimplemented!() },
    |_84, cpu, mem| { unimplemented!() },
    |_85, cpu, mem| { unimplemented!() },
    |_86, cpu, mem| { unimplemented!() },
    |_87, cpu, mem| { unimplemented!() },
    |_88, cpu, mem| { unimplemented!() },
    |_89, cpu, mem| { unimplemented!() },
    |_8A, cpu, mem| { unimplemented!() },
    |_8B, cpu, mem| { unimplemented!() },
    |_8C, cpu, mem| { unimplemented!() },
    |_8D, cpu, mem| { unimplemented!() },
    |_8E, cpu, mem| { unimplemented!() },
    |_8F, cpu, mem| { unimplemented!() },
    |_90, cpu, mem| { unimplemented!() },
    |_91, cpu, mem| { unimplemented!() },
    |_92, cpu, mem| { unimplemented!() },
    |_93, cpu, mem| { unimplemented!() },
    |_94, cpu, mem| { unimplemented!() },
    |_95, cpu, mem| { unimplemented!() },
    |_96, cpu, mem| { unimplemented!() },
    |_97, cpu, mem| { unimplemented!() },
    |_98, cpu, mem| { unimplemented!() },
    |_99, cpu, mem| { unimplemented!() },
    |_9A, cpu, mem| { unimplemented!() },
    |_9B, cpu, mem| { unimplemented!() },
    |_9C, cpu, mem| { unimplemented!() },
    |_9D, cpu, mem| { unimplemented!() },
    |_9E, cpu, mem| { unimplemented!() },
    |_9F, cpu, mem| { unimplemented!() },
    |_A0, cpu, mem| { unimplemented!() },
    |_A1, cpu, mem| { unimplemented!() },
    |_A2, cpu, mem| { unimplemented!() },
    |_A3, cpu, mem| { unimplemented!() },
    |_A4, cpu, mem| { unimplemented!() },
    |_A5, cpu, mem| { unimplemented!() },
    |_A6, cpu, mem| { unimplemented!() },
    |_A7, cpu, mem| { unimplemented!() },
    |_A8, cpu, mem| { unimplemented!() },
    |_A9, cpu, mem| { unimplemented!() },
    |_AA, cpu, mem| { unimplemented!() },
    |_AB, cpu, mem| { unimplemented!() },
    |_AC, cpu, mem| { unimplemented!() },
    |_AD, cpu, mem| { unimplemented!() },
    |_AE, cpu, mem| { unimplemented!() },
    |_AF, cpu, mem| { unimplemented!() },
    |_B0, cpu, mem| { unimplemented!() },
    |_B1, cpu, mem| { unimplemented!() },
    |_B2, cpu, mem| { unimplemented!() },
    |_B3, cpu, mem| { unimplemented!() },
    |_B4, cpu, mem| { unimplemented!() },
    |_B5, cpu, mem| { unimplemented!() },
    |_B6, cpu, mem| { unimplemented!() },
    |_B7, cpu, mem| { unimplemented!() },
    |_B8, cpu, mem| { unimplemented!() },
    |_B9, cpu, mem| { unimplemented!() },
    |_BA, cpu, mem| { unimplemented!() },
    |_BB, cpu, mem| { unimplemented!() },
    |_BC, cpu, mem| { unimplemented!() },
    |_BD, cpu, mem| { unimplemented!() },
    |_BE, cpu, mem| { unimplemented!() },
    |_BF, cpu, mem| { unimplemented!() },
    |_C0, cpu, mem| { unimplemented!() },
    |_C1, cpu, mem| { unimplemented!() },
    |_C2, cpu, mem| { unimplemented!() },
    |_C3, cpu, mem| { unimplemented!() },
    |_C4, cpu, mem| { unimplemented!() },
    |_C5, cpu, mem| { unimplemented!() },
    |_C6, cpu, mem| { unimplemented!() },
    |_C7, cpu, mem| { unimplemented!() },
    |_C8, cpu, mem| { unimplemented!() },
    |_C9, cpu, mem| { unimplemented!() },
    |_CA, cpu, mem| { unimplemented!() },
    |_CB, cpu, mem| { unimplemented!() },
    |_CC, cpu, mem| { unimplemented!() },
    |_CD, cpu, mem| { unimplemented!() },
    |_CE, cpu, mem| { unimplemented!() },
    |_CF, cpu, mem| { unimplemented!() },
    |_D0, cpu, mem| { unimplemented!() },
    |_D1, cpu, mem| { unimplemented!() },
    |_D2, cpu, mem| { unimplemented!() },
    |_D3, cpu, mem| { unimplemented!() },
    |_D4, cpu, mem| { unimplemented!() },
    |_D5, cpu, mem| { unimplemented!() },
    |_D6, cpu, mem| { unimplemented!() },
    |_D7, cpu, mem| { unimplemented!() },
    |_D8, cpu, mem| { unimplemented!() },
    |_D9, cpu, mem| { unimplemented!() },
    |_DA, cpu, mem| { unimplemented!() },
    |_DB, cpu, mem| { unimplemented!() },
    |_DC, cpu, mem| { unimplemented!() },
    |_DD, cpu, mem| { unimplemented!() },
    |_DE, cpu, mem| { unimplemented!() },
    |_DF, cpu, mem| { unimplemented!() },
    |_E0, cpu, mem| { unimplemented!() },
    |_E1, cpu, mem| { unimplemented!() },
    |_E2, cpu, mem| { unimplemented!() },
    |_E3, cpu, mem| { unimplemented!() },
    |_E4, cpu, mem| { unimplemented!() },
    |_E5, cpu, mem| { unimplemented!() },
    |_E6, cpu, mem| { unimplemented!() },
    |_E7, cpu, mem| { unimplemented!() },
    |_E8, cpu, mem| { unimplemented!() },
    |_E9, cpu, mem| { unimplemented!() },
    |_EA, cpu, mem| { unimplemented!() },
    |_EB, cpu, mem| { unimplemented!() },
    |_EC, cpu, mem| { unimplemented!() },
    |_ED, cpu, mem| { unimplemented!() },
    |_EE, cpu, mem| { unimplemented!() },
    |_EF, cpu, mem| { unimplemented!() },
    |_F0, cpu, mem| { unimplemented!() },
    |_F1, cpu, mem| { unimplemented!() },
    |_F2, cpu, mem| { unimplemented!() },
    |_F3, cpu, mem| { unimplemented!() },
    |_F4, cpu, mem| { unimplemented!() },
    |_F5, cpu, mem| { unimplemented!() },
    |_F6, cpu, mem| { unimplemented!() },
    |_F7, cpu, mem| { unimplemented!() },
    |_F8, cpu, mem| { unimplemented!() },
    |_F9, cpu, mem| { unimplemented!() },
    |_FA, cpu, mem| { unimplemented!() },
    |_FB, cpu, mem| { unimplemented!() },
    |_FC, cpu, mem| { unimplemented!() },
    |_FD, cpu, mem| { unimplemented!() },
    |_FE, cpu, mem| { unimplemented!() },
];
