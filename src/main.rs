fn main() {
    let mut gameboy = GameBoy::new();
    gameboy.run();
}

struct GameBoy {
    // time/ticks since start
    t: u64,
    // instruction pointer/index
    i: u16,
    main_ram: [u8; 8192],
    video_ram: [u8; 8192],
    high_ram: [u8; 127],
    main_registers: [u8; 12],
    boot_rom: Vec<u8>,
    game_rom: Vec<u8>,
    debug_current_op_addr: u16,
    debug_current_code: Vec<u8>,
}

impl GameBoy {
    fn new() -> GameBoy {
        GameBoy {
            t: 0,
            i: 0,
            main_ram: [0u8; 8192],
            video_ram: [0u8; 8192],
            high_ram: [0u8; 127],
            main_registers: [0u8; 12],
            boot_rom: load_boot_rom(),
            game_rom: load_game_rom("Pokemon Red (US)[:256]"),
            debug_current_op_addr: 0,
            debug_current_code: vec![],
        }
    }

    fn read_instruction(&mut self) -> u8 {
        self.debug_current_code.clear();
        self.debug_current_op_addr = self.i;
        self.read_immediate_u8()
    }

    fn read_immediate_u8(&mut self) -> u8 {
        let value = self.get_memory(self.i);
        self.debug_current_code.push(value);
        self.i += 1;
        value
    }

    fn relative_jump(&mut self, n: i32) {
        self.i = ((self.i as i32) + n) as u16;
    }

    fn print_current_code(&self, asm: String, info: String) {
        print!("{:32}", asm);
        print!(" ; ${:04x}", self.debug_current_op_addr);
        let code = self.debug_current_code.clone().into_iter().map(|c| { format!("{:02x}", c) }).collect::<Vec<String>>().join("");
        print!(" ; ${:8}", code);
        print!(" ; {}", info);
        println!();
    }

    // Main Loop

