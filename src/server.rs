use std::fs::File;
use std::io::prelude::*;

extern crate hyper;
use server::hyper::header::{ContentLength, ContentType};
use server::hyper::server::{Request, Response, Service};
use server::hyper::{Get, StatusCode};

extern crate rand;

extern crate futures;
use server::futures::future::Future;

pub struct GameBoyIOServer;

impl Service for GameBoyIOServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        println!("; {} {}", req.method(), req.path());

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
                let r = rand::random::<f64>();
                let mut contents;
                if r < 1.0 / 3.0 {
                    contents = "\x00".repeat(160 * 144 / 4).to_string();
                } else if r < 2.0 / 3.0 {
                    contents = "\x42".repeat(160 * 144 / 4).to_string();
                } else {
                    contents = "\x7F".repeat(160 * 144 / 4).to_string();
                }

                Box::new(futures::future::ok(
                    Response::new()
                        .with_header(ContentLength(contents.len() as u64))
                        .with_body(contents.clone()),
                ))
            }
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}
