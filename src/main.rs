#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]

use std::any::Any;
use std::clone::Clone;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use hyper::server::Http;

use zerodmg_emulator as emulator;

mod server;

pub fn main() -> Result<(), Box<Any + Send>> {
    let output_buffer = Arc::new(Mutex::new(emulator::Output::new()));
    let also_output_buffer = output_buffer.clone();

    let emulator_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(250));
        let mut gameboy = emulator::GameBoy::new(also_output_buffer.clone());
        gameboy.run();
    });

    let http_server_thread = thread::spawn(move || {
        let output_buffer = output_buffer.clone();
        println!("; Starting UI server at http://127.0.0.1:9898");
        let addr = "127.0.0.1:9898".parse().unwrap();
        Http::new()
            .bind(&addr, move || {
                Ok(server::GameBoyIOServer {
                    output_buffer: output_buffer.clone(),
                })
            }).unwrap()
            .run()
            .unwrap();
    });

    emulator_thread.join()?;
    http_server_thread.join()?;

    Ok(())
}
