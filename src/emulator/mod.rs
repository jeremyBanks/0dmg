
mod audio;
mod cpu;
mod memory;
mod video;

use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Read;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use self::audio::{AudioController, AudioData};
use self::cpu::{CPUController, CPUData, OperationExecution};
use self::memory::MemoryData;
use self::video::{VideoController, VideoData};

const EXECUTIONS_BUFFER_SIZE: usize = 32;
extern crate image;
use self::image::{GenericImage, DynamicImage, ImageBuffer};

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
        Self {
            display: DynamicImage::ImageRgba8(ImageBuffer::new(160, 144)),
            tiles: DynamicImage::ImageRgba8(ImageBuffer::new(256, 256)),
            bgp: DynamicImage::ImageRgba8(ImageBuffer::new(4, 1)),
            op_0: DynamicImage::ImageRgba8(ImageBuffer::new(3, 1)),
            op_1: DynamicImage::ImageRgba8(ImageBuffer::new(3, 1)),
            bg_0: DynamicImage::ImageRgba8(ImageBuffer::new(256, 256)),
            bg_1: DynamicImage::ImageRgba8(ImageBuffer::new(256, 256)),
            sprites: DynamicImage::ImageRgba8(ImageBuffer::new(80, 64)),
        }
    }

    // Merges all of the output images into a single image, with them in a
    // vertical column in a consistent order.
    pub fn combined_image(&self) -> DynamicImage {
        let mut maxWidth = 0;
        let mut totalHeight = 0;
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
            let (height, width) = image.dimensions();
            if width > maxWidth {
                maxWidth = width;
            }
            totalHeight += height;
        }
        let mut combined = ImageBuffer::new(maxWidth, totalHeight);
        let mut y = 0;
        for image in images.clone() {
            let (height, _width) = image.dimensions();
            combined.copy_from(image, 0, y);
            y += height;
        }
        DynamicImage::ImageRgba8(combined)
    }
}

impl GameBoy {
    pub fn new(output_buffer: Arc<Mutex<Output>>) -> Self {
        let mut f = File::open("./roms/blargg-tests/instr_timing/instr_timing.gb").expect("file not found");

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
        let code = opex.operation_code.clone()
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

            self.debug_latest_executions_next_i = (self.debug_latest_executions_next_i + 1) % EXECUTIONS_BUFFER_SIZE;

            for _ in 0..cycles {
                self.video_cycle();
                self.audio_cycle();

                if self.t % (1024*1024) == 0 {
                    self.print_recent_executions();
                }

                self.t += 1;
            }

            if self.t >= sync_time_at_tick {
                sync_time_at_tick += sync_time_every_ticks;

                // duration by which we allow internal time to slip ahead of real time,
                // for the sake of doing several operations in a batch, rather than
                // sleeping between each of them
                const BATCH_MARGIN:Duration = Duration::from_millis(4);
                const MAX_LAG:Duration = Duration::from_millis(4);
                const ZERO:Duration = Duration::from_secs(0);

                // TODO: this is exactly 1MHz, which is wrong.
                let internal_elapsed = Duration::new(self.t / 1000000, ((self.t * 1000) % 1000000000) as u32); // 953 should really be 1000000000 / 1048576
                let wall_elapsed = start_time.elapsed().expect("failed to get elapsed time?!");
                let skew_ahead = if internal_elapsed > wall_elapsed { internal_elapsed - wall_elapsed } else { ZERO };
                let skew_behind = if wall_elapsed > internal_elapsed { wall_elapsed - internal_elapsed } else { ZERO };


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
