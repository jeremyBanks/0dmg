use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Method, Request, Response, StatusCode};

/// Handle HTTP requests for the emulator display
pub async fn handle_request(
    req: Request<hyper::body::Incoming>,
    output_buffer: Arc<Mutex<zerodmg_emulator::Output>>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    // println!("; {} {}", req.method(), req.uri().path());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let html = include_bytes!("io.html");
            Ok(Response::builder()
                .header("content-type", "text/html")
                .header("content-length", html.len())
                .body(Full::new(Bytes::from_static(html)))
                .unwrap())
        }
        (&Method::GET, "/output.png") => {
            let display = output_buffer.lock().unwrap().combined_image();
            let mut encoded_image = Vec::new();
            display
                .write_to(&mut encoded_image, image::ImageOutputFormat::Png)
                .expect("failed to write image to memory buffer -- really?!");

            Ok(Response::builder()
                .header("content-type", "image/png")
                .header("content-length", encoded_image.len())
                .header("cache-control", "no-store")
                .body(Full::new(Bytes::from(encoded_image)))
                .unwrap())
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::default())
            .unwrap()),
    }
}