    fn run(&mut self) {
        println!("ASM:                               ADDR:   CODES:      FLAGS:");
        println!("----                               -----   ------      ------");

        while true {
            let opcode = self.read_instruction();

            match opcode {
                // 8-bit loads
                // LD nn, n
                // Put value n into nn.
                0x3E => {
                    let n = self.read_immediate_u8();
                    self.print_current_code(
                        format!("LD A, ${:02x}", n),
                        format!("A₀ = ${:02x}, A₁ = ${:02x}", self.accumulator(), n));
                    self.set_accumulator(n);
                }
                0x06 => {
                    let n = self.read_immediate_u8();
                    self.print_current_code(
                        format!("LD B, ${:02x}", n),
                        format!("B₀ = ${:02x}, B₁ = ${:02x}", self.b(), n));
                    self.set_b(n);
                }
                0x0E => {
                    let n = self.read_immediate_u8();
                    self.print_current_code(
                        format!("LD C, ${:02x}", n),
                        format!("C₀ = ${:02x}, C₁ = ${:02x}", self.c(), n));
                    self.set_c(n);
                }
                0x16 => {
                    let n = self.read_immediate_u8();
                    self.print_current_code(
                        format!("LD D, ${:02x}", n),
                        format!("D₀ = ${:02x}, D₁ = ${:02x}", self.d(), n));
                    self.set_d(n);
                }
                0x1E => {
                    let n = self.read_immediate_u8();
                    self.print_current_code(
                        format!("LD E, ${:02x}", n),
                        format!("E₀ = ${:02x}, E₁ = ${:02x}", self.e(), n));
                    self.set_e(n);
                }

                // Jumps
                // JR n
                // Unconditional relative jump.
                0x18 => {
                    let delta = self.read_immediate_u8() as i8;
                    self.print_current_code(format!("JR {})", delta), "".to_string());
                    self.relative_jump(delta as i32);
                }
                // JR cc, n
                // Conditional relative jump.
                0x20 => {
                    let delta = self.read_immediate_u8() as i8;
                    self.print_current_code(
                        format!("JR NZ, {}", delta),
                        format!("Z = {}", self.z_flag()));
                    if !self.z_flag() {
                        self.relative_jump(delta as i32);
                    }
                }
                0x28 => {
                    let delta = self.read_immediate_u8() as i8;
                    self.print_current_code(
                        format!("JR Z, {}", delta),
                        format!("Z = {}", self.z_flag()));
                    if self.z_flag() {
                        self.relative_jump(delta as i32);
                    }
                }
                0x30 => {
                    let delta = self.read_immediate_u8() as i8;
                    self.print_current_code(
                        format!("JR NC, {}", delta),
                        format!("C = {}", self.c_flag()));
                    if !self.c_flag() {
                        self.relative_jump(delta as i32);
                    }
                }
                0x38 => {
                    let delta = self.read_immediate_u8() as i8;
                    self.print_current_code(
                        format!("JR C, {}", delta),
                        format!("C = {}", self.c_flag()));
                    if self.c_flag() {
                        self.relative_jump(delta as i32);
                    }
                }

                0x21 => {
                    // LOAD HL, $1, $2
                    let h = self.read_immediate_u8();
                    let l = self.read_immediate_u8();
                    self.print_current_code(format!("LOAD HL, ${:02x}, ${:02x}", h, l), "".to_string());
                    self.set_h_l(h, l);
                }

                0x31 => {
                    // LOAD SP, $1, $2
                    let s = self.read_immediate_u8();
                    let p = self.read_immediate_u8();
                    self.print_current_code(format!("LOAD SP ${:02x}, ${:02x}", s, p), "".to_string());
                    self.set_s_p(s, p);
                }

                0x32 => {
                    // Put A into memory address HL.
                    self.print_current_code(
                        "LD (HL-), A".to_string(),
                        format!("HL₀ = ${:04x}, A = ${:02x}", self.hl(), self.accumulator()));
                    let mut hl = self.hl();
                    let accumulator = self.accumulator();
                    self.set_memory(hl, accumulator);
                    //  Decrement HL.
                    hl -= 1;
                    self.set_hl(hl);
                }

                0xE2 => {
                    // Put A into memory address 0xFF00 + C.
                    self.print_current_code(
                        "LD ($FF00+C), A ".to_string(),
                        format!("A = ${:02x}, C = ${:02x}", self.accumulator(), self.c()));
                    let accumulator = self.accumulator();
                    let address = 0xFF00 + (self.c() as u16);
                    self.set_memory(address, accumulator);
                }

                0xAF => {
                    self.print_current_code(
                        "XOR A A".to_string(),
                        format!("A₀ = ${:02x}, A₁ = $00", self.accumulator()).to_string());
                    self.set_accumulator(0);
                }

                // 8-Bit Arithmatic
                // Increment the value in register n.
                

                0xCB => {
                    // 2-byte opcode

                    let opcode_2 = self.read_immediate_u8();

                    match opcode_2 {
                        0x7C => {
                            let result = !u8_get_bit(self.h(), 7);
                            self.print_current_code(
                                "BIT 7, H".to_string(),
                                format!("Z₁ = {}", result));
                            self.set_z_flag(result);
                            self.set_n_flag(false);
                            self.set_h_flag(true);
                        }

                        _ => {
                            panic!("unsupported opcode: ${:02x}{:02x}", opcode, opcode_2);
                        }
                    }
                }

                _ => {
                    panic!("unsupported opcode: ${:02x}", opcode);
                }
            }

            self.t += 1;
        }
    }

    // Register Access

    fn accumulator(&self) -> u8 {
        return self.main_registers[0];
    }

    fn set_accumulator(&mut self, value: u8) {
        self.main_registers[0] = value;
    }

    fn flags(&self) -> u8 {
        return self.main_registers[1];
    }

    fn set_flags(&mut self, value: u8) {
        self.main_registers[1] = value;
    }

    fn b(&self) -> u8 {
        return self.main_registers[2];
    }

    fn set_b(&mut self, value: u8) {
        self.main_registers[2] = value;
    }

    fn c(&self) -> u8 {
        return self.main_registers[3];
    }

    fn set_c(&mut self, value: u8) {
        self.main_registers[3] = value;
    }

    fn d(&self) -> u8 {
        return self.main_registers[4];
    }

    fn set_d(&mut self, value: u8) {
        self.main_registers[4] = value;
    }

    fn e(&self) -> u8 {
        return self.main_registers[5];
    }

    fn set_e(&mut self, value: u8) {
        self.main_registers[5] = value;
    }

