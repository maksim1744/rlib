use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<Point> for (f64, f64) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn slen(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> f64 {
        self.slen().sqrt()
    }

    pub fn dp(&self, p: &Point) -> f64 {
        self.x * p.x + self.y * p.y
    }

    pub fn cp(&self, p: &Point) -> f64 {
        self.x * p.y - self.y * p.x
    }
}

macro_rules! impl_bin {
    ($trait:ident, $func:ident) => {
        impl $trait for Point {
            type Output = Point;

            fn $func(self, rhs: Self) -> Self::Output {
                Point::new(self.x.$func(rhs.x), self.y.$func(rhs.y))
            }
        }

        impl $trait<&Point> for Point {
            type Output = Point;

            fn $func(self, rhs: &Self) -> Self::Output {
                Point::new(self.x.$func(rhs.x), self.y.$func(rhs.y))
            }
        }

        impl $trait for &Point {
            type Output = Point;

            fn $func(self, rhs: Self) -> Self::Output {
                Point::new(self.x.$func(rhs.x), self.y.$func(rhs.y))
            }
        }

        impl $trait<Point> for &Point {
            type Output = Point;

            fn $func(self, rhs: Point) -> Self::Output {
                Point::new(self.x.$func(rhs.x), self.y.$func(rhs.y))
            }
        }
    };
}

impl_bin!(Add, add);
impl_bin!(Sub, sub);

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}
