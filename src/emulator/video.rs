use super::{GameBoy, Output};

extern crate image;
use self::image::{DynamicImage, GenericImage, ImageBuffer};

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
        let (mut display, mut bg_0, mut tiles) = {
            let output_buffer = self.output_buffer.lock().unwrap();
            (
                output_buffer.display.clone(),
                output_buffer.bg_0.clone(),
                output_buffer.tiles.clone(),
            )
        };

        // draw background palettes
        

        // draw tiles into debug buffer
        for i in 0..256 {
            let tile_data = &self.vid.vram[i * 16..(i + 1) * 16];
            let mut new_tile_data = vec![0u8; 16];
            new_tile_data.clone_from_slice(tile_data);
            for y_offset in 0..8 {
                let low_byte = new_tile_data[y_offset * 2];
                let high_byte = new_tile_data[y_offset * 2 + 1];
                let first_byte = ((high_byte & 0b10000000) >> 0) + ((high_byte & 0b01000000) >> 1)
                    + ((high_byte & 0b00100000) >> 2)
                    + ((high_byte & 0b00010000) >> 3)
                    + ((low_byte & 0b10000000) >> 1)
                    + ((low_byte & 0b01000000) >> 2)
                    + ((low_byte & 0b00100000) >> 3)
                    + ((low_byte & 0b00010000) >> 4);
                let second_byte = ((high_byte & 0b00001000) << 4) + ((high_byte & 0b00000100) << 3)
                    + ((high_byte & 0b00000010) << 1)
                    + ((high_byte & 0b00000001) << 1)
                    + ((low_byte & 0b00001000) << 3)
                    + ((low_byte & 0b00000100) << 2)
                    + ((low_byte & 0b00000010) << 1)
                    + ((low_byte & 0b00000001) << 0);
                new_tile_data[y_offset * 2] = first_byte;
                new_tile_data[y_offset * 2 + 1] = second_byte;
            }

            for j in 0..16 {
                let tile_col = (i % 16) as u32;
                let x_tile_offset = 8 * tile_col as i64;
                let x = ((x_tile_offset
                        + 4 * (j % 2) as i64) % 256) as u32;

                let tile_row = (i / 16) as u32;
                let y_tile_offset = 8 * tile_row as i64;
                let y = ((y_tile_offset
                        + 1 * (j / 2) as i64) % 256) as u32;

                let byte = new_tile_data[j];
                let a = (byte & 0b11000000) >> 6;
                let a_color = image::Rgba([
                    a * 0b01010101,
                    a * 0b01010101,
                    a * 0b01010101,
                    0xFF,
                ]);
                let b = (byte & 0b00110000) >> 4;
                let b_color = image::Rgba([
                    b * 0b01010101,
                    b * 0b01010101,
                    b * 0b01010101,
                    0xFF,
                ]);
                let c = (byte & 0b00001100) >> 2;
                let c_color = image::Rgba([
                    c * 0b01010101,
                    c * 0b01010101,
                    c * 0b01010101,
                    0xFF,
                ]);
                let d = (byte & 0b00000011) >> 0;
                let d_color = image::Rgba([
                    d * 0b01010101,
                    d * 0b01010101,
                    d * 0b01010101,
                    0xFF,
                ]);

                tiles.put_pixel(x + 0 + tile_col, y + tile_row, a_color);
                tiles.put_pixel(x + 1 + tile_col, y + tile_row, b_color);
                tiles.put_pixel(x + 2 + tile_col, y + tile_row, c_color);
                tiles.put_pixel(x + 3 + tile_col, y + tile_row, d_color);
            }
        }

        // draw background
        for i in 0..1024 {
            let tile_index = self.vid.vram[0x1800 + i];
            let tile_data_index = tile_index as usize * 16;
            let tile_data = &self.vid.vram[tile_data_index..tile_data_index + 16];

            let mut new_tile_data = vec![0u8; 16];
            new_tile_data.clone_from_slice(tile_data);

            for y_offset in 0..8 {
                let low_byte = new_tile_data[y_offset * 2];
                let high_byte = new_tile_data[y_offset * 2 + 1];
                let first_byte = ((high_byte & 0b10000000) >> 0) + ((high_byte & 0b01000000) >> 1)
                    + ((high_byte & 0b00100000) >> 2)
                    + ((high_byte & 0b00010000) >> 3)
                    + ((low_byte & 0b10000000) >> 1)
                    + ((low_byte & 0b01000000) >> 2)
                    + ((low_byte & 0b00100000) >> 3)
                    + ((low_byte & 0b00010000) >> 4);
                let second_byte = ((high_byte & 0b00001000) << 4) + ((high_byte & 0b00000100) << 3)
                    + ((high_byte & 0b00000010) << 1)
                    + ((high_byte & 0b00000001) << 1)
                    + ((low_byte & 0b00001000) << 3)
                    + ((low_byte & 0b00000100) << 2)
                    + ((low_byte & 0b00000010) << 1)
                    + ((low_byte & 0b00000001) << 0);
                new_tile_data[y_offset * 2] = first_byte;
                new_tile_data[y_offset * 2 + 1] = second_byte;
            }

            for j in 0..16 {
                let tile_col = (i % 32) as u32;
                let x_tile_offset = 8 * tile_col as i64;
                let x = ((x_tile_offset
                        + 4 * (j % 2) as i64) % 256) as u32;
                let scrolled_x = ((x as i64
                        - self.scx() as i64) % 256) as u32;

                let tile_row = (i / 32) as u32;
                let y_tile_offset = 8 * tile_row as i64;
                let y = ((y_tile_offset
                        + 1 * (j / 2) as i64) % 256) as u32;
                let scrolled_y = ((y as i64
                    - self.scy() as i64) % 256) as u32;

                let byte = !new_tile_data[j];
                let a = (byte & 0b11000000) >> 6;
                let a_color = image::Rgba([a * 0b01010101, a * 0b01010101, a * 0b01010101, 0xFF]);
                let b = (byte & 0b00110000) >> 4;
                let b_color = image::Rgba([b * 0b01010101, b * 0b01010101, b * 0b01010101, 0xFF]);
                let c = (byte & 0b00001100) >> 2;
                let c_color = image::Rgba([c * 0b01010101, c * 0b01010101, c * 0b01010101, 0xFF]);
                let d = (byte & 0b00000011) >> 0;
                let d_color = image::Rgba([d * 0b01010101, d * 0b01010101, d * 0b01010101, 0xFF]);

                bg_0.put_pixel((x + tile_col + 0) % (256 + 31), (y + tile_row) % (256 + 31), a_color);
                bg_0.put_pixel((x + tile_col + 1) % (256 + 31), (y + tile_row) % (256 + 31), b_color);
                bg_0.put_pixel((x + tile_col + 2) % (256 + 31), (y + tile_row) % (256 + 31), c_color);
                bg_0.put_pixel((x + tile_col + 3) % (256 + 31), (y + tile_row) % (256 + 31), d_color);

                if scrolled_y < 144 {
                    if (scrolled_x + 0) % 256 < 160 {
                        display.put_pixel(scrolled_x + 0, scrolled_y, a_color);
                    }
                    if (scrolled_x + 1) % 256 < 160 {
                        display.put_pixel(scrolled_x + 1, scrolled_y, b_color);
                    }
                    if (scrolled_x + 2) % 256 < 160 {
                        display.put_pixel(scrolled_x + 2, scrolled_y, c_color);
                    }
                    if (scrolled_x + 3) % 256 < 160 {
                        display.put_pixel(scrolled_x + 3, scrolled_y, d_color);
                    }
                }
            }
        }

        {
            let mut self_output_buffer = self.output_buffer.lock().unwrap();
            self_output_buffer.display = display;
            self_output_buffer.bg_0 = bg_0;
            self_output_buffer.tiles = tiles;
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
