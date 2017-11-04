// Use the stuff we need to
use std::{io, str};

use bytes::BytesMut;

/// The result of an incoming request
pub struct Request {
    raw: BytesMut,
}

impl Request {
    /// Get raw body
    pub fn data(&self) -> &BytesMut {
        &self.raw
    }
}

/// Decode the given raw HTTP request
pub fn decode(buf: &mut BytesMut) -> io::Result<Option<Request>> {
    if buf.is_empty() {
        return Ok(None);
    }

    let request = Request {
        raw: buf.clone(),
    };

    // Clear buffer for next request
    buf.clear();

    // Create a new Request object
    Ok(request.into())
}
