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
    let frame_buffer_also = frame_buffer.clone();

    let http_server_thread = thread::spawn(move || {
        let frame_buffer = frame_buffer.clone();
        println!("; Starting UI server at http://127.0.0.1:9898");
        let addr = "127.0.0.1:9898".parse().unwrap();
        let http_server = Http::new()
            .bind(&addr, move || {
                let frame_buffer = frame_buffer.clone();
                Ok(server::GameBoyIOServer { frame_buffer })
            })
            .unwrap();
        http_server.run().unwrap();
    });

    let emulator_thread = thread::spawn(move || {
        let frame_buffer = frame_buffer_also.clone();
        let mut gameboy = emulator::GameBoy::new(frame_buffer);
        gameboy.run();
    });

    if let Err(error) = emulator_thread.join() {
        return Err(format!("from emulator thread: {:?}", error));
    }

    if let Err(error) = http_server_thread.join() {
        return Err(format!("from HTTP server thread: {:?}", error));
    }

    Ok(())
}
