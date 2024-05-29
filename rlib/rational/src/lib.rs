use std::{
    cmp::*,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rlib_gcd::*;
use rlib_num_traits::*;

pub trait SignedInteger: Integer + Neg<Output = Self> {}
impl<T: Integer + Neg<Output = Self>> SignedInteger for T {}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Rational<T> {
    pub a: T,
    pub b: T,
}

impl<T: SignedInteger> Rational<T> {
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
                a: self.a.clone() / &self.b,
                b: T::ONE,
            }
        } else {
            Self {
                a: (self.a.clone() - &self.b + &T::ONE) / &self.b,
                b: T::ONE,
            }
        }
    }

    pub fn ceil(&self) -> Self {
        if self.a >= T::ZERO {
            Self {
                a: (self.a.clone() + &self.b - &T::ONE) / &self.b,
                b: T::ONE,
            }
        } else {
            Self {
                a: self.a.clone() / &self.b,
                b: T::ONE,
            }
        }
    }

    fn norm(&mut self) {
        let g = gcd(self.a.clone(), self.b.clone());
        self.a /= &g;
        self.b /= &g;
        if self.b < T::ZERO {
            // surely there is a better way
            let mut x = T::ZERO;
            std::mem::swap(&mut x, &mut self.b);
            self.b = -x;
            let mut x = T::ZERO;
            std::mem::swap(&mut x, &mut self.a);
            self.a = -x;
        }
    }
}

impl<T: ZeroOne> ZeroOne for Rational<T> {
    const ZERO: Self = Self { a: T::ZERO, b: T::ONE };
    const ONE: Self = Self { a: T::ONE, b: T::ONE };
}

impl<T: SignedInteger> Add<&Self> for Rational<T> {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        Self::new(self.a * &rhs.b + &(self.b.clone() * &rhs.a), self.b * &rhs.b)
    }
}
impl<T: SignedInteger> AddAssign<&Self> for Rational<T> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = self.clone() + rhs;
    }
}

impl<T: SignedInteger> Sub<&Self> for Rational<T> {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        Self::new(self.a * &rhs.b - &(self.b.clone() * &rhs.a), self.b * &rhs.b)
    }
}
impl<T: SignedInteger> SubAssign<&Self> for Rational<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = self.clone() - rhs;
    }
}

impl<T: SignedInteger> Mul<&Self> for Rational<T> {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        Self::new(self.a * &rhs.a, self.b * &rhs.b)
    }
}
impl<T: SignedInteger> MulAssign<&Self> for Rational<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = self.clone() * rhs;
    }
}

impl<T: SignedInteger> Div<&Self> for Rational<T> {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self {
        Self::new(self.a * &rhs.b, self.b * &rhs.a)
    }
}
impl<T: SignedInteger> DivAssign<&Self> for Rational<T> {
    fn div_assign(&mut self, rhs: &Self) {
        *self = self.clone() / rhs;
    }
}

impl<T: SignedInteger> Neg for Rational<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self { a: -self.a, b: self.b }
    }
}

macro_rules! impl_copy_op {
    ($op:ty, $func:tt) => {
        impl<T: SignedInteger + Copy> $op for Rational<T> {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self {
                (&self).$func(&rhs)
            }
        }
    };
}

impl_copy_op!(Add, add);
impl_copy_op!(Sub, sub);
impl_copy_op!(Mul, mul);
impl_copy_op!(Div, div);

macro_rules! impl_copy_assign_op {
    ($op:ty, $func:tt) => {
        impl<T: SignedInteger + Copy> $op for Rational<T> {
            fn $func(&mut self, rhs: Self) {
                self.$func(&rhs);
            }
        }
    };
}

impl_copy_assign_op!(AddAssign, add_assign);
impl_copy_assign_op!(SubAssign, sub_assign);
impl_copy_assign_op!(MulAssign, mul_assign);
impl_copy_assign_op!(DivAssign, div_assign);

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
        (self.clone() - rhs).a.cmp(&T::ZERO)
    }
}

impl<T: SignedInteger> PartialOrd for Rational<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
