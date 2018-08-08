use zerodmg_utils::little_endian::{u16_to_u8s, u8_get_bit, u8s_to_u16};

use zerodmg_codes::instruction::{
    FlagCondition, Instruction, U16Register, U8Register, U8SecondaryRegister,
};

use super::memory::MemoryController;
use super::GameBoy;
use rand;

#[derive(Debug, Clone, Copy)]
pub struct CPUData {
    /// Clock ticks
    t: u64,
    /// A/Accumulator register
    a: u8,
    /// F/Flags register
    f: u8,
    /// BC register/B and C registers
    b: u8,
    c: u8,
    /// DE register/D and E registers
    d: u8,
    e: u8,
    /// HL register/H and L registers
    h: u8,
    l: u8,
    /// SP/Stack Pointer register
    sp: u16,
    /// PC/Program Counter register
    pc: u16,
    /// Interrupt Master Enable register
    ime: bool,
    /// Interrupt Enable register 0xFFFF
    ie: u8,
    /// Interrupt Flag/trigger register 0xFF0F
    ift: u8,
    /// Disable interrupts after next instruction
    di_pending: bool,
    /// Enable interrupt after next instruction
    ei_pending: bool,
}

pub struct InstructionExecution {
    pub t_0: u64,
    pub t_1: u64,
    pub instruction: Instruction,
    /// Formats some additional debug information about the execution.
    pub tracer: Option<Box<Fn() -> String>>,
    pub source: InstructionSource,
}

pub trait CPUController:
    GetSetRegisters<U8Register, u8>
    + GetSetRegisters<U16Register, u16>
    + GetSetRegisters<U8SecondaryRegister, u8>
{
    fn tick(&mut self) -> InstructionExecution;
    fn relative_jump(&mut self, n: i8);
    fn stack_push(&mut self, value: u16);
    fn stack_pop(&mut self) -> u16;
    fn af(&self) -> u16;
    fn set_af(&mut self, value: u16);
    fn c_flag(&self) -> bool;
    fn set_c_flag(&mut self, value: bool);
    fn h_flag(&self) -> bool;
    fn set_h_flag(&mut self, value: bool);
    fn n_flag(&self) -> bool;
    fn set_n_flag(&mut self, value: bool);
    fn z_flag(&self) -> bool;
    fn set_z_flag(&mut self, value: bool);
    fn set_znhc_flags(&mut self, z: bool, n: bool, h: bool, c: bool);
    fn iter_bytes_at_pc(&'gb mut self) -> PCMemoryIterator;
    fn instruction_from_pc(&mut self) -> Instruction;
    fn condition(&self, condition: FlagCondition) -> bool;
    fn pop_interrupt(&mut self) -> Option<InterruptType>;
    fn ie(&self) -> u8;
    fn set_ie(&mut self, value: u8);
    fn ift(&self) -> u8;
    fn set_ift(&mut self, value: u8);
}

impl CPUData {
    pub fn new() -> Self {
        Self {
            t: 0x0000000000000000,
            a: rand::random(),
            f: rand::random(),
            b: rand::random(),
            c: rand::random(),
            d: rand::random(),
            e: rand::random(),
            h: rand::random(),
            l: rand::random(),
            sp: 0x0000,
            pc: 0x0000,
            ime: true,
            ie: 0xFF,
            ift: 0x00,
            di_pending: false,
            ei_pending: false,
        }
    }
}

/// Iterates over bytes at PC, while incrementing it, in a borrowed [GameBoy].
pub struct PCMemoryIterator<'gb> {
    gb: &'gb mut GameBoy,
}

impl Iterator for PCMemoryIterator<'gb> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let pc_0 = self.gb.cpu.pc;
        let byte = self.gb.mem(pc_0);
        let pc_1 = pc_0.wrapping_add(0x001);
        self.gb.cpu.pc = pc_1;
        Some(byte)
    }
}

#[derive(Clone, Copy)]
pub enum InterruptType {
    VBlank,
    LcdStatus,
    TimerOverflow,
    SerialTransfer,
    ButtonAction,
}

