#![feature(test)]
extern crate test;
extern crate bytes;

mod tests {
    use test::Bencher;
    use bytes::{BytesMut, BufMut};
    use std::fmt::{self, Write};
    use std::mem::transmute;

    #[bench]
    fn bench_push_to_buffer(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        b.iter(|| push(&mut buffer, &[72, 84, 84, 80, 47, 49, 46, 49, 32]));
    }

    #[bench]
    fn bench_extend_from_slice(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        b.iter(|| buffer.extend_from_slice(&[72, 84, 84, 80, 47, 49, 46, 49, 32]))
    }

    #[bench]
    fn bench_fastwrite(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        b.iter(|| write!(FastWrite(&mut buffer), "HTTP/1.1 ").unwrap() )
    }

    #[bench]
    fn bench_all_combined(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        b.iter(|| {
            push(&mut buffer, &[72, 84, 84]);
            buffer.extend_from_slice(&[80, 47, 49]);
            write!(FastWrite(&mut buffer), ".1 ").unwrap();
        });
    }

    #[bench]
    fn bench_convert_and_push(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        b.iter(|| {
            let length = "HTTP/1.1 200 OK".len() as u32;
            let data: [u8; 4] = unsafe { transmute(length.to_be()) };

            push(&mut buffer, &data);
        });
    }

    #[bench]
    fn bench_string_to_push(b : &mut Bencher) {
        let mut buffer = BytesMut::new();
        let data = String::from("200 OK");

        b.iter(|| {
            push(&mut buffer, data.as_bytes());
        });
    }

    fn push(buf: &mut BytesMut, data: &[u8]) {
        buf.reserve(data.len());
        unsafe {
            buf.bytes_mut()[..data.len()].copy_from_slice(data);
            buf.advance_mut(data.len());
        }
    }

    struct FastWrite<'a>(&'a mut BytesMut);

    impl<'a> fmt::Write for FastWrite<'a> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            push(&mut *self.0, s.as_bytes());
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
            fmt::write(self, args)
        }
    }
}
