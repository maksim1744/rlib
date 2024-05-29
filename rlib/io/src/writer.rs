use std::io::Write;

use rlib_num_traits::FixedSizeInteger;

pub struct Writer<'a> {
    buf: [u8; Writer::BUF_SIZE],
    end: usize,
    stdout: Box<dyn Write + 'a>,
}

impl<'a> Writer<'a> {
    const BUF_SIZE: usize = 1 << 16;

    pub fn new(stdout: Box<dyn Write + 'a>) -> Self {
        Self {
            buf: [0; Writer::BUF_SIZE],
            end: 0,
            stdout,
        }
    }

    pub fn write<T: Writable>(&mut self, t: &T) {
        t.write(self);
        #[cfg(debug_assertions)]
        self.flush();
    }

    pub fn write_char(&mut self, c: char) {
        self.write_bytes(&[c as u8]);
        #[cfg(debug_assertions)]
        self.flush();
    }

    pub fn flush(&mut self) {
        if self.end == 0 {
            return;
        }

        self.stdout.write_all(&self.buf[..self.end]).unwrap();
        self.end = 0;
    }

    fn reserve(&mut self, size: usize) {
        if self.end + size > self.buf.len() {
            self.flush();
        }
    }

    fn write_bytes(&mut self, buf: &[u8]) {
        self.reserve(buf.len());
        self.buf[self.end..self.end + buf.len()].copy_from_slice(buf);
        self.end += buf.len();
    }
}

impl Drop for Writer<'_> {
    fn drop(&mut self) {
        self.flush();
    }
}

pub trait Writable {
    fn write(&self, writer: &mut Writer);
}

impl Writable for &str {
    fn write(&self, writer: &mut Writer) {
        for chunk in self.as_bytes().chunks(Writer::BUF_SIZE) {
            writer.write_bytes(chunk);
        }
    }
}

impl Writable for String {
    fn write(&self, writer: &mut Writer) {
        for chunk in self.as_bytes().chunks(Writer::BUF_SIZE) {
            writer.write_bytes(chunk);
        }
    }
}

impl<T: Writable> Writable for Vec<T> {
    fn write(&self, writer: &mut Writer) {
        for (i, value) in self.iter().enumerate() {
            if i != 0 {
                writer.write_char(' ');
            }
            writer.write(value);
        }
    }
}

macro_rules! write_unsigned {
    ($t:ty) => {
        impl Writable for $t {
            fn write(&self, writer: &mut Writer) {
                if self == &0 {
                    writer.write_char('0');
                    return;
                }

                let mut buf = [0; <$t as FixedSizeInteger>::BASE_10_LEN];
                let mut index = buf.len();
                let mut value = *self;
                while value != 0 {
                    index -= 1;
                    buf[index] = (value % 10) as u8 + b'0';
                    value /= 10;
                }
                writer.write_bytes(&buf[index..]);
            }
        }
    };
}

macro_rules! write_signed {
    ($t:ty) => {
        impl Writable for $t {
            fn write(&self, writer: &mut Writer) {
                if self < &0 {
                    writer.write_char('-');
                }
                writer.write(&self.unsigned_abs());
            }
        }
    };
}

write_signed!(i8);
write_signed!(i16);
write_signed!(i32);
write_signed!(i64);
write_signed!(i128);
write_signed!(isize);

write_unsigned!(u8);
write_unsigned!(u16);
write_unsigned!(u32);
write_unsigned!(u64);
write_unsigned!(u128);
write_unsigned!(usize);

macro_rules! write_tuple {
    ($t1:ident, $($t:ident),*) => {
        impl<$t1: Writable, $($t: Writable,)*> Writable for ($t1, $($t,)*) {
            fn write(&self, writer: &mut Writer) {
                #[allow(non_snake_case)]
                let ($t1, $($t,)*) = self;
                writer.write($t1);
                $(
                    writer.write_char(' ');
                    writer.write($t);
                )*
            }
        }
    }
}

write_tuple!(A, B);
write_tuple!(A, B, C);
write_tuple!(A, B, C, D);
write_tuple!(A, B, C, D, E);
write_tuple!(A, B, C, D, E, F);
write_tuple!(A, B, C, D, E, F, G);
write_tuple!(A, B, C, D, E, F, G, H);
