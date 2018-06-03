use super::roms;
use super::GameBoy;

use super::audio::AudioController;
use super::video::VideoController;

pub struct MemoryData {
    main_ram: [u8; 0x2000],
    stack_ram: [u8; 0x80],
    boot_rom: [u8; 0x100],
    game_rom: Vec<u8>,
    boot_rom_mapped: bool,
}

impl MemoryData {
    pub fn new() -> Self {
        Self {
            main_ram: [0x00; 0x2000],
            stack_ram: [0x00; 0x80],
            boot_rom: roms::BOOT.clone(),
            game_rom: roms::GAME_STUB.to_vec(),
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
            // first page of game ROM
            self.mem.game_rom[addr as usize]
        } else if 0x8000 <= addr && addr <= 0x9FFF {
            let i: usize = (addr - 0x8000) as usize;
            self.vram(i)
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
        } else if addr == 0xFF47 {
            self.bgp()
        } else {
            panic!("I don't know how to get memory address ${:04x}.", addr);
        }
    }

    fn set_mem(&mut self, addr: u16, value: u8) {
        if 0x8000 <= addr && addr <= 0x9FFF {
            let i: usize = (addr - 0x8000) as usize;
            self.set_vram(i, value);
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
        } else if addr == 0xFF47 {
            self.set_bgp(value);
        } else {
            panic!("I don't know how to set memory address ${:04x}.", addr);
        }
    }
}
