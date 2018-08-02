use std::sync::{Arc, Mutex};

use hyper::header::{CacheControl, CacheDirective, ContentLength, ContentType};
use hyper::server::{Request, Response, Service};
use hyper::{Get, StatusCode};

use futures::future::Future;

use zerodmg_emulator;

/// Simple HTTP server displaying emulator output
pub struct GameBoyIOServer {
    pub output_buffer: Arc<Mutex<zerodmg_emulator::Output>>,
}

impl Service for GameBoyIOServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        // println!("; {} {}", req.method(), req.path());

        match (req.method(), req.path()) {
            (&Get, "/") => {
                let html = include_bytes!("io.html").to_vec();
                Box::new(futures::future::ok(
                    Response::new()
                        .with_header(ContentLength(html.len() as u64))
                        .with_header(ContentType::html())
                        .with_body(html),
                ))
            }
            (&Get, "/output.png") => {
                let display = self.output_buffer.lock().unwrap().combined_image();
                let mut encoded_image = Vec::new();
                display
                    .write_to(&mut encoded_image, image::ImageOutputFormat::PNG)
                    .expect("failed to write image to memory buffer -- really?!");
                Box::new(futures::future::ok(
                    Response::new()
                        .with_header(ContentLength(encoded_image.len() as u64))
                        .with_header(CacheControl(vec![CacheDirective::NoStore]))
                        .with_body(encoded_image),
                ))
            }
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}
