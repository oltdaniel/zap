#![feature(test)]
extern crate test;
extern crate bytes;

mod tests {
    use test::Bencher;
    use bytes::{BytesMut, BufMut};

    #[bench]
    fn bench_allocate_pieces(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        let data = b"This is some data!";

        b.iter(|| {
            for _i in 0..100 {
                push(&mut buffer, data);
            }
        });
    }

    #[bench]
    fn bench_allocate_pile(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        let data = b"This is some data!";
        let mut length = 0 as usize;

        b.iter(|| {
            length = data.len() * 100;
            buffer.reserve(length);

            for _i in 0..100 {
                unsafe {
                    buffer.bytes_mut()[..data.len()].copy_from_slice(data.as_ref());
                }
            }

            unsafe {
                buffer.advance_mut(length);
            }
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
