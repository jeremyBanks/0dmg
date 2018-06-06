use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

extern crate hyper;
use server::hyper::header::{ContentLength, ContentType};
use server::hyper::server::{Request, Response, Service};
use server::hyper::{Get, StatusCode};

extern crate futures;
use server::futures::future::Future;

use emulator;
extern crate image;
use self::image::{GenericImage, DynamicImage, ImageBuffer};

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
            (&Get, "/frame") => {
                let output_buffer = self.output_buffer.lock().unwrap();

                let image = output_buffer.combined_image();
                let mut encoded_buffer = Cursor::new(Vec::new());
                let (width, heigth) = image.dimensions();
                {
                    let encoder = image::png::PNGEncoder::new(&encoded_buffer);
                    encoder.encode(image)
                }

                Box::new(futures::future::ok(
                    Response::new()
                        .with_header(ContentLength(output_buffer.len() as u64))
                        .with_body(output_buffer.clone()),
                ))
            }
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}
