#![feature(test)]
extern crate test;
extern crate bytes;

mod tests {
    use test::Bencher;
    use bytes::{BytesMut, BufMut};

    pub struct Request {
        first: BytesMut,
        head: BytesMut,
        body: BytesMut,
    }

    #[bench]
    fn bench_parse_request(b : &mut Bencher) {
        let mut buffer = BytesMut::new();
        push(&mut buffer, b"GET / HTTP/1.1\nContent-Type: text/html\nContent-Length: 120\n\nHello World\n");

        b.iter(|| {
            // Clone buffer for iter
            let iter = buffer.clone();

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
            let first = buffer.split_to(firstc as usize);
            let head = buffer.split_to(headc as usize);
            let body = buffer.clone();

            // Clear buffer for next request
            buffer.clear();

            // Build request object
            let request = Request {
                first: first,
                head: head,
                body: body,
            };
        });
    }

    fn push(buf : &mut BytesMut, data : &[u8]) {
        buf.reserve(data.len());

        unsafe {
            buf.bytes_mut()[..data.len()].copy_from_slice(data);
            buf.advance_mut(data.len());
        }
    }
}
