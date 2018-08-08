use super::GameBoy;

use super::audio::AudioController;
use super::cpu::CPUController;
use super::video::VideoController;

/// Game Boy general memory state
pub struct MemoryData {
    wram: [u8; 0x2000],
    stack_ram: [u8; 0x80],
    boot_rom: Vec<u8>,
    game_rom: Vec<u8>,
    boot_rom_mapped: bool,
}

impl MemoryData {
    pub fn new(game_rom: Vec<u8>) -> Self {
        Self {
            wram: {
                let mut a = [0u8; 0x2000];
                for x in a.iter_mut() {
                    *x = rand::random();
                }
                a
            },
            stack_ram: {
                let mut a = [0u8; 0x80];
                for x in a.iter_mut() {
                    *x = rand::random();
                }
                a
            },
            game_rom,
            boot_rom: zerodmg_codes::roms::dmg_boot().to_bytes(),
            boot_rom_mapped: true,
        }
    }
}

pub trait MemoryController {
    fn mem(&self, addr: u16) -> u8;
    fn set_mem(&mut self, addr: u16, value: u8);
}

impl MemoryController for GameBoy {
    fn mem(&self, addr: u16) -> u8 {
        if self.mem.boot_rom_mapped && addr <= 0x00FF {
            // boot ROM, until unmapped to expose initial bytes of game ROM
            self.mem.boot_rom[addr as usize]
        } else if addr <= 0x7FFF {
            // println!("    ; game_rom[0x{:02X}] == 0x{:02X}", addr, self.mem.game_rom[addr as usize]);
            // first page of game ROM
            self.mem.game_rom[addr as usize]
        } else if 0x8000 <= addr && addr <= 0x9FFF {
            let i: usize = (addr - 0x8000) as usize;
            self.vram(i)
        } else if 0xC000 <= addr && addr <= 0xDFFF {
            let i: usize = (addr - 0xC000) as usize;
            self.mem.wram[i]
        } else if 0xFF80 <= addr && addr <= 0xFFFE {
            let i: usize = (addr - 0xFF80) as usize;
            self.mem.stack_ram[i]
        } else if 0xFF10 <= addr && addr <= 0xFF26 {
            let i = (addr - 0xFF10) as usize;
            self.audio_register(i)
        } else if addr == 0xFF40 {
            self.lcdc()
        } else if addr == 0xFF42 {
            self.scy()
        } else if addr == 0xFF43 {
            self.scx()
        } else if addr == 0xFF44 {
            self.ly()
        } else if addr == 0xFF47 {
            self.bgp()
        } else if addr == 0xFF50 {
            if self.mem.boot_rom_mapped {
                0x01
            } else {
                0x00
            }
        } else if addr == 0xFF0F {
            self.ift()
        } else if addr == 0xFFFF {
            self.ie()
        } else {
            panic!("I don't know how to get memory address 0x{:04X}.", addr);
        }
    }

    fn set_mem(&mut self, addr: u16, value: u8) {
        if 0x8000 <= addr && addr <= 0x9FFF {
            let i: usize = (addr - 0x8000) as usize;
            self.set_vram(i, value);
        } else if 0xC000 <= addr && addr <= 0xDFFF {
            let i: usize = (addr - 0xC000) as usize;
            self.mem.wram[i] = value;
        } else if 0xFF80 <= addr && addr <= 0xFFFE {
            let i: usize = (addr - 0xFF80) as usize;
            self.mem.stack_ram[i] = value;
        } else if 0xFF10 <= addr && addr <= 0xFF26 {
            let i = (addr - 0xFF10) as usize;
            self.set_audio_register(i, value);
        } else if addr == 0xFF40 {
            self.set_lcdc(value);
        } else if addr == 0xFF42 {
            self.set_scy(value);
        } else if addr == 0xFF43 {
            self.set_scx(value);
        } else if addr == 0xFF44 {
            self.set_ly(value);
        } else if addr == 0xFF47 {
            self.set_bgp(value);
        } else if addr == 0xFF50 {
            if value != 0x01 {
                panic!(
                    "got unexpected value (not 0x01) written to 0xFF50 boot rom disable register"
                );
            }
            self.mem.boot_rom_mapped = false;
        } else if addr == 0xFF0F {
            self.set_ift(value);
        } else if addr == 0xFFFF {
            self.set_ie(value);
        } else {
            panic!(
                "I don't know how to set memory address 0x{:04X} (to 0x{:02X}).",
                addr, value
            );
        }
    }
}