impl std::fmt::Display for InterruptType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::InterruptType::*;
        write!(
            f,
            "{}",
            match self {
                VBlank => "VBLANK",
                LcdStatus => "LCDSTA",
                TimerOverflow => "TIMERO",
                SerialTransfer => "SERIAL",
                ButtonAction => "BUTTON",
            }
        )
    }
}

impl InterruptType {
    fn handler_address(self) -> u16 {
        use self::InterruptType::*;
        match self {
            VBlank => 0x40,
            LcdStatus => 0x48,
            TimerOverflow => 0x50,
            SerialTransfer => 0x58,
            ButtonAction => 0x60,
        }
    }
}

#[derive(Clone, Copy)]
pub enum InstructionSource {
    ProgramCounter(u16),
    Interrupt(InterruptType),
}

impl std::fmt::Display for InstructionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::InstructionSource::*;
        match self {
            ProgramCounter(address) => write!(f, "0x{:04X}", address),
            Interrupt(interrupt_type) => write!(f, "{}", interrupt_type),
        }
    }
}

impl CPUController for GameBoy {
    fn tick(&mut self) -> InstructionExecution {
        use zerodmg_codes::instruction::prelude::*;

        let has_interrupt = self.pop_interrupt();

        let source;
        let instruction;

        // if self.cpu.pc == 0x0007 {
        //     // temporarily prevent blanking of video memory
        //     instruction = Instruction::DEC_16(U16Register::HL);
        //     source = InstructionSource::ProgramCounter(self.cpu.pc);
        //     self.cpu.pc = 0x0008;
        // } else
        // if true {
        //     instruction = NOP;
        //     source = InstructionSource::ProgramCounter(0xF0BA);
        // } else
        if let Some(interrupt) = has_interrupt {
            // disable interrupts
            self.cpu.ime = false;
            source = InstructionSource::Interrupt(interrupt);
            instruction = Instruction::CALL(interrupt.handler_address());
        } else {
            source = InstructionSource::ProgramCounter(self.cpu.pc);
            instruction = self.instruction_from_pc();
        };

        // println!("   t = {:<10}  f_z = {}", self.cpu.t, self.z_flag());
        // println!("  HL = {:04X}  A = {:02X}  B = {:02X}  C = {:02X}  D = {:02X}, E =
        // {:02X}", self.get_register(HL), self.get_register(A), self.get_register(B),
        // self.get_register(C), self.get_register(D), self.get_register(E));
        // println!("{:6}:   {:<16}  ; {:<16}", source, format!("{}", instruction),
        // format!("{:?}", instruction));

        let t_0 = self.cpu.t;
        let cycles;
        let tracer: Option<Box<Fn() -> String>>;
        macro_rules! trace {
            ($($x:expr),*) => {
                tracer = Some(Box::new(move || { format!($($x),*) }))
            }
        }

        match instruction {
            // Control
            NOP => {
                cycles = 1;
                tracer = None;
            }
            HALT => unimplemented!("{}", instruction),
            STOP(_unused) => unimplemented!("{}", instruction),
            EI => unimplemented!("{}", instruction),
            DI => unimplemented!("{}", instruction),
            HCF(_variant) => unimplemented!("{}", instruction),
            // 8-Bit Arithmatic and Logic
            INC(target) => {
                let (old_value, extra_read_cycles) = self.read_register(target);
                let new_value = old_value.wrapping_add(1);
                let extra_write_cycles = self.set_register(target, new_value);
                self.set_z_flag(new_value == 0);
                self.set_n_flag(false);
                self.set_h_flag(u8_get_bit(new_value, 4));
                cycles = 1 + extra_read_cycles + extra_write_cycles;
                trace!(
                    "{}₀ = 0x{:02X}, {}₁ = 0x{:02X}",
                    target,
                    old_value,
                    target,
                    new_value
                );
            }
            DEC(target) => {
                let (old_value, extra_read_cycles) = self.read_register(target);
                let new_value = old_value.wrapping_sub(1);
                let extra_write_cycles = self.set_register(target, new_value);
                self.set_z_flag(new_value == 0);
                self.set_n_flag(true);
                self.set_h_flag(u8_get_bit(new_value, 4));
                cycles = 1 + extra_read_cycles + extra_write_cycles;
                trace!(
                    "{}₀ = 0x{:02X}, {}₁ = 0x{:02X}",
                    target,
                    old_value,
                    target,
                    new_value
                );
            }
            ADD(source) => {
                let a_0 = self.cpu.a;
                let (value, extra_read_cycles) = self.read_register(source);
                let a_1 = a_0.wrapping_add(value);
                self.cpu.a = a_1;
                self.set_znhc_flags(a_1 == 0, false, u8_get_bit(a_1, 4), a_1 < a_0);
                cycles = 1 + extra_read_cycles;
                trace!(
                    "A₀ = 0x{:02X}, {} = 0x{:02X}, A₁ = 0x{:02X}",
                    a_0,
                    source,
                    value,
                    a_1
                );
            }
            ADC(_source) => unimplemented!("{}", instruction),
            SUB(source) => {
                let (value, extra_read_cycles) = self.read_register(source);
                let a_0 = self.cpu.a;
                let a_1 = a_0.wrapping_sub(value);
                self.cpu.a = a_1;
                self.set_znhc_flags(a_1 == 0, false, u8_get_bit(a_1, 4), a_1 > a_0);
                cycles = 1 + extra_read_cycles;
                trace!(
                    "A₀ = 0x{:02X}, {} = 0x{:02X}, A₁ = 0x{:02X}",
                    a_0,
                    source,
                    value,
                    a_1
                );
            }
            SBC(_source) => unimplemented!("{}", instruction),
            AND(source) => {
                let (value, extra_read_cycles) = self.read_register(source);
                let a_0 = self.cpu.a;
                let a_1 = a_0 & value;
                self.cpu.a = a_1;
                self.set_znhc_flags(a_1 == 0, true, true, false);
                cycles = 1 + extra_read_cycles;
                trace!(
                    "A₀ = 0x{:02X}, {} = 0x{:02X}, A₁ = 0x{:02X}",
                    a_0,
                    source,
                    value,
                    a_1
                );
            }
            XOR(source) => {
                let a_0 = self.cpu.a;
                let (value, extra_read_cycles) = self.read_register(source);
                let a_1 = a_0 ^ value;
                self.cpu.a = a_1;
                self.set_znhc_flags(a_1 == 0, false, false, false);
                cycles = 1 + extra_read_cycles;
                trace!(
                    "A₀ = 0x{:02X}, {} = 0x{:02X}, A₁ = 0x{:02X}",
                    a_0,
                    source,
                    value,
                    a_1
                );
            }
            OR(source) => {
                let (value, extra_read_cycles) = self.read_register(source);
                let a_0 = self.cpu.a;
                let a_1 = a_0 | value;
                self.cpu.a = a_1;
                self.set_znhc_flags(a_1 == 0, false, false, false);
                cycles = 1 + extra_read_cycles;
                trace!(
                    "A₀ = 0x{:02X}, {} = 0x{:02X}, A₁ = 0x{:02X}",
                    a_0,
                    source,
                    value,
                    a_1
                );
            }
            CP(source) => {
                let (value, extra_read_cycles) = self.read_register(source);
                let a = self.cpu.a;
                let delta = a.wrapping_sub(value);
                self.set_znhc_flags(delta == 0, false, u8_get_bit(delta, 4), delta > a);
                cycles = 1 + extra_read_cycles;
                trace!("A = 0x{:02X}, {} = 0x{:02X}", a, source, value);
            }
            ADD_IMMEDIATE(_value) => unimplemented!("{}", instruction),
            ADC_IMMEDIATE(_value) => unimplemented!("{}", instruction),
            SUB_IMMEDIATE(_value) => unimplemented!("{}", instruction),
            SBC_IMMEDIATE(_value) => unimplemented!("{}", instruction),
            AND_IMMEDIATE(_value) => unimplemented!("{}", instruction),
            XOR_IMMEDIATE(_value) => unimplemented!("{}", instruction),
            OR_IMMEDIATE(_value) => unimplemented!("{}", instruction),
            CP_IMMEDIATE(value) => {
                let a = self.cpu.a;
                let delta = a.wrapping_sub(value);
                self.set_znhc_flags(delta == 0, true, u8_get_bit(delta, 4), a < value);
                let z_flag = self.z_flag();
                let c_flag = self.c_flag();
                cycles = 2;
                trace!("A = 0x{:02X}, F_Z = {}, F_C = {}", a, z_flag, c_flag);
            }
            CPL => unimplemented!("{}", instruction),
            CCF => unimplemented!("{}", instruction),
            SCF => unimplemented!("{}", instruction),
            DAA => unimplemented!("{}", instruction),
            // 16-Bit Arithmatic and Logic
            INC_16(target) => {
                let old_value = self.get_register(target);
                let new_value = old_value.wrapping_add(1);
                self.set_register(target, new_value);
                cycles = 2;
                trace!(
                    "{:?}₀ = 0x{:02X}, {:?}₁ = 0x{:02X}",
                    target,
                    old_value,
                    target,
                    new_value
                );
            }
            DEC_16(target) => {
                let old_value = self.get_register(target);
                let new_value = old_value.wrapping_sub(1);
                self.set_register(target, new_value);
                cycles = 2;
                trace!(
                    "{:?}₀ = 0x{:02X}, {:?}₁ = 0x{:02X}",
                    target,
                    old_value,
                    target,
                    new_value
                );
            }
            ADD_TO_HL(_) => unimplemented!("{}", instruction),
            ADD_SP(_) => unimplemented!("{}", instruction),
            // 8-Bit Bitwise Operations
            RL(register) => {
                let f_c_0 = self.c_flag();
                let value_0 = self.get_register(register);
                let value_1 = (value_0 << 1) + if f_c_0 { 1 } else { 0 };
                let f_c_1 = value_0 & 0b1000_0000 > 0;
                self.set_register(register, value_1);
                self.set_znhc_flags(value_1 == 0, false, false, f_c_1);
                cycles = 2;
                trace!(
                    "Fc₀ = {}, {}₀ = 0x{:02X}, Fc₁ = {}, {}₁ = 0x{:02X}",
                    f_c_0,
                    register,
                    value_0,
                    f_c_1,
                    register,
                    value_1
                );
            }
            RLA => {
                let f_c_0 = self.c_flag();
                let a_0 = self.cpu.a;
                let a_1 = (a_0 << 1) + if f_c_0 { 1 } else { 0 };
                let f_c_1 = a_0 & 0b1000_0000 > 0;
                self.cpu.a = a_1;
                // We're setting the wrong flags!
                self.set_znhc_flags(a_1 == 0, false, false, f_c_1);
                cycles = 2;
                trace!(
                    "Fc₀ = {}, A₀ = 0x{:02X}, Fc₁ = {}, A₁ = 0x{:02X}",
                    f_c_0,
                    a_0,
                    f_c_1,
                    a_1
                );
            }
            RLC(_register) => unimplemented!("{}", instruction),
            RLCA => unimplemented!("{}", instruction),
            RR(_register) => unimplemented!("{}", instruction),
            RRA => unimplemented!("{}", instruction),
            RRC(_register) => unimplemented!("{}", instruction),
            RRCA => unimplemented!("{}", instruction),
            SRL(_register) => unimplemented!("{}", instruction),
            SRA(_register) => unimplemented!("{}", instruction),
            SLA(_register) => unimplemented!("{}", instruction),
            SWAP(_register) => unimplemented!("{}", instruction),
            BIT(bit, register) => {
                let value = self.get_register(register);
                let result = !u8_get_bit(value, bit.index());
                self.set_z_flag(result);
                self.set_n_flag(false);
                self.set_h_flag(true);
                cycles = 2;
                trace!("Z₁ = {}", result);
            }
            SET(_bit, _register) => unimplemented!("{}", instruction),
            RES(_bit, _register) => unimplemented!("{}", instruction),
            // 8-Bit Loads
            LD_8_INTERNAL(dest, source) => {
                let dest_value_0 = self.get_register(dest);
                let (source_value, extra_read_cycles) = self.read_register(source);
                let extra_write_cycles = self.set_register(dest, source_value);
                cycles = 1 + extra_read_cycles + extra_write_cycles;
                trace!(
                    "{} = {}, {}₀ = {}",
                    source,
                    source_value,
                    dest,
                    dest_value_0
                );
            }
            LD_8_IMMEDIATE(dest, value) => {
                let dest_value_0 = self.get_register(dest);
                let extra_write_cycles = self.set_register(dest, value);
                cycles = 2 + extra_write_cycles;
                trace!("{}₀ = 0x{:02X}", dest, dest_value_0);
            }
            LD_8_TO_SECONDARY(dest) => {
                let dest_value_0 = self.get_register(dest);
                let a = self.cpu.a;
                self.set_register(dest, a);
                cycles = 2;
                trace!("{}₀ = 0x{:02X}, A = 0x{:02X}", dest, dest_value_0, a)
            }
            LD_8_FROM_SECONDARY(source) => {
                let a_0 = self.cpu.a;
                let a_1 = self.get_register(source);
                self.cpu.a = a_1;
                cycles = 2;
                trace!("A₀ = 0x{:02X}, {} = 0x{:02X}", a_0, source, a_1)
            }
            LD_8_TO_FF_IMMEDIATE(offset) => {
                let a = self.cpu.a;
                let address = 0xFF00 + u16::from(offset);
                self.set_mem(address, a);
                cycles = 4;
                trace!("A = 0x{:02X}", a);
            }
            LD_8_FROM_FF_IMMEDIATE(offset) => {
                let a_0 = self.cpu.a;
                let a_1 = self.mem(0xFF00 + u16::from(offset));
                self.cpu.a = a_1;
                cycles = 3;
                trace!("A₀ = 0x{:02X}, A₁ = 0x{:02X}", a_0, a_1);
            }
            LD_8_TO_FF_C => {
                let a = self.cpu.a;
                let c = self.cpu.c;
                let address = 0xFF00 + u16::from(c);
                let old_value = self.mem(address);
                self.set_mem(address, a);
                cycles = 2;
                trace!(
                    "C = 0x{:02X}, A = 0x{:02X}, (0xFFFF + C)₀ = 0x{:02X}",
                    c,
                    a,
                    old_value
                );
            }
            LD_8_FROM_FF_C => unimplemented!("{}", instruction),
            LD_8_TO_MEMORY_IMMEDIATE(address) => {
                let a = self.cpu.a;
                let old_value = self.mem(address);
                self.set_mem(address, a);
                cycles = 4;
                trace!(
                    "A = {:02X}, (0x{:04X})₀ = 0x{:02X}",
                    address,
                    a,
                    old_value
                );
            }
            LD_8_FROM_MEMORY_IMMEDIATE(_address) => unimplemented!("{}", instruction),
            // 16-Bit Loads
            LD_16_IMMEDIATE(dest, value) => {
                let old_value = self.get_register(dest);
                self.set_register(dest, value);
                cycles = 3;
                trace!("{:?}₀ = 0x{:04X}", dest, old_value);
            }
            LD_HL_FROM_SP => unimplemented!("{}", instruction),
            LD_HL_FROM_SP_PLUS(_value) => unimplemented!("{}", instruction),
            LD_SP_TO_IMMEDIATE_ADDRESS(_address) => unimplemented!("{}", instruction),
            PUSH(register) => {
                let value = self.get_register(register);
                self.stack_push(value);
                let sp_1 = self.cpu.sp;
                cycles = 4;
                trace!("{:?} = 0x{:02X}, SP₁ = 0x{:04X}", register, value, sp_1);
            }
            POP(register) => {
                let value = self.stack_pop();
                let sp_1 = self.cpu.sp;
                self.set_register(register, value);
                cycles = 3;
                trace!(
                    "{:?}₁ = 0x{:02X}, SP₁ = 0x{:04X}",
                    register,
                    value,
                    sp_1
                );
            }
            // Jumps and Calls
            JP_IF(condition, address) => {
                if self.condition(condition) {
                    self.cpu.pc = address;
                    trace!("jumped - condition true");
                    cycles = 4;
                } else {
                    trace!("skipped - condition false");
                    cycles = 3;
                }
            }
            JP(address) => {
                self.cpu.pc = address;
                cycles = 4;
                tracer = None;
            }
            JP_HL => unimplemented!("{}", instruction),
            JR_IF(condition, offset) => {
                if self.condition(condition) {
                    self.relative_jump(offset);
                    trace!("jumped - condition true");
                    cycles = 3;
                } else {
                    trace!("skipped - condition false");
                    cycles = 2;
                }
            }
            JR(offset) => {
                self.relative_jump(offset);
                cycles = 3;
                tracer = None;
            }
            CALL_IF(condition, address) => {
                if self.condition(condition) {
                    let pc_0 = self.cpu.pc;
                    self.stack_push(pc_0);
                    self.cpu.pc = address;
                    let sp_1 = self.cpu.sp;
                    cycles = 6;
                    trace!("SP₁ = {:04X}", sp_1);
                } else {
                    cycles = 3;
                    trace!("skipped - condition false");
                }
            }
            CALL(address) => {
                let pc_0 = self.cpu.pc;
                self.stack_push(pc_0);
                self.cpu.pc = address;
                let sp_1 = self.cpu.sp;
                cycles = 6;
                trace!("SP₁ = {:04X}", sp_1);
            }
            RST(address) => {
                let pc_0 = self.cpu.pc;
                let pc_1 = address.address().into();
                self.stack_push(pc_0);
                self.cpu.pc = pc_1;
                cycles = 4;
                tracer = None;
            }
            RET => {
                let pc_1 = self.stack_pop();
                let sp_1 = self.cpu.sp;
                self.cpu.pc = pc_1;
                cycles = 2;
                trace!("SP₁ = {:04X}", sp_1);
            }
            RET_IF(_condition) => unimplemented!("{}", instruction),
            RETI => unimplemented!("{}", instruction),
        }

        let t_1 = t_0 + cycles;
        self.cpu.t = t_1;

        InstructionExecution {
            instruction,
            t_0,
            t_1,
            source,
            tracer,
        }
    }

