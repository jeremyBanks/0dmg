use super::GameBoy;

// seems to be the right value to meet the apparent framerate
const CYCLES_PER_LINE: u64 = 113;

pub struct VideoData {
    t: u64,
    vram: [u8; 0x2000],
    // background palette register
    bgp: u8,
    // background scroll/offset x and y
    scx: u8,
    scy: u8,
    // LCD control register
    lcdc: u8,
    // LCD Y draw line
    ly: u8,
}

impl VideoData {
    pub fn new() -> Self {
        Self {
            t: 0,
            vram: [0x00; 0x2000],
            bgp: 0x00,
            scx: 0x00,
            scy: 0x00,
            lcdc: 0x00,
            ly: 0x00,
        }
    }
}

pub trait VideoController {
    fn video_cycle(&mut self);
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
    fn ly(&self) -> u8;
    fn set_ly(&mut self, value: u8);
    fn draw_frame_buffer(&mut self);
    fn draw_character(&mut self, index: usize, frame_buffer: &mut Vec<u8>);
}

impl VideoController for GameBoy {
    fn video_cycle(&mut self) {
        self.vid.t += 1;
        self.vid.ly = ((self.vid.t / CYCLES_PER_LINE) % (144 + 10)) as u8;

        // after vblank, draw
        if 0 == self.vid.ly && 0 == self.vid.t % CYCLES_PER_LINE {
            self.draw_frame_buffer();
        }
    }

    fn vram(&self, index: usize) -> u8 {
        self.vid.vram[index]
    }

    fn set_vram(&mut self, index: usize, value: u8) {
        // println!("    ; vram[${:02x}] = ${:02x}", index, value);
        self.vid.vram[index] = value;
    }

    fn draw_frame_buffer(&mut self) {
        // redraw frame buffer because vram was touched!
        let mut frame_buffer = {
            self.frame_buffer.lock().unwrap().clone()
        };
        let len = frame_buffer.len();

        for i in 0..len {
            frame_buffer[i] = 0;
        }

        for i in 0..1024 {
            let character_index = self.vid.vram[0x1800 + i];

            let character_data_index = character_index as usize * 16;
            let character_data = &self.vid.vram[character_data_index..character_data_index + 16];
            for j in 0..16 {
                // each byte is 4x1 pixels
                let byte = character_data[j];
                let x = ((j % 2) + i * 2).wrapping_sub(self.scx() as usize) % (160 / 4);
                let y = (((j / 2) + (i / (160 / 4)) * 8).wrapping_sub(self.scy() as usize)) % (144);
                frame_buffer[x + y * (160 / 4)] ^= byte;
            }
        }
        
        {
            let mut frame_buffer_for_write = self.frame_buffer.lock().unwrap();
            frame_buffer_for_write.clone_from(&frame_buffer);
        };
    }

    fn draw_character(&mut self, index: usize, frame_buffer: &mut Vec<u8>) {
    }

    fn bgp(&self) -> u8 {
        return self.vid.bgp;
    }

    fn set_bgp(&mut self, value: u8) {
        // println!("    ; vid bgp = ${:02x}", value);
        self.vid.bgp = value;
    }

    fn scy(&self) -> u8 {
        return self.vid.scy;
    }

    fn set_scy(&mut self, value: u8) {
        // println!("    ; vid scy = ${:02x}", value);
        self.vid.scy = value;
    }

    fn scx(&self) -> u8 {
        return self.vid.scx;
    }

    fn set_scx(&mut self, value: u8) {
        // println!("    ; vid scx = ${:02x}", value);
        self.vid.scx = value;
    }

    fn lcdc(&self) -> u8 {
        return self.vid.lcdc;
    }

    fn set_lcdc(&mut self, value: u8) {
        // println!("    ; vid lcdc = ${:02x}", value);
        self.vid.lcdc = value;
    }

    fn ly(&self) -> u8 {
        return self.vid.ly;
    }

    fn set_ly(&mut self, _value: u8) {
        panic!("writing to LY is not supported");
    }
}
