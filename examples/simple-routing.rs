extern crate zap;

use std::io::Error as ZapError;
use zap::prelude::*;
use std::str;

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

        // Different content, depending on route
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

        // Set parameters
        resp.body(body);
        resp.status(status);

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