    /// Returns the next InterruptType currently set in the interrupt register,
    /// and unsets it there.
    fn pop_interrupt(&mut self) -> Option<InterruptType> {
        let enabled_and_triggered = self.cpu.ie & self.cpu.ift;
        if enabled_and_triggered & 0b00001 != 0 {
            self.cpu.ift &= !0b00001;
            Some(InterruptType::VBlank)
        } else if enabled_and_triggered & 0b00010 != 0 {
            self.cpu.ift &= !0b00010;
            Some(InterruptType::LcdStatus)
        } else if enabled_and_triggered & 0b00100 != 0 {
            self.cpu.ift &= !0b00100;
            Some(InterruptType::TimerOverflow)
        } else if enabled_and_triggered & 0b01000 != 0 {
            self.cpu.ift &= !0b01000;
            Some(InterruptType::SerialTransfer)
        } else if enabled_and_triggered & 0b10000 != 0 {
            self.cpu.ift &= !0b10000;
            Some(InterruptType::ButtonAction)
        } else {
            None
        }
    }

    fn ie(&self) -> u8 {
        return self.cpu.ie;
    }

    fn set_ie(&mut self, ie: u8) {
        self.cpu.ie = ie;
    }

    fn ift(&self) -> u8 {
        return self.cpu.ift;
    }

