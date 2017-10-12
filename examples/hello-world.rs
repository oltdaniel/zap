extern crate zap;

use std::io::Error as ZapError;
use zap::prelude::*;

// Our HelloWorld Handler
struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = ZapError;
    type Future = ZapResult;

    fn call(&self, _: Request) -> ZapResult {
        // Create new Response
        let mut resp = Response::new();

        // Set body, status is 200 by default
        resp.body("Hello World!");

        // Send response
        resp.ok()
    }
}

fn main() {
    // Set address
    let addr = "0.0.0.0:8080".parse().unwrap();

    // Create server
    let mut server = Server::new(Http, addr);

    // Set number of threads
    server.threads(8);

    // Serve the Handler
    server.serve(|| Ok(HelloWorld));
}
