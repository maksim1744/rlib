use rlib_gcd::*;

#[test]
fn simple() {
    assert_eq!(gcd(10, 15), 5);
    assert_eq!(gcd(10, -15), 5);
    assert_eq!(gcd(-10, 15), 5);
    assert_eq!(gcd(-10, -15), 5);

    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(0, -10), 10);
    assert_eq!(gcd(10, 0), 10);

    assert_eq!(lcm(10, 15), 30);
    assert_eq!(lcm(10, -15), 30);
    assert_eq!(lcm(-10, 15), 30);
    assert_eq!(lcm(-10, -15), 30);

    assert_eq!(lcm(0, -10), 0);
    assert_eq!(lcm(-10, 0), 0);
}

#[test]
fn egcd() {
    for a in -10..10 {
        for b in -10..10 {
            if a == 0 || b == 0 {
                continue;
            }
            for k in -10..10 {
                let c = k * gcd(a, b);
                let (x, y) = rlib_gcd::egcd(a, b, c);
                assert_eq!(a * x + b * y, c);
            }
        }
    }
}

#[test]
#[should_panic]
fn egcd_panic() {
    rlib_gcd::egcd(10, 15, 7);
}
