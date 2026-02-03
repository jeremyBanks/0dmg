use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use zerodmg_emulator as emulator;

mod server;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_buffer = Arc::new(Mutex::new(emulator::Output::new()));
    let also_output_buffer = output_buffer.clone();

    // Spawn emulator on blocking thread
    tokio::task::spawn_blocking(move || {
        thread::sleep(Duration::from_millis(250));
        let mut gameboy = emulator::GameBoy::new(also_output_buffer.clone());
        gameboy.run();
    });

    // HTTP server
    println!("; Starting UI server at http://127.0.0.1:9898");
    let addr = SocketAddr::from(([127, 0, 0, 1], 9898));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let output_buffer = output_buffer.clone();

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| server::handle_request(req, output_buffer.clone())),
                )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
