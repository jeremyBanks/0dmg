use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

use hyper::header::{CacheControl, CacheDirective, ContentLength, ContentType};
use hyper::server::{Request, Response, Service};
use hyper::{Get, StatusCode};

use futures::future::Future;

use crate::emulator;

pub struct GameBoyIOServer {
    pub output_buffer: Arc<Mutex<emulator::Output>>,
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
                let mut f = File::open("./src/io.html").expect("file not found");

                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .expect("something went wrong reading the file");;

                Box::new(futures::future::ok(
                    Response::new()
                        .with_header(ContentLength(contents.len() as u64))
                        .with_header(ContentType::html())
                        .with_body(contents),
                ))
            }
            (&Get, "/output.png") => {
                let display = {
                    let mut output = self.output_buffer.lock().unwrap();
                    output.combined_image()
                };
                let mut encoded_image = Vec::new();
                display.write_to(&mut encoded_image, image::ImageOutputFormat::PNG);
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
