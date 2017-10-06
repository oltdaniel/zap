use bytes::{BytesMut, BufMut};
use futures::future;
use std::fmt::Write;

use ZapResult;

pub struct Response {
    headers: BytesMut,
    response: BytesMut,
    status: usize,
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: BytesMut::new(),
            response: BytesMut::new(),
            status: 200,
        }
    }

    pub fn status(&mut self, code: usize) -> &mut Response {
        self.status = code;
        self
    }

    pub fn header(&mut self, name: &str, val: &str) -> &mut Response {
        push(&mut self.headers, name.as_bytes());
        push(&mut self.headers, b": ");
        push(&mut self.headers, val.as_bytes());
        push(&mut self.headers, b"\r\n");

        self
    }

    pub fn body(&mut self, s: &str) -> &mut Response {
        self.response.write_str(s).unwrap();
        self
    }

    pub fn body_raw(&mut self, s: &[u8]) -> &mut Response {
        push(&mut self.response, s);
        self
    }

    pub fn ok(self) -> ZapResult {
        future::ok(self)
    }
}

pub fn encode(msg: &Response, buf: &mut BytesMut) {
    let length = msg.response.len();

    push(buf, b"HTTP/1.1 ");
    push(buf, &usize_to_bytes(msg.status));
    push(buf, b"\r\nContent-Length: ");
    push(buf, &usize_to_bytes(length));
    push(buf, b"\r\n");
    push(buf, msg.headers.as_ref());

    push(buf, b"\r\n");
    push(buf, msg.response.as_ref());
}

fn push(buf: &mut BytesMut, data: &[u8]) {
    buf.reserve(data.len());
    unsafe {
        buf.bytes_mut()[..data.len()].copy_from_slice(data);
        buf.advance_mut(data.len());
    }
}

fn usize_to_bytes(s : usize) -> [u8; 4] {
    let mut data : [u8; 4] = [0; 4];
    let mut length = s as u32;

    // Convert u16 to ASCII bytes
    for i in 1..5 {
        let base = (10u16.pow(4 - (i as u32))) as u32;
        data[i - 1] = 48 + (&length / &base) as u8;
        length = (&length % &base) as u32;
    }

    return data;
}
