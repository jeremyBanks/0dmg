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
        match addr {
            // Boot ROM, until unmapped to expose initial bytes of game ROM
            0x0000..=0x00FF if self.mem.boot_rom_mapped => self.mem.boot_rom[addr as usize],
            // Game ROM
            0x0000..=0x7FFF => self.mem.game_rom[addr as usize],
            // Video RAM
            0x8000..=0x9FFF => {
                let i = (addr - 0x8000) as usize;
                self.vram(i)
            }
            // Working RAM
            0xC000..=0xDFFF => {
                let i = (addr - 0xC000) as usize;
                self.mem.wram[i]
            }
            // Stack RAM
            0xFF80..=0xFFFE => {
                let i = (addr - 0xFF80) as usize;
                self.mem.stack_ram[i]
            }
            // Audio registers
            0xFF10..=0xFF26 => {
                let i = (addr - 0xFF10) as usize;
                self.audio_register(i)
            }
            // LCD Control
            0xFF40 => self.lcdc(),
            // Scroll Y
            0xFF42 => self.scy(),
            // Scroll X
            0xFF43 => self.scx(),
            // LCD Y-Coordinate
            0xFF44 => self.ly(),
            // Background Palette
            0xFF47 => self.bgp(),
            // Boot ROM disable register
            0xFF50 => {
                if self.mem.boot_rom_mapped {
                    0x01
                } else {
                    0x00
                }
            }
            // Serial Data (SB)
            0xFF01 => self.sb_register,
            // Serial Control (SC) - always reads 0
            0xFF02 => 0x00,
            // Interrupt Flag
            0xFF0F => self.ift(),
            // Interrupt Enable
            0xFFFF => self.ie(),
            _ => panic!("I don't know how to get memory address {addr:#06X}"),
        }
    }

    fn set_mem(&mut self, addr: u16, value: u8) {
        match addr {
            // Video RAM
            0x8000..=0x9FFF => {
                let i = (addr - 0x8000) as usize;
                self.set_vram(i, value);
            }
            // Working RAM
            0xC000..=0xDFFF => {
                let i = (addr - 0xC000) as usize;
                self.mem.wram[i] = value;
            }
            // Stack RAM
            0xFF80..=0xFFFE => {
                let i = (addr - 0xFF80) as usize;
                self.mem.stack_ram[i] = value;
            }
            // Audio registers
            0xFF10..=0xFF26 => {
                let i = (addr - 0xFF10) as usize;
                self.set_audio_register(i, value);
            }
            // LCD Control
            0xFF40 => self.set_lcdc(value),
            // Scroll Y
            0xFF42 => self.set_scy(value),
            // Scroll X
            0xFF43 => self.set_scx(value),
            // LCD Y-Coordinate
            0xFF44 => self.set_ly(value),
            // Background Palette
            0xFF47 => self.set_bgp(value),
            // Boot ROM disable register
            0xFF50 => {
                if value != 0x01 {
                    panic!(
                        "got unexpected value (not 0x01) written to 0xFF50 boot ROM disable register"
                    );
                }
                self.mem.boot_rom_mapped = false;
            }
            // Serial Data (SB)
            0xFF01 => self.sb_register = value,
            // Serial Control (SC) - writing 0x81 triggers transfer
            0xFF02 => {
                if value == 0x81 {
                    // Transfer initiated - capture SB to output buffer
                    self.serial_output.push(self.sb_register);
                }
            }
            // Interrupt Flag
            0xFF0F => self.set_ift(value),
            // Interrupt Enable
            0xFFFF => self.set_ie(value),
            _ => panic!(
                "I don't know how to set memory address {addr:#06X} (to {value:#04X})"
            ),
        }
    }
}
