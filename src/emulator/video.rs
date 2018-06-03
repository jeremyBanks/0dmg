use super::GameBoy;

pub struct VideoData {
    vram: [u8; 0x2000],
    // background palette register
    bgp: u8,
    // background scroll/offset x and y
    scx: u8,
    scy: u8,
    // LCD control register
    lcdc: u8,
}

impl VideoData {
    pub fn new() -> Self {
        Self {
            vram: [0x00; 0x2000],
            bgp: 0x00,
            scx: 0x00,
            scy: 0x00,
            lcdc: 0x00,
        }
    }
}

pub trait VideoController {
    fn vram(&self, index: usize) -> u8;
    fn set_vram(&mut self, index: usize, value: u8);
    fn bgp(&self) -> u8;
    fn set_bgp(&mut self, value: u8);
    fn scy(&self) -> u8;
    fn set_scy(&mut self, value: u8);
    fn scx(&self) -> u8;
    fn set_scx(&mut self, value: u8);
    fn lcdc(&self) -> u8;
    fn set_lcdc(&mut self, value: u8);
}

impl VideoController for GameBoy {
    fn vram(&self, index: usize) -> u8 {
        self.vid.vram[index]
    }

    fn set_vram(&mut self, index: usize, value: u8) {
        println!("    ; vram[${:02x}] = ${:02x}", index, value);
        self.vid.vram[index] = value;
    }

    fn bgp(&self) -> u8 {
        return self.vid.bgp;
    }

    fn set_bgp(&mut self, value: u8) {
        println!("    ; vid bgp = ${:02x}", value);
        self.vid.bgp = value;
    }

    fn scy(&self) -> u8 {
        return self.vid.scy;
    }

    fn set_scy(&mut self, value: u8) {
        println!("    ; vid scy = ${:02x}", value);
        self.vid.scy = value;
    }

    fn scx(&self) -> u8 {
        return self.vid.scx;
    }

    fn set_scx(&mut self, value: u8) {
        println!("    ; vid scx = ${:02x}", value);
        self.vid.scx = value;
    }

    fn lcdc(&self) -> u8 {
        return self.vid.lcdc;
    }

    fn set_lcdc(&mut self, value: u8) {
        println!("    ; vid lcdc = ${:02x}", value);
        self.vid.lcdc = value;
    }
}
