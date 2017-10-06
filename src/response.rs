use bytes::{BytesMut, BufMut};
use futures::future;
use std::fmt::Write;

use ZapResult;

/// The result of an outgoing request
pub struct Response {
    headers: BytesMut,
    response: BytesMut,
    status: usize,
}

impl Response {
    // Create a new default response
    pub fn new() -> Response {
        Response {
            headers: BytesMut::new(),
            response: BytesMut::new(),
            status: 200,
        }
    }

    /// Set the status code
    pub fn status(&mut self, code: usize) -> &mut Response {
        self.status = code;
        self
    }

    /// Set a header with value
    pub fn header(&mut self, name: &str, val: &str) -> &mut Response {
        // Encode new header and write it to buffer
        push(&mut self.headers, name.as_bytes());
        push(&mut self.headers, b": ");
        push(&mut self.headers, val.as_bytes());
        push(&mut self.headers, b"\r\n");

        self
    }

    /// Set the response body
    pub fn body(&mut self, s: &str) -> &mut Response {
        self.response.write_str(s).unwrap();
        self
    }

    /// Write raw data to the body
    pub fn body_raw(&mut self, s: &[u8]) -> &mut Response {
        push(&mut self.response, s);
        self
    }

    // Finish the request
    pub fn ok(self) -> ZapResult {
        future::ok(self)
    }
}

/// Encode our response to a buffer
pub fn encode(msg: &Response, buf: &mut BytesMut) {
    // Get the content length
    let length = msg.response.len();

    // Encode the data
    push(buf, b"HTTP/1.1 ");
    push(buf, &usize_to_bytes(msg.status));
    push(buf, b"\r\nContent-Length: ");
    push(buf, &usize_to_bytes(length));
    push(buf, b"\r\n");
    push(buf, msg.headers.as_ref());

    push(buf, b"\r\n");
    push(buf, msg.response.as_ref());
}

// Push bytes to a buffer
fn push(buf: &mut BytesMut, data: &[u8]) {
    // Alloc new space
    buf.reserve(data.len());

    unsafe {
        // Put the data in the reserved space
        buf.bytes_mut()[..data.len()].copy_from_slice(data);
        buf.advance_mut(data.len());
    }
}

// Convert a usize to raw data without strings
fn usize_to_bytes(s : usize) -> [u8; 4] {
    // Define varibales we need
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
