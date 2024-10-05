use std::mem::swap;

use super::{circle::Circle, line::Line, point::Point};

pub const EPS: f64 = 1e-9;

pub fn dist(a: &Point, b: &Point) -> f64 {
    (a - b).len()
}

pub fn parallel(a: &Line, b: &Line) -> bool {
    a.ort().cp(&b.ort()).abs() < EPS
}

pub fn intersect_ll(u: &Line, v: &Line) -> Option<Point> {
    if parallel(u, v) {
        return None;
    }
    // u.a * x + u.b * y + u.c == 0
    // v.a * x + v.b * y + v.c == 0
    let x = -(u.c * v.b - u.b * v.c) / (u.a * v.b - u.b * v.a);
    let y = -(u.c * v.a - u.a * v.c) / (u.b * v.a - u.a * v.b);
    Some(Point::new(x, y))
}

pub enum CircleLineIntersection {
    None,
    Touch(Point),
    Intersect(Point, Point),
}

impl IntoIterator for CircleLineIntersection {
    type Item = Point;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<Point>, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            CircleLineIntersection::None => [None, None],
            CircleLineIntersection::Touch(p) => [Some(p), None],
            CircleLineIntersection::Intersect(a, b) => [Some(a), Some(b)],
        }
        .into_iter()
        .flatten()
    }
}

pub fn intersect_cl(c: &Circle, l: &Line) -> CircleLineIntersection {
    let d = l.dist(&c.c);
    if d > c.r + EPS {
        CircleLineIntersection::None
    } else if d > c.r - EPS {
        let ort = Point::new(l.a, l.b);
        let ort = ort / ort.len();
        CircleLineIntersection::Touch(ort * c.r)
    } else {
        let mut ort = Point::new(l.a, l.b);
        if ort.len() != 0.0 {
            ort = ort / ort.len();
        }
        if l.a * c.c.x + l.b * c.c.y + l.c > 0.0 {
            ort = ort * -1.0;
        }
        let par = Point::new(-ort.y, ort.x);
        let ort = ort * d;
        let side = (c.r * c.r - d * d).max(0.0).sqrt();
        CircleLineIntersection::Intersect(c.c + ort + par * side, c.c + ort - par * side)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CircleIntersection {
    None,
    Same,
    TouchInside(Point),
    TouchOutside(Point),
    Intersect(Point, Point),
}

impl IntoIterator for CircleIntersection {
    type Item = Point;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<Point>, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            CircleIntersection::None | CircleIntersection::Same => [None, None],
            CircleIntersection::TouchInside(p) | CircleIntersection::TouchOutside(p) => [Some(p), None],
            CircleIntersection::Intersect(a, b) => [Some(a), Some(b)],
        }
        .into_iter()
        .flatten()
    }
}

pub fn intersect_cc<'a>(mut a: &'a Circle, mut b: &'a Circle) -> CircleIntersection {
    if a.r < b.r {
        swap(&mut a, &mut b);
    }
    let d = dist(&a.c, &b.c);
    if d < EPS && a.r < b.r + EPS {
        return CircleIntersection::Same;
    }
    if d < a.r - b.r - EPS {
        CircleIntersection::None
    } else if d < a.r - b.r + EPS {
        CircleIntersection::TouchInside(a.c + (b.c - a.c) / d * a.r)
    } else if d < a.r + b.r - EPS {
        let line = Line::new(
            -a.c.x * 2.0 + b.c.x * 2.0,
            -a.c.y * 2.0 + b.c.y * 2.0,
            a.c.x.powi(2) + a.c.y.powi(2) - b.c.x.powi(2) - b.c.y.powi(2) - a.r.powi(2) + b.r.powi(2),
        );
        match intersect_cl(a, &line) {
            CircleLineIntersection::None => CircleIntersection::None,
            CircleLineIntersection::Touch(p) => CircleIntersection::TouchOutside(p),
            CircleLineIntersection::Intersect(u, v) => CircleIntersection::Intersect(u, v),
        }
    } else if d < a.r + b.r + EPS {
        CircleIntersection::TouchOutside(a.c + (b.c - a.c) / d * a.r)
    } else {
        CircleIntersection::None
    }
}