    fn set_ift(&mut self, ift: u8) {
        self.cpu.ift = ift;
    }

    // Returns the instruction in memory at PC, and advances PC past it.
    fn instruction_from_pc(&mut self) -> Instruction {
        Instruction::from_byte_iter(&mut self.iter_bytes_at_pc()).unwrap()
    }

    // Returns an Iterator that yields bytes from memory at PC++.
    fn iter_bytes_at_pc<'gb>(&'gb mut self) -> PCMemoryIterator<'gb> {
        PCMemoryIterator { gb: self }
    }

    fn relative_jump(&mut self, n: i8) {
        self.cpu.pc = (i32::from(self.cpu.pc) + i32::from(n)) as u16;
    }

    fn stack_push(&mut self, value: u16) {
        let sp0 = self.cpu.sp;
        let sp1 = sp0 - 2;
        let (value_low, value_high) = u16_to_u8s(value);
        self.set_mem(sp1 + 1, value_low);
        self.set_mem(sp1 + 0, value_high);
        self.cpu.sp = sp1;
    }

    fn stack_pop(&mut self) -> u16 {
        let sp0 = self.cpu.sp;
        let sp1 = sp0 + 2;
        let value_low = self.mem(sp0 + 1);
        let value_high = self.mem(sp0 + 0);
        let value = u8s_to_u16(value_low, value_high);
        self.cpu.sp = sp1;
        value
    }

    fn af(&self) -> u16 {
        u8s_to_u16(self.cpu.f, self.cpu.a)
    }

    fn set_af(&mut self, value: u16) {
        let (f, a) = u16_to_u8s(value);
        self.cpu.a = a;
        self.cpu.f = f;
    }

    fn c_flag(&self) -> bool {
        (self.cpu.f & 0x10) == 0x10
    }

    fn set_c_flag(&mut self, value: bool) {
        if value {
            self.cpu.f |= 0x10;
        } else {
            self.cpu.f &= !0x10;
        }
    }

    fn h_flag(&self) -> bool {
        (self.cpu.f & 0x20) == 0x20
    }

    fn set_h_flag(&mut self, value: bool) {
        if value {
            self.cpu.f |= 0x20;
        } else {
            self.cpu.f &= !0x20;
        }
    }

    fn n_flag(&self) -> bool {
        (self.cpu.f & 0x40) == 0x40
    }

    fn set_n_flag(&mut self, value: bool) {
        if value {
            self.cpu.f |= 0x40;
        } else {
            self.cpu.f &= !0x40;
        }
    }

    fn z_flag(&self) -> bool {
        (self.cpu.f & 0x80) == 0x80
    }

    fn set_z_flag(&mut self, value: bool) {
        if value {
            self.cpu.f |= 0x80;
        } else {
            self.cpu.f &= !0x80;
        }
    }

    fn set_znhc_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.cpu.f = 0x00
            | if z { 0x80 } else { 0x00 }
            | if n { 0x40 } else { 0x00 }
            | if h { 0x20 } else { 0x00 }
            | if c { 0x10 } else { 0x00 };
    }

    fn condition(&self, condition: FlagCondition) -> bool {
        use zerodmg_codes::instruction::prelude::*;
        match condition {
            if_Z => self.z_flag(),
            if_NZ => !self.z_flag(),
            if_C => self.c_flag(),
            if_NC => !self.c_flag(),
        }
    }
}

