use rlib_rational::*;

type R = Rational<i32>;

#[test]
fn simple() {
    assert_eq!(R::new(3, 4), R::new(6, 8));
    assert_eq!(R::new(3, 4), R::new(-6, -8));
    assert_eq!(R::new(-3, 4), R::new(6, -8));
    assert_eq!(R::new(0, -42), R::new(0, 1));

    assert_eq!(R::new(7, 10) + R::new(5, 6), R::new(23, 15));
    assert_eq!(R::new(7, 10) - R::new(5, 6), R::new(-2, 15));
    assert_eq!(R::new(7, 10) * R::new(-5, 6), R::new(-7, 12));
    assert_eq!(R::new(7, 10) / R::new(-5, 6), R::new(-21, 25));

    let mut a = R::new(7, 10);
    let b = R::new(5, 6);
    a += b;
    assert_eq!(a, R::new(23, 15));

    let mut a = R::new(7, 10);
    let b = R::new(5, 6);
    a -= b;
    assert_eq!(a, R::new(-2, 15));

    let mut a = R::new(7, 10);
    let b = R::new(-5, 6);
    a *= b;
    assert_eq!(a, R::new(-7, 12));

    let mut a = R::new(7, 10);
    let b = R::new(-5, 6);
    a /= b;
    assert_eq!(a, R::new(-21, 25));

    assert_eq!(format!("{}", R::new(-3, 4)), "-3/4");
    assert_eq!(format!("{:?}", R::new(-3, 4)), "-3/4");
}

#[test]
fn ord_eq() {
    for num1 in -10..=10 {
        for den1 in 1..10 {
            for num2 in -10..=10 {
                for den2 in 1..10 {
                    let a = R::new(num1, den1);
                    let b = R::new(num2, den2);
                    let af = num1 as f64 / den1 as f64;
                    let bf = num2 as f64 / den2 as f64;

                    assert_eq!(a.cmp(&b), af.total_cmp(&bf));
                    assert_eq!(a == b, af == bf);
                }
            }
        }
    }
}

#[test]
fn floor_ceil() {
    assert_eq!(R::new(3, 4).floor(), R::new(0, 1));
    assert_eq!(R::new(3, 4).ceil(), R::new(1, 1));
    assert_eq!(R::new(-3, 4).floor(), R::new(-1, 1));
    assert_eq!(R::new(-3, 4).ceil(), R::new(0, 1));

    for num in -10..=10 {
        for den in 1..10 {
            let a = R::new(num, den);
            let af = num as f64 / den as f64;
            let floor = a.floor();
            let ceil = a.ceil();

            assert_eq!(floor.a, af.floor().round() as i32);
            assert_eq!(ceil.a, af.ceil().round() as i32);
        }
    }
}
