use std::cmp::*;
use std::fmt::{Debug, Display};
use std::ops::*;

pub trait ZeroOne {
    const ZERO: Self;
    const ONE: Self;
}

pub trait MinMax {
    const MIN: Self;
    const MAX: Self;
}

pub trait Integer:
    for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> DivAssign<&'a Self>
    + for<'a> Rem<&'a Self, Output = Self>
    + for<'a> RemAssign<&'a Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> MulAssign<&'a Self>
    + ZeroOne
    + MinMax
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Clone
    + Sized
    + Default
    + Display
    + Debug
{
    fn abs(&self) -> Self;
    fn into_abs(self) -> Self;
}

pub trait FixedSizeInteger:
    Integer
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Copy
{
    type Unsigned: Integer;
    type Signed: Integer;

    const BASE_10_LEN: usize;

    fn unsigned_abs(self) -> Self::Unsigned;
}

pub trait Float:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
    + ZeroOne
    + PartialEq
    + PartialOrd
    + Clone
    + Copy
    + Default
    + Sized
    + Display
    + Debug
{
    const PI: Self;

    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn abs(&self) -> Self;
    fn round(&self) -> Self;

    fn from_usize(x: usize) -> Self;
    fn from_i32(x: i32) -> Self;
    fn to_i64(&self) -> i64;
}

macro_rules! integer_common {
    ($it:ty, $ut:ty, $len:expr) => {
        type Unsigned = $ut;
        type Signed = $it;

        const BASE_10_LEN: usize = $len;
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

macro_rules! fixed_size_integer {
    ($it:ty, $ut:ty, $len:expr) => {
        impl Integer for $it {
            fn abs(&self) -> Self {
                <$it>::abs(*self)
            }
            fn into_abs(self) -> Self {
                <$it>::abs(self)
            }
        }

        impl FixedSizeInteger for $it {
            integer_common!($it, $ut, $len);

            fn unsigned_abs(self) -> Self::Unsigned {
                Self::unsigned_abs(self)
            }
        }

        impl Integer for $ut {
            fn abs(&self) -> Self {
                *self
            }
            fn into_abs(self) -> Self {
                self
            }
        }

        impl FixedSizeInteger for $ut {
            integer_common!($it, $ut, $len);

            fn unsigned_abs(self) -> Self::Unsigned {
                self
            }
        }
    };

    ($it:ty, $ut:ty) => {
        fixed_size_integer!($it, $ut, base_10_len!($ut));
    };
}

macro_rules! impl_zomm {
    ($($t:ty),*) => {
        $(
            impl ZeroOne for $t {
                const ZERO: $t = 0 as $t;
                const ONE: $t = 1 as $t;
            }

            impl MinMax for $t {
                const MIN: $t = <$t>::MIN;
                const MAX: $t = <$t>::MAX;
            }
        )*
    };
}

fixed_size_integer!(i8, u8);
fixed_size_integer!(i16, u16);
fixed_size_integer!(i32, u32);
fixed_size_integer!(i64, u64);
fixed_size_integer!(i128, u128);
fixed_size_integer!(isize, usize);

impl_zomm!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
impl_zomm!(f32, f64);

macro_rules! impl_float {
    ($t:ty, $pi:expr) => {
        impl Float for $t {
            const PI: Self = $pi;

            fn sin(&self) -> Self {
                <$t>::sin(*self)
            }
            fn cos(&self) -> Self {
                <$t>::cos(*self)
            }
            fn sqrt(&self) -> Self {
                <$t>::sqrt(*self)
            }
            fn abs(&self) -> Self {
                <$t>::abs(*self)
            }
            fn round(&self) -> Self {
                <$t>::round(*self)
            }

            fn from_usize(x: usize) -> Self {
                x as $t
            }
            fn from_i32(x: i32) -> Self {
                x as $t
            }
            fn to_i64(&self) -> i64 {
                self.round() as i64
            }
        }
    };
}

impl_float!(f32, std::f32::consts::PI);
impl_float!(f64, std::f64::consts::PI);
