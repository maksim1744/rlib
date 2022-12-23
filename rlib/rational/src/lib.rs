use std::cmp::*;
use std::ops::*;

use rlib_gcd::*;
use rlib_integer::*;

pub trait SignedInteger: Integer + std::ops::Neg<Output = Self> + Copy {}
impl<T: Integer + std::ops::Neg<Output = Self> + Copy> SignedInteger for T {}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Rational<T> {
    pub a: T,
    pub b: T,
}

impl<T: SignedInteger> Rational<T> {
    pub const ZERO: Self = Self { a: T::ZERO, b: T::ONE };
    pub const ONE: Self = Self { a: T::ONE, b: T::ONE };

    pub fn new(a: T, b: T) -> Self {
        let mut r = Self { a, b };
        r.norm();
        r
    }

    pub fn new_int(a: T) -> Self {
        Self { a, b: T::ONE }
    }

    pub fn floor(&self) -> Self {
        if self.a >= T::ZERO {
            Self {
                a: self.a / self.b,
                b: T::ONE,
            }
        } else {
            Self {
                a: (self.a - self.b + T::ONE) / self.b,
                b: T::ONE,
            }
        }
    }

    pub fn ceil(&self) -> Self {
        if self.a >= T::ZERO {
            Self {
                a: (self.a + self.b - T::ONE) / self.b,
                b: T::ONE,
            }
        } else {
            Self {
                a: self.a / self.b,
                b: T::ONE,
            }
        }
    }

    fn norm(&mut self) {
        let g = gcd(self.a, self.b);
        self.a /= g;
        self.b /= g;
        if self.b < T::ZERO {
            self.b = -self.b;
            self.a = -self.a;
        }
    }
}

impl<T: SignedInteger> Add for Rational<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.a * rhs.b + self.b * rhs.a, self.b * rhs.b)
    }
}
impl<T: SignedInteger> AddAssign for Rational<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: SignedInteger> Sub for Rational<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.a * rhs.b - self.b * rhs.a, self.b * rhs.b)
    }
}
impl<T: SignedInteger> SubAssign for Rational<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T: SignedInteger> Mul for Rational<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.a * rhs.a, self.b * rhs.b)
    }
}
impl<T: SignedInteger> MulAssign for Rational<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: SignedInteger> Div for Rational<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::new(self.a * rhs.b, self.b * rhs.a)
    }
}
impl<T: SignedInteger> DivAssign for Rational<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: SignedInteger + std::fmt::Display> std::fmt::Display for Rational<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.a, self.b)
    }
}
impl<T: SignedInteger + std::fmt::Debug> std::fmt::Debug for Rational<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}/{:?}", self.a, self.b)
    }
}

impl<T: SignedInteger> Ord for Rational<T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        (*self - *rhs).a.cmp(&T::ZERO)
    }
}

impl<T: SignedInteger> PartialOrd for Rational<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        (*self - *rhs).a.partial_cmp(&T::ZERO)
    }
}
