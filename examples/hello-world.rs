extern crate zap;

use std::io::Error as ZapError;
use zap::prelude::*;

struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = ZapError;
    type Future = ZapResult;

    fn call(&self, _: Request) -> ZapResult {
        let mut resp = Response::new();

        resp.body("Hello World!");

        resp.ok()
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut server = Server::new(Http, addr);
    server.threads(8);
    server.serve(|| Ok(HelloWorld));
}