pub trait GetSetRegisters<Register, RegisterValue> {
    /// Reads the value in the given register.
    ///
    /// If this is a pseudo-register like (HL+), this may have side effects.
    fn read_register(&mut self, register: Register) -> (RegisterValue, u64) {
        (self.get_register(register), 0)
    }

    /// Reads the value in the given register, suppressing any side effects.
    fn get_register(&self, register: Register) -> RegisterValue;

    /// Updates the value in the given register.
    fn set_register(&mut self, register: Register, value: RegisterValue) -> u64;
}

impl GetSetRegisters<U8Register, u8> for GameBoy {
    fn read_register(&mut self, register: U8Register) -> (u8, u64) {
        use zerodmg_codes::instruction::prelude::*;
        (
            self.get_register(register),
            match register {
                AT_HL => 1,
                _ => 0,
            },
        )
    }

    fn get_register(&self, register: U8Register) -> u8 {
        use zerodmg_codes::instruction::prelude::*;
        match register {
            B => self.cpu.b,
            C => self.cpu.c,
            D => self.cpu.d,
            E => self.cpu.e,
            H => self.cpu.h,
            L => self.cpu.l,
            AT_HL => {
                let hl = self.get_register(HL);
                self.mem(hl)
            }
            A => self.cpu.a,
        }
    }

