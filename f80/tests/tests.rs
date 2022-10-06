use rlib_f80::*;

#[test]
fn precision() {
    f80_init();

    // 1 + 1e17 - 1 will give 0 in f64
    let a = f80::from(1e17);
    let b = f80::from(1.);
    let c = a + b;
    let d = c - a;
    assert_eq!(d, f80::from(1.));
}

#[test]
fn arithmetics() {
    f80_init();

    let a = f80::from(123.456);
    let b = f80::from(-100.100);

    let assert_eq_eps = |x: f80, y: f80| {
        assert!((x - y).abs() < f80::from(1e-12));
    };

    assert_eq_eps(a + b, f80::from(23.356));
    assert_eq_eps(a - b, f80::from(223.556));
    assert_eq_eps(a / b, f80::from(-1.2333266733266735));
    assert_eq_eps(a * b, f80::from(-12357.9456));

    let mut c = a;
    c += b;
    assert_eq_eps(c, f80::from(23.356));
    let mut c = a;
    c -= b;
    assert_eq_eps(c, f80::from(223.556));
    let mut c = a;
    c /= b;
    assert_eq_eps(c, f80::from(-1.2333266733266735));
    let mut c = a;
    c *= b;
    assert_eq_eps(c, f80::from(-12357.9456));

    assert_eq!(-a, f80::from(-123.456));
    assert_eq!(-b, f80::from(100.100));
}

#[test]
fn ord() {
    f80_init();

    let v: Vec<f64> = vec![
        0., -0., 0.0001, -0.0001, 1e50, -1e50, 42.42, -42.42, 1., -1., 42.4200001,
    ];

    let dumb_min_max = |x: f80, y: f80| -> (f80, f80) {
        if x < y {
            (x, y)
        } else {
            (y, x)
        }
    };

    for &a in v.iter() {
        for &b in v.iter() {
            let x = f80::from(a);
            let y = f80::from(b);
            assert_eq!(dumb_min_max(x, y), (x.min(y), x.max(y)));

            assert_eq!(a < b, x < y);
            assert_eq!(a <= b, x <= y);
            assert_eq!(a > b, x > y);
            assert_eq!(a >= b, x >= y);
            assert_eq!(a.partial_cmp(&b), x.partial_cmp(&y));
        }
    }
}

#[test]
fn print() {
    f80_init();

    assert_eq!(format!("{:.5}", f80::from(10. / 3.)), "3.33333");
    assert_eq!(format!("{:.5?}", f80::from(10. / 3.)), "3.33333");
    assert_eq!(format!("{:.5}", f80::from(-10. / 3.)), "-3.33333");
    assert_eq!(format!("{:.5?}", f80::from(-10. / 3.)), "-3.33333");
}

#[test]
fn constants() {
    f80_init();

    assert_eq!(f80::ZERO, f80::from(0.));
    assert_eq!(f80::ONE, f80::from(1.));
}
