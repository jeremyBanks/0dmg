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
    let frame_buffer = Arc::new(Mutex::new(vec![0u8; 160 * 144 / 4]));
    let also_frame_buffer = frame_buffer.clone();

    let emulator_thread = thread::spawn(move || {
        let mut gameboy = emulator::GameBoy::new(also_frame_buffer.clone());
        gameboy.run();
    });

    let http_server_thread = thread::spawn(move || {
        let frame_buffer = frame_buffer.clone();
        println!("; Starting UI server at http://127.0.0.1:9898");
        let addr = "127.0.0.1:9898".parse().unwrap();
        Http::new()
            .bind(&addr, move || {
                Ok(server::GameBoyIOServer { frame_buffer: frame_buffer.clone() })
            })
            .unwrap()
            .run()
            .unwrap();
    });

    if let Err(_error) = emulator_thread.join() {
        println!("; emulator thread panicked");
        thread::sleep_ms(125);
        return Err(format!("emulator thread panicked"));
    }

    if let Err(_error) = http_server_thread.join() {
        return Err(format!("HTTP server thread panicked"));
    }

    Ok(())
}
