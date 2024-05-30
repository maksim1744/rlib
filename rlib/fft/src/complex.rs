use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rlib_num_traits::{Float, ZeroOne};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Complex<F: Float> {
    pub x: F,
    pub y: F,
}

impl<F: Float> Complex<F> {
    pub const I: Self = Complex::new(F::ZERO, F::ONE);

    pub const fn new(x: F, y: F) -> Self {
        Self { x, y }
    }

    pub const fn new_real(x: F) -> Self {
        Self { x, y: F::ZERO }
    }

    pub fn abs2(&self) -> F {
        self.x * self.x + self.y * self.y
    }

    pub fn abs(&self) -> F {
        self.abs2().sqrt()
    }

    pub fn conj(&self) -> Self {
        Self::new(self.x, -self.y)
    }
}

impl<F: Float> Add for Complex<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl<F: Float> AddAssign for Complex<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<F: Float> Sub for Complex<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl<F: Float> SubAssign for Complex<F> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<F: Float> Mul<F> for Complex<F> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
impl<F: Float> MulAssign<F> for Complex<F> {
    fn mul_assign(&mut self, rhs: F) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<F: Float> Mul for Complex<F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x - self.y * rhs.y, self.x * rhs.y + self.y * rhs.x)
    }
}
impl<F: Float> MulAssign for Complex<F> {
    fn mul_assign(&mut self, rhs: Self) {
        let x = self.x * rhs.x - self.y * rhs.y;
        let y = self.x * rhs.y + self.y * rhs.x;
        self.x = x;
        self.y = y;
    }
}

impl<F: Float> Div<F> for Complex<F> {
    type Output = Self;
    fn div(self, rhs: F) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
impl<F: Float> DivAssign<F> for Complex<F> {
    fn div_assign(&mut self, rhs: F) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<F: Float> Div for Complex<F> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.conj() / rhs.abs2()
    }
}
impl<F: Float> DivAssign for Complex<F> {
    fn div_assign(&mut self, rhs: Self) {
        let res = *self / rhs;
        self.x = res.x;
        self.y = res.y;
    }
}

impl<F: Float> Neg for Complex<F> {
    type Output = Self;
    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
    }
}

impl<F: Float> ZeroOne for Complex<F> {
    const ZERO: Self = Self::new_real(F::ZERO);
    const ONE: Self = Self::new_real(F::ONE);
}
