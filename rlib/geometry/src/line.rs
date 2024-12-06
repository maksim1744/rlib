use rlib_show::show_struct;

use crate::util::EPS;

use super::point::Point;

#[derive(Copy, Clone, Default, Debug)]
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Line {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        let d = Point::new(a, b).len();
        Self {
            a: a / d,
            b: b / d,
            c: c / d,
        }
    }

    pub fn between(u: &Point, v: &Point) -> Self {
        let a = u.y - v.y;
        let b = v.x - u.x;
        let c = -(a * u.x + b * u.y);
        Self::new(a, b, c)
    }

    pub fn dist(&self, p: &Point) -> f64 {
        (self.a * p.x + self.b * p.y + self.c).abs()
    }

    pub fn contains(&self, p: &Point) -> bool {
        self.dist(p) < EPS
    }

    pub fn ort(&self) -> Point {
        Point::new(self.a, self.b)
    }
}

show_struct!(Line, a, b, c);
