use rlib_geometry::{
    circle::{Circle, PointPosition},
    line::Line,
    point::Point,
    util::{dist, intersect_cc, intersect_cl, intersect_ll, CircleIntersection, CircleLineIntersection, EPS},
};
use rlib_rand::{Rand, Rng};

const ITERS: usize = 1000;

fn assert_same(a: f64, b: f64) {
    assert!((a - b).abs() < EPS, "{a:.9} != {b:.9}");
}

#[test]
fn point() {
    let a = Point::new(3., 4.);
    let b = Point::new(5., -6.);

    assert_eq!(a.dp(&b), -9.);
    assert_eq!(a.cp(&b), -38.);
    assert_same(dist(&a, &b), 104f64.sqrt());
    assert_same(a.slen(), 25.);
    assert_same(a.len(), 5.);

    assert_eq!(a * -2., Point::new(-6., -8.));
    assert_eq!(a / -2., Point::new(-1.5, -2.));
    assert_eq!(a + b, Point::new(8., -2.));
    assert_eq!(a - b, Point::new(-2., 10.));

    assert_eq!(a + b, Point::new(8., -2.));
    assert_eq!(a + &b, Point::new(8., -2.));
    assert_eq!(&a + b, Point::new(8., -2.));
    assert_eq!(a + b, Point::new(8., -2.));
}

#[test]
fn line() {
    let a = Point::new(3., 4.);
    let b = Point::new(5., -6.);

    let l = Line::between(&a, &b);
    assert!(l.contains(&a));
    assert!(l.contains(&b));
    assert_same(l.dist(&Point::new(8., 5.)), 26f64.sqrt());

    let l2 = Line::new(2., -5., 7.);
    let p = intersect_ll(&l, &l2).unwrap();
    assert_same(p.x, 3.2592592592592595);
    assert_same(p.y, 2.7037037037037033);
}

#[test]
fn line_stress() {
    let mut rng = Rng::from_seed(42);
    for _ in 0..ITERS {
        let v = (0..4)
            .map(|_| Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64)))
            .collect::<Vec<_>>();
        let l1 = Line::between(&v[0], &v[1]);
        let l2 = Line::between(&v[2], &v[3]);
        // surely probability of lines being parallel is low enough that it won't happen
        let p = intersect_ll(&l1, &l2).unwrap();
        assert!(l1.contains(&p));
        assert!(l2.contains(&p));
    }
}

#[test]
fn circle() {
    let c = Circle::new(Point::new(3., 4.), 5.);

    assert_eq!(c.position(&Point::default()), PointPosition::Border);
    assert_eq!(c.position(&Point::new(6., 8.)), PointPosition::Border);
    assert_eq!(c.position(&Point::new(6., 7.)), PointPosition::Inside);
    assert_eq!(c.position(&Point::new(6., 9.)), PointPosition::Outside);
}

#[test]
fn circle_stress_touch() {
    let mut rng = Rng::from_seed(42);
    for _ in 0..ITERS {
        let p1 = Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64));
        let p2 = Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64));
        let r1 = rng.next(0f64..1000f64);
        let d = dist(&p1, &p2);
        let r2 = (r1 - d).abs();
        let c1 = Circle::new(p1, r1);
        let c2 = Circle::new(p2, r2);
        if r1 < d {
            match intersect_cc(&c1, &c2) {
                CircleIntersection::TouchOutside(p) => {
                    assert_eq!(c1.position(&p), PointPosition::Border);
                    assert_eq!(c2.position(&p), PointPosition::Border);
                }
                x => panic!("Must touch outside, instead: {:?}", x),
            }
        } else {
            match intersect_cc(&c1, &c2) {
                CircleIntersection::TouchInside(p) => {
                    assert_eq!(c1.position(&p), PointPosition::Border);
                    assert_eq!(c2.position(&p), PointPosition::Border);
                }
                x => panic!("Must touch inside, instead: {:?}", x),
            }
        }
    }
}

#[test]
fn circle_stress_intersect() {
    let mut rng = Rng::from_seed(42);
    for _ in 0..ITERS {
        let p1 = Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64));
        let p2 = Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64));
        let r1 = rng.next(0f64..1000f64);
        let d = dist(&p1, &p2);
        let r2 = rng.next((d - r1).abs()..(d + r1));
        let c1 = Circle::new(p1, r1);
        let c2 = Circle::new(p2, r2);
        match intersect_cc(&c1, &c2) {
            CircleIntersection::Intersect(p1, p2) => {
                assert_eq!(c1.position(&p1), PointPosition::Border);
                assert_eq!(c2.position(&p1), PointPosition::Border);
                assert_eq!(c1.position(&p2), PointPosition::Border);
                assert_eq!(c2.position(&p2), PointPosition::Border);
            }
            x => panic!("Must intersect, instead: {:?}", x),
        }
    }
}

#[test]
fn circle_stress() {
    let mut rng = Rng::from_seed(42);
    for _ in 0..ITERS {
        let c1 = Circle::new(
            Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64)),
            rng.next(0f64..1000f64),
        );
        let c2 = Circle::new(
            Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64)),
            rng.next(0f64..1000f64),
        );

        for p in intersect_cc(&c1, &c2).into_iter() {
            assert_eq!(c1.position(&p), PointPosition::Border);
            assert_eq!(c2.position(&p), PointPosition::Border);
        }
    }
}

#[test]
fn circle_line_stress() {
    let mut rng = Rng::from_seed(42);
    for _ in 0..ITERS {
        let c = Circle::new(
            Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64)),
            rng.next(0f64..1000f64),
        );
        let l = Line::between(
            &Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64)),
            &Point::new(rng.next(-1000f64..1000f64), rng.next(-1000f64..1000f64)),
        );

        match intersect_cl(&c, &l) {
            CircleLineIntersection::None => {
                assert!(l.dist(&c.c) >= c.r);
            }
            CircleLineIntersection::Touch(p) => {
                assert!(l.contains(&p));
                assert_eq!(c.position(&p), PointPosition::Border);
                assert!((l.dist(&c.c) - c.r).abs() < EPS);
            }
            CircleLineIntersection::Intersect(p1, p2) => {
                assert!(l.dist(&c.c) <= c.r);
                assert!(l.contains(&p1));
                assert_eq!(c.position(&p1), PointPosition::Border);
                assert!(l.contains(&p2));
                assert_eq!(c.position(&p2), PointPosition::Border);
            }
        }
    }
}
