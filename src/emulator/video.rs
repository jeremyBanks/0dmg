use super::{GameBoy, Output};


extern crate image;
use self::image::{GenericImage, DynamicImage, ImageBuffer};

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
    fn draw_output(&mut self);
}

impl VideoController for GameBoy {
    fn video_cycle(&mut self) {
        self.vid.t += 1;
        self.vid.ly = ((self.vid.t / CYCLES_PER_LINE) % (144 + 10)) as u8;

        // after vblank, draw
        if 0 == self.vid.ly && 0 == self.vid.t % CYCLES_PER_LINE {
            self.draw_output();
        }
    }

    fn vram(&self, index: usize) -> u8 {
        self.vid.vram[index]
    }

    fn set_vram(&mut self, index: usize, value: u8) {
        // println!("    ; vram[${:02x}] = ${:02x}", index, value);
        self.vid.vram[index] = value;
    }

    fn draw_output(&mut self) {
        // redraw display because vram was touched!
        let mut display = {
            self.output_buffer.lock().unwrap().display.clone()
        };

        // draw background
        for i in 0..1024 {
            let character_index = self.vid.vram[0x1800 + i];
            let character_data_index = character_index as usize * 16;
            let character_data = &self.vid.vram[character_data_index..character_data_index + 16];

            let mut new_character_data = vec![0u8; 16];
            new_character_data.clone_from_slice(character_data);

            for y_offset in 0..8 {
                let low_byte = new_character_data[y_offset * 2];
                let high_byte = new_character_data[y_offset * 2 + 1];
                let first_byte = 
                    ((high_byte & 0b10000000) >> 0) +
                    ((high_byte & 0b01000000) >> 1) + 
                    ((high_byte & 0b00100000) >> 2) + 
                    ((high_byte & 0b00010000) >> 3) + 
                    ((low_byte & 0b10000000) >> 1) +
                    ((low_byte & 0b01000000) >> 2) + 
                    ((low_byte & 0b00100000) >> 3) + 
                    ((low_byte & 0b00010000) >> 4);
                let second_byte = 
                    ((high_byte & 0b00001000) << 4) +
                    ((high_byte & 0b00000100) << 3) + 
                    ((high_byte & 0b00000010) << 1) + 
                    ((high_byte & 0b00000001) << 1) + 
                    ((low_byte & 0b00001000) << 3) +
                    ((low_byte & 0b00000100) << 2) + 
                    ((low_byte & 0b00000010) << 1) + 
                    ((low_byte & 0b00000001) << 0);
                new_character_data[y_offset * 2] = first_byte;
                new_character_data[y_offset * 2 + 1] = second_byte;
            }

            for j in 0..16 {
                let x =
                    ((0
                        // tile offset
                        + 8 * (i % 32) as i64
                        // pixel offset
                        + 4 * (j % 2) as i64
                        // scroll offset
                        - self.scx() as i64
                    ) % 256) as u32;
                    
                let y = 
                ((0
                    // tile offset
                    + 8 * (i / 32) as i64
                    // pixel offset
                    + 1 * (j / 2) as i64
                    // scroll offset
                    - self.scy() as i64
                ) % 256) as u32;
                
                if x + 3 >= 160 { continue; }
                if y >= 144 { continue; }
                
                let byte = !new_character_data[j];
                let a = (byte & 0b11000000) >> 6;
                let b = (byte & 0b00110000) >> 4;
                let c = (byte & 0b00001100) >> 2;
                let d = (byte & 0b00000011) >> 0;
                display.put_pixel(x + 0, y, image::Rgba([a * 0b01010101, a * 0b01010101, a * 0b01010101, 0xFF]));
                display.put_pixel(x + 1, y, image::Rgba([b * 0b01010101, b * 0b01010101, b * 0b01010101, 0xFF]));
                display.put_pixel(x + 2, y, image::Rgba([c * 0b01010101, c * 0b01010101, c * 0b01010101, 0xFF]));
                display.put_pixel(x + 3, y, image::Rgba([d * 0b01010101, d * 0b01010101, d * 0b01010101, 0xFF]));
            }
        }
        
        {
            let mut self_output_buffer = self.output_buffer.lock().unwrap();
            self_output_buffer.display = display;
        };
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