    fn set_register(&mut self, register: U8Register, value: u8) -> u64 {
        use zerodmg_codes::instruction::prelude::*;
        let mut extra_cycles = 0;
        match register {
            B => self.cpu.b = value,
            C => self.cpu.c = value,
            D => self.cpu.d = value,
            E => self.cpu.e = value,
            H => self.cpu.h = value,
            L => self.cpu.l = value,
            AT_HL => {
                extra_cycles = 1;
                let hl = self.get_register(HL);
                self.set_mem(hl, value);
            }
            A => {
                self.cpu.a = value;
            }
        }
        extra_cycles
    }
}

impl GetSetRegisters<U16Register, u16> for GameBoy {
    fn get_register(&self, register: U16Register) -> u16 {
        use zerodmg_codes::instruction::prelude::*;
        match register {
            BC => u8s_to_u16(self.cpu.c, self.cpu.b),
            DE => u8s_to_u16(self.cpu.e, self.cpu.d),
            HL => u8s_to_u16(self.cpu.l, self.cpu.h),
            SP => self.cpu.sp,
        }
    }

    fn set_register(&mut self, register: U16Register, value: u16) -> u64 {
        use zerodmg_codes::instruction::prelude::*;
        let (low, high) = u16_to_u8s(value);
        match register {
            BC => {
                self.cpu.b = high;
                self.cpu.c = low;
            }
            DE => {
                self.cpu.d = high;
                self.cpu.e = low;
            }
            HL => {
                self.cpu.h = high;
                self.cpu.l = low;
            }
            SP => {
                self.cpu.sp = value;
            }
        }
        0
    }
}