    fn h(&self) -> u8 {
        // XXX: this has been swapped with l as a test, clean-up required
        return self.main_registers[7];
    }

    fn set_h(&mut self, value: u8) {
        // XXX: this has been swapped with l as a test, clean-up required
        self.main_registers[7] = value;
    }

    fn l(&self) -> u8 {
        // XXX: this has been swapped with h as a test, clean-up required
        return self.main_registers[6];
    }

    fn set_l(&mut self, value: u8) {
        // XXX: this has been swapped with h as a test, clean-up required
        self.main_registers[6] = value;
    }

    fn hl(&self) -> u16 {
        return u8s_to_u16(self.main_registers[6], self.main_registers[7]);
    }

    fn set_hl(&mut self, value:u16) {
        let (h, l) = u16_to_u8s(value);
        self.main_registers[6] = h;
        self.main_registers[7] = l;
    }

    fn set_h_l(&mut self, h: u8, l: u8) {
        self.main_registers[6] = h;
        self.main_registers[7] = l;
    }

    fn sp(&self) -> u16 {
        return u8s_to_u16(self.main_registers[8], self.main_registers[9]);
    }

    fn set_sp(&mut self, value: u16) {
        let (s, p) = u16_to_u8s(value);
        self.main_registers[8] = s;
        self.main_registers[9] = p;
    }

    fn set_s_p(&mut self, s: u8, p: u8) {
        self.main_registers[8] = s;
        self.main_registers[9] = p;
    }

    fn pc(&self) -> u16 {
        return u8s_to_u16(self.main_registers[10], self.main_registers[11]);
    }

    fn set_pc(&mut self, value: u16) {
        let (p, c) = u16_to_u8s(value);
        self.main_registers[10] = p;
        self.main_registers[11] = c;
    }

    fn set_p_c(&mut self, p: u8, c: u8) {
        self.main_registers[10] = p;
        self.main_registers[11] = c;
    }

    fn z_flag(&self) -> bool {
        u8_get_bit(self.flags(), 1)
    }

    fn set_z_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 1, value);
        self.set_flags(flags);
    }

    fn n_flag(&self) -> bool {
        u8_get_bit(self.flags(), 2)
    }

    fn set_n_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 2, value);
        self.set_flags(flags);
    }

    fn h_flag(&self) -> bool {
        u8_get_bit(self.flags(), 3)
    }

    fn set_h_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 3, value);
        self.set_flags(flags);
    }

    fn c_flag(&self) -> bool {
        u8_get_bit(self.flags(), 4)
    }

    fn set_c_flag(&mut self, value: bool) {
        let mut flags = self.flags();
        u8_set_bit(&mut flags, 4, value);
        self.set_flags(flags);
    }

    // Memory Access

    fn get_memory(&self, address: u16) -> u8 {
        if address <= 0x00FF {
            return self.boot_rom[address as usize];
        }
        
        else if 0x8000 <= address && address <= 0x9FFF {
            let i: usize = (address - 0x8000) as usize;
            return self.video_ram[i];
        }
        
        else if 0xFF80 <= address && address <= 0xFFFE {
            let i: usize = (address - 0xFF80) as usize;
            return self.high_ram[i];
        }
        
        else {
            panic!("I don't know how to get memory address ${:04x}.", address);
        }
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        if 0x8000 <= address && address <= 0x9FFF {
            let i: usize = (address - 0x8000) as usize;
            println!("  ; video_ram[${:04x}] = ${:02x}", i, value);
            self.video_ram[i] = value;
        }
        
        else if 0xFF80 <= address && address <= 0xFFFE {
            let i: usize = (address - 0xFF80) as usize;
            println!("  ; high_ram[${:04x}] = ${:02x}", i, value);
            self.high_ram[i] = value;
        }

        else if 0xFF10 <= address && address <= 0xFF26 {
            println!("  ; skipping write to sound control memory -- not implemented");
        }
        
        else {
            panic!("I don't know how to set memory address ${:04x}.", address);
        }
    }
}

fn u8s_to_u16(accumulator: u8, b: u8) -> u16 {
    return accumulator as u16 + ((b as u16) << 8)
}

fn u16_to_u8s(x: u16) -> (u8, u8) {
    (x as u8, (x >> 8) as u8)
}

