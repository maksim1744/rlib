use std::cmp::*;
use std::ops::*;

pub trait Integer:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Mul<Output = Self>
    + MulAssign
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Copy
    + Clone
    + Sized
    + Default
{
    type Unsigned: Integer;
    type Signed: Integer;

    const BASE_10_LEN: usize;

    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;

    const MIN: Self;
    const MAX: Self;

    fn unsigned_abs(self) -> Self::Unsigned;
    fn abs(self) -> Self;
}

macro_rules! integer_common {
    ($it:ty, $ut:ty, $len:expr) => {
        type Unsigned = $ut;
        type Signed = $it;

        const BASE_10_LEN: usize = $len;

        const ZERO: Self = 0;
        const ONE: Self = 1;
        const TWO: Self = 2;

        const MIN: Self = Self::MIN;
        const MAX: Self = Self::MAX;
    };
}

macro_rules! base_10_len {
    ($ut:ty) => {{
        let mut value = <$ut>::MAX;
        let mut ans: usize = 0;
        while value != 0 {
            value /= 10;
            ans += 1;
        }
        ans
    }};
}

macro_rules! integer {
    ($it:ty, $ut:ty, $len:expr) => {
        impl Integer for $it {
            integer_common!($it, $ut, $len);

            fn unsigned_abs(self) -> Self::Unsigned {
                Self::unsigned_abs(self)
            }
            fn abs(self) -> Self {
                self.abs()
            }
        }

        impl Integer for $ut {
            integer_common!($it, $ut, $len);

            fn unsigned_abs(self) -> Self::Unsigned {
                self
            }
            fn abs(self) -> Self {
                self
            }
        }
    };

    ($it:ty, $ut:ty) => {
        integer!($it, $ut, base_10_len!($ut));
    };
}

integer!(i8, u8);
integer!(i16, u16);
integer!(i32, u32);
integer!(i64, u64);
integer!(i128, u128);
integer!(isize, usize);