impl GetSetRegisters<U8SecondaryRegister, u8> for GameBoy {
    fn read_register(&mut self, register: U8SecondaryRegister) -> (u8, u64) {
        use zerodmg_codes::instruction::prelude::*;
        (
            match register {
                AT_HL_Plus => {
                    let hl_0 = self.get_register(HL);
                    let hl_1 = hl_0.wrapping_add(0x0001);
                    self.set_register(HL, hl_1);
                    self.mem(hl_0)
                }
                AT_HL_Minus => {
                    let hl_0 = self.get_register(HL);
                    let hl_1 = hl_0.wrapping_sub(0x0001);
                    self.set_register(HL, hl_1);
                    self.mem(hl_0)
                }
                _ => self.get_register(register),
            },
            0,
        )
    }

    fn get_register(&self, register: U8SecondaryRegister) -> u8 {
        use zerodmg_codes::instruction::prelude::*;
        let address = match register {
            AT_BC => self.get_register(BC),
            AT_DE => self.get_register(DE),
            AT_HL_Plus | AT_HL_Minus => self.get_register(HL),
        };
        self.mem(address)
    }

    fn set_register(&mut self, register: U8SecondaryRegister, value: u8) -> u64 {
        use zerodmg_codes::instruction::prelude::*;
        match register {
            AT_BC => {
                let bc = self.get_register(BC);
                self.set_mem(bc, value);
            }
            AT_DE => {
                let de = self.get_register(DE);
                self.set_mem(de, value);
            }
            AT_HL_Plus => {
                let hl_0 = self.get_register(HL);
                let hl_1 = hl_0.wrapping_add(0x0001);
                self.set_mem(hl_0, value);
                self.set_register(HL, hl_1);
            }
            AT_HL_Minus => {
                let hl_0 = self.get_register(HL);
                let hl_1 = hl_0.wrapping_sub(0x0001);
                self.set_mem(hl_0, value);
                self.set_register(HL, hl_1);
            }
        }
        0
    }
}