fn u8_get_bit(x: u8, offset: u8) -> bool {
    if offset > 7 { panic!(); }

    (x >> offset) & 1 == 1
}

fn u8_set_bit(x: &mut u8, offset: u8, value: bool) {
    if offset > 7 { panic!(); }

    let mask = 1 << offset;
    if value {
        *x |= mask;
    } else {
        *x &= !mask;
    }
}

fn load_boot_rom() -> Vec<u8> {
    return vec![
        0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32,
        0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
        0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3,
        0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
        0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A,
        0xCD, 0x95,    0, 0xCD, 0x96,    0, 0x13, 0x7B,
        0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8,    0, 0x06,
        0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
        0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99,
        0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
        0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64,
        0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
        0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90,
        0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
        0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62,
        0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
        0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42,
        0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
        0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04,
        0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
        0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9,
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D,    0, 0x0B,
        0x03, 0x73,    0, 0x83,    0, 0x0C,    0, 0x0D,
           0, 0x08, 0x11, 0x1F, 0x88, 0x89,    0, 0x0E,
        0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
        0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
        0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
        0x21, 0x04, 0x01, 0x11, 0xA8,    0, 0x1A, 0x13,
        0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
        0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20,
        0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
    ];
}

fn load_game_rom(game_name: &str) -> Vec<u8> {
    match game_name {
        "Pokemon Red (US)[:256]" => {
            return vec![
                0xFF,    0,    0,    0,    0,    0,    0,    0,
                0xFF,    0,    0,    0,    0,    0,    0,    0,
                0xFF,    0,    0,    0,    0,    0,    0,    0,
                0xFF,    0,    0,    0,    0,    0,    0,    0,
                0xFF,    0,    0,    0,    0,    0,    0,    0,
                0xFF,    0,    0,    0,    0,    0,    0,    0,
                0xFF,    0,    0,    0,    0,    0,    0,    0,
                0xFF,    0,    0,    0,    0,    0,    0,    0,
                0xC3, 0x24, 0x20,    0,    0,    0,    0,    0,
                0xFF,    0,    0,    0,    0,    0,    0,    0,
                0xC3, 0x06, 0x23,    0,    0,    0,    0,    0,
                0xC3, 0x25, 0x21,    0,    0,    0,    0,    0,
                0xD9, 0xAF, 0xE0, 0x0F, 0xF0, 0xFF, 0x47, 0xCB,
                0x87, 0xE0, 0xFF, 0xF0, 0x44, 0xFE, 0x91, 0x20,
                0xFA, 0xF0, 0x40, 0xE6, 0x7F, 0xE0, 0x40, 0x78,
                0xE0, 0xFF, 0xC9, 0xF0, 0x40, 0xCB, 0xFF, 0xE0,
                0x40, 0xC9, 0xAF, 0x21,    0, 0xC3, 0x06, 0xA0,
                0x22, 0x05, 0x20, 0xFC, 0xC9, 0x3E, 0xA0, 0x21,
                   0, 0xC3, 0x11, 0x04,    0, 0x06, 0x28, 0x77,
                0x19, 0x05, 0x20, 0xFB, 0xC9, 0xEA, 0xE9, 0xCE,
                0xF0, 0xB8, 0xF5, 0xFA, 0xE9, 0xCE, 0xE0, 0xB8,
                0xEA,    0, 0x20, 0xCD, 0xB5,    0, 0xF1, 0xE0,
                0xB8, 0xEA,    0, 0x20, 0xC9, 0x2A, 0x12, 0x13,
                0x0B, 0x79, 0xB0, 0x20, 0xF8, 0xC9,    0,    0,
                   0,    0,    0,    0,    0,    0,    0,    0,
                   0,    0,    0,    0,    0,    0,    0,    0,
                   0,    0,    0,    0,    0,    0,    0,    0,
                   0,    0,    0,    0,    0,    0,    0,    0,
                   0,    0,    0,    0,    0,    0,    0,    0,
                   0,    0,    0,    0,    0,    0,    0,    0,
                   0,    0,    0,    0,    0,    0,    0,    0,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ];
        }

        _ => {
            panic!("Game ROM Not Available: {}", game_name)
        }
    }
}
