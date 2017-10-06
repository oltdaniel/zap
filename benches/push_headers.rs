#![feature(test)]
extern crate test;
extern crate bytes;

mod tests {
    use test::Bencher;
    use bytes::{BytesMut, BufMut};

    #[bench]
    fn bench_vec_to_bytes_out(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        b.iter(|| {
            let mut headers : Vec<(String, String)> = Vec::new();

            headers.push(("Content-Type".to_string(), "text/plain".to_string()));
            headers.push(("X-Access-Token".to_string(), "XQB6h4MkjhJT9Mqde5kkdLupYU8MrL6d".to_string()));

            for &(ref n, ref v) in &headers {
                push(&mut buffer, n.as_bytes());
                push(&mut buffer, b": ");
                push(&mut buffer, v.as_bytes());
                push(&mut buffer, b"\r\n");
            }
        });
    }

    #[bench]
    fn bench_bytes_to_bytes_out(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        b.iter(|| {
            let mut name = "Content-Type";
            let mut value = "text/plain";

            push(&mut buffer, name.as_bytes());
            push(&mut buffer, b": ");
            push(&mut buffer, value.as_bytes());
            push(&mut buffer, b"\r\n");

            name = "X-Access-Token";
            value = "XQB6h4MkjhJT9Mqde5kkdLupYU8MrL6d";

            push(&mut buffer, name.as_bytes());
            push(&mut buffer, b": ");
            push(&mut buffer, value.as_bytes());
            push(&mut buffer, b"\r\n");
        });
    }

    fn push(buf: &mut BytesMut, data: &[u8]) {
        buf.reserve(data.len());
        unsafe {
            buf.bytes_mut()[..data.len()].copy_from_slice(data);
            buf.advance_mut(data.len());
        }
    }
}
