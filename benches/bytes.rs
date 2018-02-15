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
            for _i in 0..1000 {
                buffer.reserve(data.len());
            }
        });
    }

    #[bench]
    fn bench_allocate_pile(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        let data = b"This is some data!";
        let mut length = 0 as usize;

        b.iter(|| {
            for _i in 0..1000 {
                length += data.len();
            }

            buffer.reserve(length);
        });
    }
}
