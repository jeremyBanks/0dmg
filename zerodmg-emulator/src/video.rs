use super::GameBoy;

use image::GenericImage;

// seems to be the right value to meet the apparent framerate
const CYCLES_PER_LINE: u64 = 113;

/// Game Boy video memory state
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

const GB_WIDTH: u8 = 160;
const GB_HEIGHT: u8 = 144;

impl VideoData {
    pub fn new() -> Self {
        Self {
            t: 0,
            vram: {
                let mut a = [0u8; 0x2000];
                for x in a.iter_mut() {
                    *x = rand::random();
                }
                a
            },
            bgp: rand::random(),
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
        self.vid.ly = ((self.vid.t / CYCLES_PER_LINE) % u64::from(GB_HEIGHT + 10)) as u8;

        // after vblank, draw
        if 0 == self.vid.ly && 0 == self.vid.t % CYCLES_PER_LINE {
            self.draw_output();
        }
    }

    fn vram(&self, index: usize) -> u8 {
        self.vid.vram[index]
    }

    fn set_vram(&mut self, index: usize, value: u8) {
        // println!("    ; vram[0x{:02X}] = 0x{:02X}", index, value);
        self.vid.vram[index] = value;
    }

    fn draw_output(&mut self) {
        // redraw display because vram was touched!
        let (mut display, mut bg_0, mut tiles, mut bgp) = {
            let output_buffer = self.output_buffer.lock().unwrap();
            (
                output_buffer.display.clone(),
                output_buffer.bg_0.clone(),
                output_buffer.tiles.clone(),
                output_buffer.bgp.clone(),
            )
        };

        // draw background palettes
        // the ! is because the GB colours values are darkness, but RGB is brightness
        let bgp_a = (!self.bgp() & 0b1100_0000) >> 6;
        let bgp_a_color = image::Rgba([
            bgp_a * 0b0101_0101,
            bgp_a * 0b0101_0101,
            bgp_a * 0b0101_0101,
            0xFF,
        ]);
        let bgp_b = (!self.bgp() & 0b0011_0000) >> 4;
        let bgp_b_color = image::Rgba([
            bgp_b * 0b0101_0101,
            bgp_b * 0b0101_0101,
            bgp_b * 0b0101_0101,
            0xFF,
        ]);
        let bgp_c = (!self.bgp() & 0b0000_1100) >> 2;
        let bgp_c_color = image::Rgba([
            bgp_c * 0b0101_0101,
            bgp_c * 0b0101_0101,
            bgp_c * 0b0101_0101,
            0xFF,
        ]);
        let bgp_d = (!self.bgp() & 0b0000_0011) >> 0;
        let bgp_d_color = image::Rgba([
            bgp_d * 0b0101_0101,
            bgp_d * 0b0101_0101,
            bgp_d * 0b0101_0101,
            0xFF,
        ]);
        bgp.put_pixel(0, 0, bgp_a_color);
        bgp.put_pixel(1, 0, bgp_b_color);
        bgp.put_pixel(2, 0, bgp_c_color);
        bgp.put_pixel(3, 0, bgp_d_color);
        let bg_palette = [bgp_a_color, bgp_b_color, bgp_c_color, bgp_d_color];

        // draw tiles into debug buffer
        for i in 0..256 {
            let tile_data = &self.vid.vram[i * 16..(i + 1) * 16];
            let mut new_tile_data = vec![0u8; 16];
            new_tile_data.clone_from_slice(tile_data);
            for y_offset in 0..8 {
                let low_byte = new_tile_data[y_offset * 2 + 1];
                let high_byte = new_tile_data[y_offset * 2];
                let first_byte = ((high_byte & 0b1000_0000) >> 0)
                    | ((low_byte & 0b1000_0000) >> 1)
                    | ((high_byte & 0b0100_0000) >> 1)
                    | ((low_byte & 0b0100_0000) >> 2)
                    | ((high_byte & 0b0010_0000) >> 2)
                    | ((low_byte & 0b0010_0000) >> 3)
                    | ((high_byte & 0b0001_0000) >> 3)
                    | ((low_byte & 0b0001_0000) >> 4);
                let second_byte = ((high_byte & 0b0000_1000) << 4)
                    | ((low_byte & 0b0000_1000) << 3)
                    | ((high_byte & 0b0000_0100) << 3)
                    | ((low_byte & 0b0000_0100) << 2)
                    | ((high_byte & 0b0000_0010) << 2)
                    | ((low_byte & 0b0000_0010) << 1)
                    | ((high_byte & 0b0000_0001) << 1)
                    | ((low_byte & 0b0000_0001) << 0);
                new_tile_data[y_offset * 2] = first_byte;
                new_tile_data[y_offset * 2 + 1] = second_byte;
            }

            for j in 0..16 {
                let tile_col = (i % 16) as u32;
                let x_tile_offset = 8 * i64::from(tile_col);
                let x = ((x_tile_offset + 4 * (j % 2) as i64) % 256) as u32;

                let tile_row = (i / 16) as u32;
                let y_tile_offset = 8 * i64::from(tile_row);
                let y = ((y_tile_offset + 1 * (j / 2) as i64) % 256) as u32;

                let byte = !new_tile_data[j];
                let a = (byte & 0b1100_0000) >> 6;
                let a_color =
                    image::Rgba([a * 0b0101_0101, a * 0b0101_0101, a * 0b0101_0101, 0xFF]);
                let b = (byte & 0b0011_0000) >> 4;
                let b_color =
                    image::Rgba([b * 0b0101_0101, b * 0b0101_0101, b * 0b0101_0101, 0xFF]);
                let c = (byte & 0b0000_1100) >> 2;
                let c_color =
                    image::Rgba([c * 0b0101_0101, c * 0b0101_0101, c * 0b0101_0101, 0xFF]);
                let d = (byte & 0b0000_0011) >> 0;
                let d_color =
                    image::Rgba([d * 0b0101_0101, d * 0b0101_0101, d * 0b0101_0101, 0xFF]);

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
                // XXX: this is really dumb.
                // why are you converting back into bytes?
                // convert into pixels.
                let low_byte = new_tile_data[y_offset * 2 + 1];
                let high_byte = new_tile_data[y_offset * 2];
                let first_byte = ((high_byte & 0b1000_0000) >> 0)
                    | ((low_byte & 0b1000_0000) >> 1)
                    | ((high_byte & 0b0100_0000) >> 1)
                    | ((low_byte & 0b0100_0000) >> 2)
                    | ((high_byte & 0b0010_0000) >> 2)
                    | ((low_byte & 0b0010_0000) >> 3)
                    | ((high_byte & 0b0001_0000) >> 3)
                    | ((low_byte & 0b0001_0000) >> 4);
                let second_byte = ((high_byte & 0b0000_1000) << 4)
                    | ((low_byte & 0b0000_1000) << 3)
                    | ((high_byte & 0b0000_0100) << 3)
                    | ((low_byte & 0b0000_0100) << 2)
                    | ((high_byte & 0b0000_0010) << 2)
                    | ((low_byte & 0b0000_0010) << 1)
                    | ((high_byte & 0b0000_0001) << 1)
                    | ((low_byte & 0b0000_0001) << 0);
                new_tile_data[y_offset * 2] = first_byte;
                new_tile_data[y_offset * 2 + 1] = second_byte;
            }

            for j in 0..16 {
                let tile_col = (i % 32) as u32;
                let x_tile_offset = 8 * i64::from(tile_col);
                let x = ((x_tile_offset + 4 * (j % 2) as i64) % 256) as u32;
                let scrolled_x = (x + 256 - u32::from(self.scx())) % 256;

                let tile_row = (i / 32) as u32;
                let y_tile_offset = 8 * i64::from(tile_row);
                let y = ((y_tile_offset + 1 * (j / 2) as i64) % 256) as u32;
                let scrolled_y = (y + 256 - u32::from(self.scy())) % 256;

                let byte = !new_tile_data[j];
                let a = (byte & 0b1100_0000) >> 6;
                let a_color = bg_palette[a as usize];
                let b = (byte & 0b0011_0000) >> 4;
                let b_color = bg_palette[b as usize];
                let c = (byte & 0b0000_1100) >> 2;
                let c_color = bg_palette[c as usize];
                let d = (byte & 0b0000_0011) >> 0;
                let d_color = bg_palette[d as usize];

                bg_0.put_pixel((x + 0) % 256, y % 256, a_color);
                bg_0.put_pixel((x + 1) % 256, y % 256, b_color);
                bg_0.put_pixel((x + 2) % 256, y % 256, c_color);
                bg_0.put_pixel((x + 3) % 256, y % 256, d_color);

                if scrolled_y < GB_HEIGHT.into() {
                    if ((scrolled_x + 0) as u8) < GB_WIDTH {
                        display.put_pixel((scrolled_x + 0) % 256, scrolled_y, a_color);
                    }
                    if ((scrolled_x + 1) as u8) < GB_WIDTH {
                        display.put_pixel((scrolled_x + 1) % 256, scrolled_y, b_color);
                    }
                    if ((scrolled_x + 2) as u8) < GB_WIDTH {
                        display.put_pixel((scrolled_x + 2) % 256, scrolled_y, c_color);
                    }
                    if ((scrolled_x + 3) as u8) < GB_WIDTH {
                        display.put_pixel((scrolled_x + 3) % 256, scrolled_y, d_color);
                    }
                }
            }
        }

        // Draw border around active background in debug buffer.
        let border_width: i16 = 12;
        for dy in -border_width..border_width {
            let dya: u8 = (if dy > 0 { dy + 1 } else { -dy }) as u8;
            let dyp = if dy < 0 {
                dy
            } else {
                dy + i16::from(GB_HEIGHT)
            };
            let y = u32::from((i16::from(self.scy()) + dyp) as u8);

            for x in 0..=0xFF {
                let mut color = bg_0.get_pixel(x, y);
                color.data[3] = ((color.data[3] as u32 * dya as u32) / (border_width as u32)) as u8;
                bg_0.put_pixel(x, y, color);
            }
        }
        for dx in -border_width..border_width {
            let dxa: u8 = (if dx > 0 { dx + 1 } else { -dx }) as u8;
            let dxp = if dx < 0 { dx } else { dx + i16::from(GB_WIDTH) };
            let x = u32::from((i16::from(self.scx()) + dxp) as u8);

            for y in 0..=0xFF {
                let mut color = bg_0.get_pixel(x, y);
                color.data[3] = ((color.data[3] as u32 * dxa as u32) / (border_width as u32)) as u8;
                bg_0.put_pixel(x, y, color);
            }
        }

        {
            let mut self_output_buffer = self.output_buffer.lock().unwrap();
            self_output_buffer.display = display;
            self_output_buffer.bg_0 = bg_0;
            self_output_buffer.tiles = tiles;
            self_output_buffer.bgp = bgp;
        };
    }

    fn bgp(&self) -> u8 {
        self.vid.bgp
    }

    fn set_bgp(&mut self, value: u8) {
        // println!("    ; vid bgp = 0x{:02X}", value);
        self.vid.bgp = value;
    }

    fn scy(&self) -> u8 {
        self.vid.scy
    }

    fn set_scy(&mut self, value: u8) {
        // println!("    ; vid scy = 0x{:02X}", value);
        self.vid.scy = value;
    }

    fn scx(&self) -> u8 {
        self.vid.scx
    }

    fn set_scx(&mut self, value: u8) {
        // println!("    ; vid scx = 0x{:02X}", value);
        self.vid.scx = value;
    }

    fn lcdc(&self) -> u8 {
        self.vid.lcdc
    }

    fn set_lcdc(&mut self, value: u8) {
        // println!("    ; vid lcdc = 0x{:02X}", value);
        self.vid.lcdc = value;
    }

    fn ly(&self) -> u8 {
        self.vid.ly
    }

    fn set_ly(&mut self, _value: u8) {
        panic!("writing to LY is not supported");
    }
}
