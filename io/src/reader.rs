use std::io::{Read, Stdin, StdinLock};

pub struct Reader {
    buf: [u8; Reader::BUF_SIZE],
    begin: usize,
    end: usize,
    stdin: StdinLock<'static>,
    eof: bool,
}

impl Reader {
    const BUF_SIZE: usize = 1 << 16;

    pub fn new(stdin: &Stdin) -> Self {
        Self {
            buf: [0; Reader::BUF_SIZE],
            begin: 0,
            end: 0,
            stdin: stdin.lock(),
            eof: false,
        }
    }

    pub fn read<T: Readable>(&mut self) -> T {
        T::read(self)
    }

    pub fn read_vec<T: Readable>(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::<T>::with_capacity(n);
        for _ in 0..n {
            result.push(self.read());
        }
        result
    }

    fn refill(&mut self) {
        if self.eof {
            return;
        }

        if self.begin != 0 {
            self.buf.copy_within(self.begin..self.end, 0);
            self.end -= self.begin;
            self.begin = 0;
        }

        let bytes = self.stdin.read(&mut self.buf[self.end..]).unwrap();
        if bytes == 0 {
            self.eof = true;
        }
        self.end += bytes;
    }

    fn skip_whitespace(&mut self) {
        while {
            if self.begin == self.end {
                self.refill();
            }
            !self.eof && self.peek().is_ascii_whitespace()
        } {
            self.begin += 1;
            if self.begin == self.end {
                self.refill();
            }
        }
    }

    fn peek(&mut self) -> u8 {
        if self.begin == self.end {
            self.refill();
        }
        self.buf[self.begin]
    }
}

pub trait Readable {
    fn read(reader: &mut Reader) -> Self;
}

impl Readable for String {
    fn read(reader: &mut Reader) -> Self {
        reader.skip_whitespace();
        let mut result = String::new();
        while {
            if reader.begin == reader.end {
                reader.refill();
            }
            !reader.eof && !reader.peek().is_ascii_whitespace()
        } {
            result.push(reader.peek() as char);
            reader.begin += 1;
        }
        result
    }
}

macro_rules! read_signed {
    ($t:ty) => {
        impl Readable for $t {
            fn read(reader: &mut Reader) -> Self {
                reader.skip_whitespace();
                let mut result: $t = 0;
                if reader.peek() == b'-' {
                    reader.begin += 1;
                    while {
                        if reader.begin == reader.end {
                            reader.refill();
                        }
                        !reader.eof && !reader.peek().is_ascii_whitespace()
                    } {
                        result = result * 10 - (reader.buf[reader.begin] - ('0' as u8)) as $t;
                        reader.begin += 1;
                    }
                } else {
                    while {
                        if reader.begin == reader.end {
                            reader.refill();
                        }
                        !reader.eof && !reader.peek().is_ascii_whitespace()
                    } {
                        result = result * 10 + (reader.buf[reader.begin] - ('0' as u8)) as $t;
                        reader.begin += 1;
                    }
                }
                result
            }
        }
    };
}

macro_rules! read_unsigned {
    ($t:ty) => {
        impl Readable for $t {
            fn read(reader: &mut Reader) -> Self {
                reader.skip_whitespace();
                let mut result: $t = 0;
                while {
                    if reader.begin == reader.end {
                        reader.refill();
                    }
                    !reader.eof && !reader.peek().is_ascii_whitespace()
                } {
                    result = result * 10 + (reader.buf[reader.begin] - ('0' as u8)) as $t;
                    reader.begin += 1;
                }
                result
            }
        }
    };
}

read_signed!(i8);
read_signed!(i16);
read_signed!(i32);
read_signed!(i64);
read_signed!(i128);
read_signed!(isize);

read_unsigned!(u8);
read_unsigned!(u16);
read_unsigned!(u32);
read_unsigned!(u64);
read_unsigned!(u128);
read_unsigned!(usize);

macro_rules! read_tuple {
    ($($t:ident),*) => {
        impl<$($t,)*> Readable for ($($t,)*) where $($t: Readable,)* {
            fn read(reader: &mut Reader) -> Self {
                ($($t::read(reader)),*)
            }
        }
    }
}

read_tuple!(A, B);
read_tuple!(A, B, C);
read_tuple!(A, B, C, D);
read_tuple!(A, B, C, D, E);
read_tuple!(A, B, C, D, E, F);
read_tuple!(A, B, C, D, E, F, G);
read_tuple!(A, B, C, D, E, F, G, H);