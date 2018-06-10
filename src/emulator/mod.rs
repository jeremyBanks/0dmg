mod audio;
mod cpu;
mod memory;
mod video;

use self::audio::{AudioController, AudioData};
use self::cpu::{CPUController, CPUData, OperationExecution};
use self::memory::MemoryData;
use self::video::{VideoController, VideoData};
use std::clone::Clone;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const EXECUTIONS_BUFFER_SIZE: usize = 32;
extern crate image;
use self::image::{DynamicImage, GenericImage, ImageBuffer};

pub struct GameBoy {
    cpu: CPUData,
    mem: MemoryData,
    aud: AudioData,
    vid: VideoData,

    debug_latest_executions: Vec<OperationExecution>,
    debug_latest_executions_next_i: usize,

    t: u64,

    pub output_buffer: Arc<Mutex<Output>>,
}

pub struct Output {
    // Fully-Rendered Game Boy Display
    pub display: DynamicImage,
    // Tile Data
    pub tiles: DynamicImage,
    // Background Palette
    pub bgp: DynamicImage,
    // Object Palette 0
    pub op_0: DynamicImage,
    // Object Palette 1
    pub op_1: DynamicImage,
    // Background 1
    pub bg_0: DynamicImage,
    // Background 2
    pub bg_1: DynamicImage,
    // Sprites (Tile + Palette + Transform)
    pub sprites: DynamicImage,
}

impl Output {
    pub fn new() -> Self {
        let filled = |width: u32, height: u32| {
            let mut image = DynamicImage::ImageRgba8(ImageBuffer::new(width, height));
            let fill_colors = [
                image::Rgba([0x87, 0x9C, 0x57, 0xFF]),
                image::Rgba([0x87, 0x57, 0x9C, 0xFF]),
                image::Rgba([0x87, 0x87, 0x6C, 0xFF]),
            ];
            let border_colors = [
                image::Rgba([0xFF, 0x00, 0x00, 0xFF]),
                image::Rgba([0xCC, 0xCC, 0x00, 0xFF]),
                image::Rgba([0xCC, 0xCC, 0x00, 0xFF]),
            ];
            for x in 0..width {
                for y in 0..height {
                    image.put_pixel(
                        x,
                        y,
                        if x <= 1 || x >= width - 2 || y <= 1 || y >= height - 2 {
                            border_colors[(x + y) as usize % border_colors.len()]
                        } else {
                            fill_colors[(x + y) as usize % fill_colors.len()]
                        },
                    );
                }
            }
            image
        };

        Self {
            display: filled(160, 144),
            tiles: filled(128, 128),
            bgp: filled(4, 1),
            op_0: filled(3, 1),
            op_1: filled(3, 1),
            bg_0: filled(256, 256),
            bg_1: filled(256, 256),
            sprites: filled(80, 64),
        }
    }

    // Merges all of the output images into a single image, with them in a
    // vertical column in a consistent order.
    pub fn combined_image(&self) -> DynamicImage {
        let mut max_width = 0;
        let mut total_height = 0;
        let mut images = vec![
            &self.display,
            &self.tiles,
            &self.bgp,
            &self.op_0,
            &self.op_1,
            &self.bg_0,
            &self.bg_1,
            &self.sprites,
        ];
        for image in images.clone() {
            let (width, height) = image.dimensions();
            if width > max_width {
                max_width = width;
            }
            total_height += height;
        }
        let mut combined = ImageBuffer::new(max_width, total_height);
        let mut y = 0;
        for image in images.clone() {
            let (width, height) = image.dimensions();
            combined.copy_from(image, 0, y);
            y += height;
        }
        DynamicImage::ImageRgba8(combined)
    }
}

impl GameBoy {
    pub fn new(output_buffer: Arc<Mutex<Output>>) -> Self {
        let mut f = match File::open("./roms/default.gb") {
            Ok(f) => f,
            Err(_) => File::open("./roms/blargg-tests/instr_timing/instr_timing.gb")
                .expect("failed to open game ROM file"),
        };

        let mut game_rom = vec![];
        f.read_to_end(&mut game_rom)
            .expect("something went wrong reading the file");;

        Self {
            cpu: CPUData::new(),
            mem: MemoryData::new(game_rom),
            aud: AudioData::new(),
            vid: VideoData::new(),
            t: 0,
            debug_latest_executions: vec![],
            debug_latest_executions_next_i: 0,
            output_buffer: output_buffer,
        }
    }

