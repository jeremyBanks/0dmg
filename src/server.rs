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
                        .with_body(encoded_image),
                ))
            }
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}
