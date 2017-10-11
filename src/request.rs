// Use the stuff we need to
use std::{io, slice, str};

use bytes::BytesMut;

use httparse;

/// The result of an incoming request
pub struct Request {
    method: Slice,
    path: Slice,
    version: u8,
    headers: Vec<(Slice, Slice)>,
    data: BytesMut,
    body: BytesMut,
}

// Store start position and data length
type Slice = (usize, usize);

pub struct RequestHeaders<'req> {
    headers: slice::Iter<'req, (Slice, Slice)>,
    req: &'req Request,
}

impl Request {
    /// Returns the request method
    pub fn method(&self) -> &str {
        str::from_utf8(self.slice(&self.method)).unwrap()
    }

    /// Returns the requested path
    pub fn path(&self) -> &str {
        str::from_utf8(self.slice(&self.path)).unwrap()
    }

    /// Returns the requested HTTP Version
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Return our Request headers
    pub fn headers(&self) -> RequestHeaders {
        RequestHeaders {
            headers: self.headers.iter(),
            req: self,
        }
    }

    /// Get the request raw data wit a given length
    pub fn slice(&self, slice: &Slice) -> &[u8] {
        &self.data[slice.0..slice.1]
    }

    /// Get raw request
    pub fn data(&self) -> BytesMut {
        self.data.clone()
    }

    /// Get raw body
    pub fn body(&self) -> BytesMut {
        self.body.clone()
    }
}

/// Decode the given raw HTTP request
pub fn decode(buf: &mut BytesMut) -> io::Result<Option<Request>> {
    // Extract the details as slices
    let (method, path, version, headers, amt) = {
        // Parse with httparse
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut r = httparse::Request::new(&mut headers);
        let status = try!(r.parse(buf).map_err(|e| {
            let msg = format!("failed to parse http request: {:?}", e);
            io::Error::new(io::ErrorKind::Other, msg)
        }));

        // Get the request body data
        let amt = match status {
            httparse::Status::Complete(amt) => amt,
            httparse::Status::Partial => return Ok(None),
        };

        // Convert raw data to slice
        let toslice = |a: &[u8]| {
            let start = a.as_ptr() as usize - buf.as_ptr() as usize;
            assert!(start < buf.len());
            (start, start + a.len())
        };

        // Return the slices
        (toslice(r.method.unwrap().as_bytes()),
         toslice(r.path.unwrap().as_bytes()),
         r.version.unwrap(),
         // Convert headers to slices
         r.headers
             .iter()
             .map(|h| (toslice(h.name.as_bytes()), toslice(h.value)))
             .collect(),
         amt)
    };

    // Create a new Request object
    Ok(Request {
               method: method,
               path: path,
               version: version,
               headers: headers,
               data: buf.split_to(amt),
               body: buf.clone(),
           }
           .into())
}

impl<'req> Iterator for RequestHeaders<'req> {
    type Item = (&'req str, &'req [u8]);

    fn next(&mut self) -> Option<(&'req str, &'req [u8])> {
        self.headers
            .next()
            .map(|&(ref a, ref b)| {
                     let a = self.req.slice(a);
                     let b = self.req.slice(b);
                     (str::from_utf8(a).unwrap(), b)
                 })
    }
}
