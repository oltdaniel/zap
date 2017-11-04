extern crate zap;

use std::io::Error as ZapError;
use zap::prelude::*;

// Our handler
struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = ZapError;
    type Future = ZapResult;

    fn call(&self, req: Request) -> ZapResult {
        // Create new Response
        let mut resp = Response::new();
        let head = req.data();

        // Different content, depending on route
        if head.starts_with(b"GET / HTTP/1.1\r\n") {
            resp.body_raw(b"Hello World");
        } else if head.starts_with(b"GET /bye HTTP/1.1\r\n") {
            resp.body_raw(b"Bye Bye");
        } else {
            resp.body_raw(b"Not Found");
            resp.status(404);
        }

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
