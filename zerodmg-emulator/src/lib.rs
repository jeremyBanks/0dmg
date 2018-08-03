#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![feature(try_from)]
// #![warn(missing_docs, missing_debug_implementations)]

mod audio;
mod cpu;
mod memory;
mod video;

use self::audio::{AudioController, AudioData};
use self::cpu::{CPUController, CPUData, InstructionExecution};
use self::memory::MemoryData;
use self::video::{VideoController, VideoData};
use std::clone::Clone;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use zerodmg_codes;

const EXECUTIONS_BUFFER_SIZE: usize = 1024;
use image::{DynamicImage, GenericImage, ImageBuffer};

pub struct GameBoy {
    cpu: CPUData,
    mem: MemoryData,
    aud: AudioData,
    vid: VideoData,

    debug_latest_executions: Vec<InstructionExecution>,
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

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

impl Output {
    pub fn new() -> Self {
        let filled = |width: u32, height: u32| {
            let mut image = DynamicImage::ImageRgba8(ImageBuffer::new(width, height));
            let fill_colors = [image::Rgba([0x90, 0x60, 0x90, 0xFF])];
            let border_colors = [image::Rgba([0xCC, 0x33, 0xCC, 0xFF])];
            let border_width = 1;
            for x in 0..width {
                for y in 0..height {
                    image.put_pixel(
                        x,
                        y,
                        if x < border_width
                            || x >= width - border_width
                            || y < border_width
                            || y >= height - border_width
                        {
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
            tiles: filled(128 + 15, 128 + 15),
            bgp: filled(4, 1),
            op_0: filled(3, 1),
            op_1: filled(3, 1),
            bg_0: filled(256, 256),
            bg_1: filled(256, 256),
            sprites: filled(80 + 4, 64 + 7),
        }
    }

    // Merges all of the output images into a single image, with them in a
    // vertical column in a consistent order.
    pub fn combined_image(&self) -> DynamicImage {
        let mut max_width = 0;
        let mut total_height = 0;
        let images = vec![
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
            let (_width, height) = image.dimensions();
            combined.copy_from(image, 0, y);
            y += height;
        }
        DynamicImage::ImageRgba8(combined)
    }
}

impl GameBoy {
    pub fn new(output_buffer: Arc<Mutex<Output>>) -> Self {
        use zerodmg_codes::roms::*;

        let game_rom = jeb_demo().assemble().to_bytes();

        Self {
            cpu: CPUData::new(),
            mem: MemoryData::new(game_rom),
            aud: AudioData::new(),
            vid: VideoData::new(),
            t: 0,
            debug_latest_executions: vec![],
            debug_latest_executions_next_i: 0,
            output_buffer,
        }
    }

    pub fn print_recent_executions(&mut self, limit: usize) {
        println!("; assembly:                        addr:         t|μs:   codes:");
        println!("; ---------                        ------        -----   --------");

        let len = self.debug_latest_executions.len();
        for i in 0..len.min(limit) {
            let offset_i = (self.debug_latest_executions_next_i + i) % len;
            let opex = &self.debug_latest_executions[offset_i];
            self.print_execution(opex);
        }

        self.debug_latest_executions.clear();
        self.debug_latest_executions_next_i = 0;

        println!();
    }

    fn print_execution(&self, opex: &InstructionExecution) {
        print!("{:32}", format!("{}", opex.instruction));
        print!(" ; {:6}", opex.source);
        print!(" ; {:10}", opex.t_0);
        let code = opex
            .instruction
            .to_bytes()
            .into_iter()
            .map(|c| format!("{:02X}", c))
            .collect::<Vec<String>>()
            .join("");
        print!(" ; 0x{:8}", code);
        if let Some(ref tracer) = opex.tracer {
            let trace = tracer();
            print!(" ; {}", trace);
        }
        println!();
    }

    pub fn run(&mut self) -> ! {
        let log_size = EXECUTIONS_BUFFER_SIZE.min(32);
        let log_interval = (1024 * 1024) / 2;

        let mut last_color: &'static str = "";
        let red = "\x1b[91m";
        let _green = "\x1b[92m";
        let yellow = "\x1b[93m";
        let _blue = "\x1b[94m";
        let clear = "\x1b[0m";

        // using Instant seems to produced very unsteady results
        let start_time = SystemTime::now();
        let sync_time_every_ticks = 1024 * 4;
        let mut sync_time_at_tick = sync_time_every_ticks;

        loop {
            let opex = self.tick();

            let t_0 = opex.t_0;
            let t_1 = opex.t_1;

            if self.debug_latest_executions.len() < EXECUTIONS_BUFFER_SIZE {
                self.debug_latest_executions.push(opex);
            } else {
                self.debug_latest_executions[self.debug_latest_executions_next_i] = opex;
            }

            self.debug_latest_executions_next_i =
                (self.debug_latest_executions_next_i + 1) % EXECUTIONS_BUFFER_SIZE;

            let mut should_log = false;

            for _t in t_0..t_1 {
                self.video_cycle();
                self.audio_cycle();

                if (self.t + log_interval - log_interval.min(log_size as u64)) % log_interval == 0 {
                    should_log = true;
                }

                self.t += 1;
            }

            if self.t >= sync_time_at_tick {
                sync_time_at_tick += sync_time_every_ticks;

                // duration by which we allow internal time to slip ahead of real time,
                // for the sake of doing several operations in a batch, rather than
                // sleeping between each of them
                const BATCH_MARGIN: Duration = Duration::from_millis(8);
                const MAX_LAG: Duration = Duration::from_millis(8);
                const ZERO: Duration = Duration::from_secs(0);

                // TODO: this is exactly 1MHz, which is wrong.
                let internal_elapsed =
                    Duration::new(self.t / 1000000, ((self.t * 1000) % 1000000000) as u32);
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

                if skew_ahead > BATCH_MARGIN {
                    // going too fast -- sleep a bit
                    thread::sleep(skew_ahead);
                    if clear != last_color {
                        last_color = clear;
                        print!("{}", clear);
                    }
                } else if skew_ahead > ZERO {
                    // going good
                    if clear != last_color {
                        last_color = clear;
                        print!("{}", clear);
                    }
                } else if skew_behind < MAX_LAG {
                    // going a bit slow
                    if yellow != last_color {
                        last_color = yellow;
                        print!("{}", yellow);
                    }
                } else {
                    // going waaay too slow! crap!
                    if red != last_color {
                        last_color = red;
                        print!("{}", red);
                    }
                }
            }

            if should_log {
                self.print_recent_executions(log_size);
            }
        }
    }
}
