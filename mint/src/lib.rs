use std::ops::*;

use rlib_io::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Modular<const M: i32> {
    v: i32,
}

impl<const M: i32> Modular<M> {
    pub const ZERO: Self = Self { v: 0 };
    pub const ONE: Self = Self { v: 1 };

    pub fn new(v: i64) -> Self {
        let mut v = (v % M as i64) as i32;
        if v < 0 {
            v += M;
        }
        Self { v }
    }

    pub fn inv(&self) -> Self {
        let mut a = self.v;
        let mut b = M;
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

    pub fn md() -> i32 {
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
}

impl<const M: i32> Add for Modular<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut v = self.v + rhs.v;
        if v >= M {
            v -= M;
        }
        Self { v }
    }
}
impl<const M: i32> AddAssign for Modular<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const M: i32> Sub for Modular<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut v = self.v - rhs.v;
        if v < 0 {
            v += M;
        }
        Self { v }
    }
}
impl<const M: i32> SubAssign for Modular<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const M: i32> Mul for Modular<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.v as i64 * rhs.v as i64)
    }
}
impl<const M: i32> MulAssign for Modular<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const M: i32> Div for Modular<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}
impl<const M: i32> DivAssign for Modular<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const M: i32> Neg for Modular<M> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut v = -self.v;
        if v < 0 {
            v += M;
        }
        Self { v }
    }
}

impl<const M: i32> Readable for Modular<M> {
    fn read(reader: &mut Reader) -> Self {
        Self::new(reader.read())
    }
}
impl<const M: i32> Writable for Modular<M> {
    fn write(&self, writer: &mut Writer) {
        self.v.write(writer)
    }
}
impl<const M: i32> std::fmt::Display for Modular<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.v.fmt(f)
    }
}
impl<const M: i32> std::fmt::Debug for Modular<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.v.fmt(f)
    }
}

pub type Mint998 = Modular<998244353>;
pub type Mint107 = Modular<1000000007>;
