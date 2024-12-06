use std::ops::*;

use rlib_io::*;
use rlib_show::{Show, ShowSettings};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Modular<const M: u32> {
    v: u32,
}

impl<const M: u32> Modular<M> {
    pub const ZERO: Self = Self { v: 0 };
    pub const ONE: Self = Self { v: 1 };

    pub fn new(v: i64) -> Self {
        let mut v = (v % M as i64) as i32;
        if v < 0 {
            v += M as i32;
        }
        Self { v: v as u32 }
    }

    pub fn inv(&self) -> Self {
        let mut a = self.v as i32;
        let mut b = M as i32;
        let mut x = 0;
        let mut y = 1;
        while a != 0 {
            let k = b / a;
            b -= k * a;
            x -= k * y;
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut x, &mut y);
        }
        Self::new(x as i64)
    }

    pub fn md() -> u32 {
        M
    }

    pub fn pow(&self, mut d: u64) -> Self {
        let mut res = Self::ONE;
        let mut a = *self;
        while d != 0 {
            if d % 2 == 1 {
                res *= a;
            }
            a *= a;
            d /= 2;
        }
        res
    }

    pub fn inner(&self) -> u32 {
        self.v
    }
}

impl<const M: u32> Add for Modular<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut v = self.v + rhs.v;
        if v >= M {
            v -= M;
        }
        Self { v }
    }
}
impl<const M: u32> AddAssign for Modular<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const M: u32> Sub for Modular<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut v = self.v + Self::md() - rhs.v;
        if v >= M {
            v -= M;
        }
        Self { v }
    }
}
impl<const M: u32> SubAssign for Modular<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const M: u32> Mul for Modular<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.v as i64 * rhs.v as i64)
    }
}
impl<const M: u32> MulAssign for Modular<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<const M: u32> Div for Modular<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}
impl<const M: u32> DivAssign for Modular<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const M: u32> Neg for Modular<M> {
    type Output = Self;
    fn neg(self) -> Self {
        if self.v == 0 {
            self
        } else {
            Self { v: Self::md() - self.v }
        }
    }
}

impl<const M: u32> Readable for Modular<M> {
    fn read(reader: &mut Reader) -> Self {
        Self::new(reader.read())
    }
}
impl<const M: u32> Writable for Modular<M> {
    fn write(&self, writer: &mut Writer) {
        self.v.write(writer)
    }
}
impl<const M: u32> std::fmt::Display for Modular<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.v.fmt(f)
    }
}
impl<const M: u32> std::fmt::Debug for Modular<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.v.fmt(f)
    }
}

pub type Mint998 = Modular<998244353>;
pub type Mint107 = Modular<1000000007>;

impl<const M: u32> Show for Modular<M> {
    fn show(&self, settings: &ShowSettings) -> String {
        let max_denominator = if settings.mint_rational {
            settings.mint_max.min(Self::md() as i64 - 1)
        } else {
            1
        };
        for denominator in 1..=max_denominator {
            for numerator in -settings.mint_max..=settings.mint_max {
                if Self::new(numerator) / Self::new(denominator) == *self {
                    if denominator == 1 {
                        return numerator.to_string();
                    } else {
                        return format!("{}/{}", numerator, denominator);
                    }
                }
            }
        }

        if settings.mint_max == 0 {
            self.inner().to_string()
        } else {
            format!("?{}", self.inner())
        }
    }
}
