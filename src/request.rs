// Use the stuff we need to
use std::io;

use bytes::BytesMut;

/// The result of an incoming request
pub struct Request {
    first: BytesMut,
    head: BytesMut,
    body: BytesMut,
}

impl Request {
    /// Get raw body
    pub fn body(&self) -> &BytesMut {
        &self.body
    }

    /// Get first line
    pub fn first(&self) -> &BytesMut {
        &self.first
    }

    /// Get head
    pub fn head(&self) -> &BytesMut {
        &self.head
    }
}

/// Decode the given raw HTTP request
pub fn decode(buf: &mut BytesMut) -> io::Result<Option<Request>> {
    if buf.is_empty() {
        return Ok(None);
    }

    // Clone buffer for iter
    let iter = buf.clone();

    // Loop over bytes, to find line endings
    let mut it = iter.iter();
    let mut firstc : u16 = 0_u16;
    let mut headc : u16 = 0_u16;

    loop {
        // Check if end is reached
        match it.next() {
            Some(&b'\n') => break,
            None => break,
            _ => {
                firstc += 1;
            }
        };
    }

    // Cache headers line length
    let mut linec : u16 = 0_u16;

    loop {
        // Check if end of headers reached
        match it.next() {
            // On line end
            Some(&b'\n') => {
                // Headers end reached
                if linec == 1 {
                    break;
                }

                // Increment total length
                headc += 1;

                // Reset line length
                linec = 0;
            },
            // Buffer end
            None => break,
            _ => {
                // Else increment length
                linec += 1;
                headc += 1;
            }
        };
    }

    // Split buffers into parts
    let first = buf.split_to(firstc as usize);
    let head = buf.split_to(headc as usize);
    let body = buf.clone();

    // Clear buffer for next request
    buf.clear();

    // Build request object
    let request = Request {
        first: first,
        head: head,
        body: body,
    };

    // Create a new Request object
    Ok(request.into())
}
