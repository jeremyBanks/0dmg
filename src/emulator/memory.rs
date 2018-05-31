pub struct MemoryController {
    main_ram: [u8; 0x2000],
    video_ram: [u8; 0x2000],
    stack_ram: [u8; 0x7F],
    boot_rom: [u8; 0xFF],
}

impl MemoryController {
    pub fn new() -> Self {
        MemoryController {
            main_ram: [0x00; 0x2000],
            video_ram: [0x00; 0x2000],
            stack_ram: [0x00; 0x7F],
            boot_rom: [0x00; 0xFF],
        }
    }

    pub fn get(&mut self, addr: u16) -> u8 {

    }

    pub fn set(&mut self, addr: u16, value: u8) -> {

    }
}