    fn print_execution(&self, opex: &OperationExecution) {
        if let Some(s) = &opex.execution.asm {
            print!("{:32}", s);
        } else {
            print!("{:32}", "");
        }
        print!(" ; ${:04x}", opex.operation_address);
        print!(" ; {:10}", opex.t);
        let code = opex.operation_code
            .clone()
            .into_iter()
            .map(|c| format!("{:02x}", c))
            .collect::<Vec<String>>()
            .join("");
        print!(" ; ${:8}", code);
        if let Some(s) = &opex.execution.trace {
            print!(" ; {}", s);
        }
        println!();
    }

    pub fn print_recent_executions(&self) {
        println!("; assembly:                        addr:        t/μs:   codes:      flags:");
        println!("; ---------                        -----        -----   ------      ------");

        let len = self.debug_latest_executions.len();
        for i in 0..len {
            let offset_i = (self.debug_latest_executions_next_i + i) % len;
            let opex = &self.debug_latest_executions[offset_i];
            self.print_execution(opex);
        }
        println!();
    }

    pub fn run(&mut self) -> ! {
        let mut last_color: &'static str = "";
        let red = "\x1b[91m";
        let green = "\x1b[92m";
        let yellow = "\x1b[93m";
        let blue = "\x1b[94m";
        let clear = "\x1b[0m";

        // using Instant seems to produced very unsteady results
        let start_time = SystemTime::now();
        let sync_time_every_ticks = 1024 * 4;
        let mut sync_time_at_tick = sync_time_every_ticks;

        loop {
            let opex = self.tick();
            let cycles = opex.execution.cycles;
            if self.debug_latest_executions.len() < EXECUTIONS_BUFFER_SIZE {
                self.debug_latest_executions.push(opex);
            } else {
                self.debug_latest_executions[self.debug_latest_executions_next_i] = opex;
            }

            self.debug_latest_executions_next_i =
                (self.debug_latest_executions_next_i + 1) % EXECUTIONS_BUFFER_SIZE;

            for _ in 0..cycles {
                self.video_cycle();
                self.audio_cycle();

                if self.t % (1024 * 1024) == 0 {
                    self.print_recent_executions();
                }

                self.t += 1;
            }

            if self.t >= sync_time_at_tick {
                sync_time_at_tick += sync_time_every_ticks;

                // duration by which we allow internal time to slip ahead of real time,
                // for the sake of doing several operations in a batch, rather than
                // sleeping between each of them
                const BATCH_MARGIN: Duration = Duration::from_millis(4);
                const MAX_LAG: Duration = Duration::from_millis(4);
                const ZERO: Duration = Duration::from_secs(0);

                // TODO: this is exactly 1MHz, which is wrong.
                let internal_elapsed =
                    Duration::new(self.t / 1000000, ((self.t * 1000) % 1000000000) as u32); // 953 should really be 1000000000 / 1048576
                let wall_elapsed = start_time.elapsed().expect("failed to get elapsed time?!");
                let skew_ahead = if internal_elapsed > wall_elapsed {
                    internal_elapsed - wall_elapsed
                } else {
                    ZERO
                };
                let skew_behind = if wall_elapsed > internal_elapsed {
                    wall_elapsed - internal_elapsed
                } else {
                    ZERO
                };

                // println!("internal / wall = {:?} / {:?}", internal_elapsed, wall_elapsed);
                // println!("behind / ahead = {:?} / {:?}", skew_behind, skew_ahead);

                if skew_ahead > BATCH_MARGIN {
                    // going too fast -- sleep a bit
                    sleep(skew_ahead);
                    if clear != last_color {
                        last_color = clear;
                        print!("{}", clear);
                    }
                } else if skew_behind > MAX_LAG {
                    // going waaay too slow! crap!
                    if red != last_color {
                        last_color = red;
                        print!("{}", red);
                    }
                } else if skew_ahead > ZERO {
                    // going good
                    if clear != last_color {
                        last_color = clear;
                        print!("{}", clear);
                    }
                } else {
                    // going a bit slow
                    if yellow != last_color {
                        last_color = yellow;
                        print!("{}", yellow);
                    }
                }
            }
        }
    }
}
