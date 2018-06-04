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

pub struct GameBoy {
    cpu: CPUData,
    mem: MemoryData,
    aud: AudioData,
    vid: VideoData,

    debug_latest_executions: Vec<OperationExecution>,
    debug_latest_executions_next_i: usize,

    t: u64,

    pub frame_buffer: Arc<Mutex<Vec<u8>>>,
}

impl GameBoy {
    pub fn new(frame_buffer: Arc<Mutex<Vec<u8>>>) -> Self {
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
            frame_buffer: frame_buffer,
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
