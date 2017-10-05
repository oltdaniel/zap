extern crate zap;

use std::io;

use zap::prelude::*;

struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = ZapResult;

    fn call(&self, req: Request) -> ZapResult {
        let mut resp = Response::new();

        match (req.method(), req.path()) {
            ("GET", "/") => {
                resp.body("Hello World!");
            },
            ("GET", "/bye") => {
                resp.body("Bye World!");
            },
            _ => {
                resp.body("Not Found").status(404);
            }
        }

        resp.ok()
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut server = Server::new(Http, addr);
    server.threads(8);
    server.serve(|| Ok(HelloWorld));
}
