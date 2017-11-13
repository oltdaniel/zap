//! The `zap` web framework crate.
//!
//! ## Example
//!
//! Here a short HelloWorld example writte in Rust with zap:
//!
//! ```no_run
//! extern crate zap;
//!
//! use std::io::Error as ZapError;
//! use zap::prelude::*;
//!
//! struct HelloWorld;
//!
//! impl Handler for HelloWorld {
//!     type Request = Request;
//!     type Response = Response;
//!     type Error = ZapError;
//!     type Future = ZapResult;
//!
//!     fn call(&self, _ : Request) -> ZapResult {
//!         let mut resp = Response::new();
//!
//!         resp.body("Hello World!");
//!
//!         resp.ok()
//!     }
//! }
//! ```
//!

// Load all crates and modules
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

mod request;
mod response;

// Use the stuff we need to
use std::io;

use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;
use futures::future;

// Make Server, Handler, Request and Response public accessable
pub use tokio_proto::TcpServer as Server;
pub use tokio_service::Service as Handler;
pub use request::Request;
pub use response::Response;

/// The expected response result
pub type ZapResult = future::Ok<Response, io::Error>;

/// A module to import the required things.
pub mod prelude {
    pub use ZapResult;
    pub use Server;
    pub use Handler;
    pub use request::Request;
    pub use response::Response;
    pub use Http;
}

/// Handling incoming requests with tokio-proto
pub struct Http;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Http {
    // Setup ServerProto types
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, HttpCodec>;
    type BindTransport = io::Result<Framed<T, HttpCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, HttpCodec>> {
        // Frame the request with tokio-io and handle it with HttpCodec
        Ok(io.framed(HttpCodec))
    }
}

/// Encode and decode our request
pub struct HttpCodec;

impl Decoder for HttpCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        // Decode our buffer
        request::decode(buf)
    }
}

impl Encoder for HttpCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
        // Encode and write response
        response::encode(&msg, buf);
        Ok(())
    }
}
