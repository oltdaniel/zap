extern crate bytes;
extern crate futures;
extern crate httparse;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

mod request;
mod response;

use std::io;

pub use request::Request;
pub use response::Response;

use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;
use futures::future;

pub use tokio_proto::TcpServer as Server;
pub use tokio_service::Service as Handler;

pub type ZapResult = future::Ok<Response, io::Error>;

pub mod prelude {
    pub use ZapResult;
    pub use Server;
    pub use Handler;
    pub use request::Request;
    pub use response::Response;
    pub use Http;
}

pub struct Http;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Http {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, HttpCodec>;
    type BindTransport = io::Result<Framed<T, HttpCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, HttpCodec>> {
        Ok(io.framed(HttpCodec))
    }
}

pub struct HttpCodec;

impl Decoder for HttpCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        request::decode(buf)
    }
}

impl Encoder for HttpCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
        response::encode(&msg, buf);
        Ok(())
    }
}
