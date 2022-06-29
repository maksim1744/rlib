pub trait Integer {
    type Unsigned;
    type Signed;

    const BASE_10_LEN: usize;

    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;

    fn unsigned_abs(self) -> Self::Unsigned;
}

macro_rules! integer_common {
    ($it:ty, $ut:ty, $len:expr) => {
        type Unsigned = $ut;
        type Signed = $it;

        const BASE_10_LEN: usize = $len;

        const ZERO: Self = 0;
        const ONE: Self = 1;
        const TWO: Self = 2;
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
                self.overflowing_abs().0 as Self::Unsigned
            }
        }

        impl Integer for $ut {
            integer_common!($it, $ut, $len);

            fn unsigned_abs(self) -> Self::Unsigned {
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
