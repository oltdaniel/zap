use bytes::{BytesMut, BufMut};
use futures::future;

use ZapResult;

pub struct Response {
    headers: Vec<(String, String)>,
    response: String,
    status: usize,
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: Vec::new(),
            response: String::new(),
            status: 200,
        }
    }

    pub fn status(&mut self, code: usize) -> &mut Response {
        self.status = code;
        self
    }

    pub fn header(&mut self, name: &str, val: &str) -> &mut Response {
        self.headers.push((name.to_string(), val.to_string()));
        self
    }

    pub fn body(&mut self, s: &str) -> &mut Response {
        self.response = s.to_string();
        self
    }

    pub fn ok(self) -> ZapResult {
        future::ok(self)
    }
}

pub fn encode(msg: &Response, buf: &mut BytesMut) {
    let length = msg.response.len();

    push(buf, &[72, 84, 84, 80, 47, 49, 46, 49, 32]); // "HTTP/1.1 "
    push(buf, &usize_to_bytes(msg.status));
    push(buf, &[13, 10, 67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32]); // "Content-Length: "
    push(buf, &usize_to_bytes(length));
    push(buf, &[13, 10]); // "\r\n"

    for &(ref k, ref v) in &msg.headers {
        push(buf, k.as_bytes());
        push(buf, &[58, 32]); // ": "
        push(buf, v.as_bytes());
        push(buf, &[13, 10]); // "\r\n"
    }

    push(buf, &[13, 10]); // "\r\n"
    push(buf, msg.response.as_bytes());
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
    let mut length = s as u16;

    // Convert u16 to ASCII bytes
    for i in 1..5 {
        let base = (10 * i) as u16;
        data[4 - i] = 48 + (&length % &base) as u8;
        length = &length / &base;
    }

    return data;
}
