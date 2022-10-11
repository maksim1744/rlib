use std::ops::*;

pub trait Randomable<T: Sized> {
    fn gen_from_u64(self, rng: u64) -> T;
}

macro_rules! implement_ranges {
    ($t:ty) => {
        impl Randomable<$t> for RangeTo<$t> {
            fn gen_from_u64(self, rng: u64) -> $t {
                (0..self.end).gen_from_u64(rng)
            }
        }
        impl Randomable<$t> for RangeInclusive<$t> {
            fn gen_from_u64(self, rng: u64) -> $t {
                if *self.start() != <$t>::MIN {
                    (*self.start() - 1..*self.end()).gen_from_u64(rng) + 1
                } else if *self.end() != <$t>::MAX {
                    (*self.start()..*self.end() + 1).gen_from_u64(rng)
                } else {
                    rng as $t
                }
            }
        }
        impl Randomable<$t> for RangeToInclusive<$t> {
            fn gen_from_u64(self, rng: u64) -> $t {
                (0..=self.end).gen_from_u64(rng)
            }
        }
    };
}

macro_rules! make_randomable {
    ($it:ty, $ut:ty) => {
        impl Randomable<$it> for Range<$it> {
            fn gen_from_u64(self, rng: u64) -> $it {
                assert!(!self.is_empty());
                let len = (self.end as $ut).wrapping_sub(self.start as $ut);
                ((rng % len as u64) as $ut).wrapping_add(self.start as $ut) as $it
            }
        }

        impl Randomable<$ut> for Range<$ut> {
            fn gen_from_u64(self, rng: u64) -> $ut {
                assert!(!self.is_empty());
                let len = self.end - self.start;
                (rng % len as u64) as $ut + self.start
            }
        }

        implement_ranges!($it);
        implement_ranges!($ut);
    };
}

make_randomable!(i8, u8);
make_randomable!(i16, u16);
make_randomable!(i32, u32);
make_randomable!(i64, u64);
make_randomable!(isize, usize);

impl Randomable<f64> for Range<f64> {
    fn gen_from_u64(self, rng: u64) -> f64 {
        assert!(!self.is_empty());
        let len = self.end - self.start;
        (rng as f64 / u64::MAX as f64) * len + self.start
    }
}
