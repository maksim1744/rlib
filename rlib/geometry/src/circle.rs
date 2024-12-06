use rlib_show::show_struct;

use crate::util::EPS;

use super::point::Point;

#[derive(Copy, Clone, Default, Debug)]
pub struct Circle {
    pub c: Point,
    pub r: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PointPosition {
    Inside,
    Border,
    Outside,
}

impl Circle {
    pub fn new(c: Point, r: f64) -> Self {
        Self { c, r }
    }

    pub fn position(&self, p: &Point) -> PointPosition {
        let d = ((self.c - p).len() - self.r) / self.r;
        if d < -EPS {
            PointPosition::Inside
        } else if d > EPS {
            PointPosition::Outside
        } else {
            PointPosition::Border
        }
    }
}

show_struct!(Circle, c, r);
