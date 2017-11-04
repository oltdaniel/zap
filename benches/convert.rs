#![feature(test)]
extern crate test;
extern crate bytes;

mod tests {
    use test::Bencher;
    use bytes::{BytesMut, BufMut};

    #[bench]
    fn bench_usize_to_ascii(b : &mut Bencher) {
        let mut buffer = BytesMut::new();
        let s : usize = 12345;

        b.iter(|| {
            let mut data : BytesMut = BytesMut::new();
            let mut length = s as u32;

            loop {
                let c = 48 + (&length % 10 as u32) as u8;
                data.put_u8(c);
                length = (&length / 10 as u32) as u32;

                if length == 0 {
                    break;
                }
            }

            data.reverse();

            push(&mut buffer, data.as_ref());
        });
    }

    #[bench]
    fn bench_usize_to_string(b : &mut Bencher) {
        let mut buffer = BytesMut::new();
        let s : usize = 12345;

        b.iter(|| {
            push(&mut buffer, s.to_string().as_bytes());
        })
    }

    #[bench]
    fn bench_limited_usize_to_ascii(b : &mut Bencher) {
        let mut buffer = BytesMut::new();

        b.iter(|| {
            let mut data : [u8; 6] = [0; 6];
            let mut length : usize = 123456;

            for i in 0..5 {
                data[5 - i] = 48 + (&length % 10) as u8;
                length = (&length / 10) as usize;

                if length <= 9 {
                    data[4 - i] = 48 + length as u8;
                    push(&mut buffer, &data[(4 - i)..6]);
                    break;
                }
            }
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
