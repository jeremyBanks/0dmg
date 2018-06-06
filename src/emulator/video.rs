use super::{GameBoy, Output};

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

        // clear frame buffer
        for i in 0..len {
            frame_buffer[i] = 0;
        }

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
                new_character_data[y_offset * 2] = first_byte.reverse_bits() ^ (y_offset as u8);
                new_character_data[y_offset * 2 + 1] = second_byte.reverse_bits() ^ (y_offset as u8);
            }

            for j in 0..16 {
                let byte = new_character_data[j];
                let x = ((((j % 2) + i * 2) as u8).wrapping_sub(self.scx())) as usize;
                // if x >= 160 / 4 { continue; }
                let y = ((((j / 2) + 2 * (i / 32)) as u8).wrapping_sub(self.scy())) as usize;
                // if y >= 144 { continue; }
                frame_buffer[(x % (160 / 4)) + (y % 144) * (160 / 4)] |= byte;
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
