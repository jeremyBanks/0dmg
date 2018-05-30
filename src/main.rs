mod emulator;
mod server;

extern crate hyper;
use hyper::server::Http;
use std::thread;

pub fn main() {
    let http_server_thread = thread::spawn(move || {
        println!("; Starting UI server at http://127.0.0.1:9898");
        let addr = "127.0.0.1:9898".parse().unwrap();
        let http_server = Http::new()
            .bind(&addr, || Ok(server::GameBoyIOServer))
            .unwrap();
        http_server.run().unwrap();
    });

    let emulator_thread = thread::spawn(move || {
        let mut gameboy = emulator::GameBoy::new();
        gameboy.run();
    });

    http_server_thread.join();
    emulator_thread.join();

}
