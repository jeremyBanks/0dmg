use super::roms;
use super::GameBoy;

pub struct MemoryData {
    main_ram: [u8; 0x2000],
    video_ram: [u8; 0x2000],
    stack_ram: [u8; 0x80],
    boot_rom: [u8; 0x100],
    game_rom: Vec<u8>,
    boot_rom_mapped: bool,
}

impl MemoryData {
    pub fn new() -> Self {
        Self {
            main_ram: [0x00; 0x2000],
            video_ram: [0x00; 0x2000],
            stack_ram: [0x00; 0x80],
            boot_rom: roms::BOOT.clone(),
            game_rom: roms::GAME_STUB.to_vec(),
            boot_rom_mapped: true,
        }
    }
}

pub trait MemoryController {
    fn get(&mut self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, value: u8);
}

impl MemoryController for GameBoy {
    fn get(&mut self, addr: u16) -> u8 {
        let value;
        if self.mem.boot_rom_mapped && addr <= 0x00FF {
            // boot ROM, until unmapped to expose initial bytes of game ROM
            value = self.mem.boot_rom[addr as usize];
        } else if addr <= 0x7FFF {
            // first page of game ROM
            value = self.mem.game_rom[addr as usize];
        } else if 0x8000 <= addr && addr <= 0x9FFF {
            let i: usize = (addr - 0x8000) as usize;
            value = self.mem.video_ram[i];
        } else if 0xFF80 <= addr && addr <= 0xFFFE {
            let i: usize = (addr - 0xFF80) as usize;
            value = self.mem.stack_ram[i];
        } else {
            panic!("I don't know how to get memory address ${:04x}.", addr);
        }

        value
    }

    fn set(&mut self, addr: u16, value: u8) {
        if 0x8000 <= addr && addr <= 0x9FFF {
            let i: usize = (addr - 0x8000) as usize;
            self.mem.video_ram[i] = value;
            println!("  ; video_ram[${:04x}] = ${:02x}", i, value);
        } else if 0xFF80 <= addr && addr <= 0xFFFE {
            let i: usize = (addr - 0xFF80) as usize;
            self.mem.stack_ram[i] = value;
            println!("  ; stack_ram[${:02x}] = ${:02x}", i, value);
        } else if 0xFF10 <= addr && addr <= 0xFF26 {
            println!("  ; skipping write to sound control memory -- not implemented");
        } else if addr == 0xFF47 {
            // this should probably delegate to a VideoController
            println!("  ; skiping background pallet update");
        } else {
            panic!("I don't know how to set memory address ${:04x}.", addr);
        }
    }
}
