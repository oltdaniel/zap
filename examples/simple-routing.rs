extern crate zap;

use std::io::Error as ZapError;
use zap::prelude::*;
use std::str;

struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = ZapError;
    type Future = ZapResult;

    fn call(&self, req: Request) -> ZapResult {
        let mut resp = Response::new();

        let (status, body) = match (req.method(), req.path()) {
            ("GET", "/") => {
                (200, "Hello World!")
            },
            ("GET", "/bye") => {
                (200, "Bye World!")
            },
            ("POST", "/echo") => {
                let b = str::from_utf8(req.body().as_ref()).unwrap_or("No Body");
                (200, b)
            }
            _ => {
                (404, "Not Found")
            }
        };

        resp.body(body);
        resp.status(status);

        resp.ok()
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut server = Server::new(Http, addr);
    server.threads(8);
    server.serve(|| Ok(HelloWorld));
}
