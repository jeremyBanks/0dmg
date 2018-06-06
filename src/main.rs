#![feature(reverse_bits)]

use std::sync::{Arc, Mutex};
use std::thread;

extern crate hyper;
use hyper::server::Http;

mod emulator;
mod server;

pub fn main() {
    try_main().unwrap();
}

pub fn try_main() -> Result<(), String> {
    let output_buffer = Arc::new(Mutex::new(emulator::Output::new()));
    let also_output_buffer = output_buffer.clone();

    let mut b = {
        output_buffer.lock().unwrap().clone()
    };
    {
        let mut c = output_buffer.lock().unwrap();
        c.clone_from(&b);
    };

    let emulator_thread = thread::spawn(move || {
        thread::sleep_ms(250);
        let mut gameboy = emulator::GameBoy::new(also_output_buffer.clone());
        gameboy.run();
    });

    let http_server_thread = thread::spawn(move || {
        let output_buffer = output_buffer.clone();
        println!("; Starting UI server at http://127.0.0.1:9898");
        let addr = "127.0.0.1:9898".parse().unwrap();
        Http::new()
            .bind(&addr, move || {
                Ok(server::GameBoyIOServer { output_buffer: output_buffer.clone() })
            })
            .unwrap()
            .run()
            .unwrap();
    });

    if let Err(_error) = emulator_thread.join() {
        println!("; emulator thread panicked");
        thread::sleep_ms(5000);
        return Err(format!("emulator thread panicked"));
    }

    if let Err(_error) = http_server_thread.join() {
        return Err(format!("HTTP server thread panicked"));
    }

    Ok(())
}
