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
fn test_egcd() {
    for a in -10..10 {
        for b in -10..10 {
            if a == 0 || b == 0 {
                continue;
            }
            for k in -10..10 {
                let c = k * gcd(a, b);
                let (x, y) = egcd(a, b, c).unwrap();
                assert_eq!(a * x + b * y, c);
            }
        }
    }
}

#[test]
fn egcd_no_solution() {
    assert_eq!(egcd(10, 15, 7), None);
}

#[test]
fn test_crt() {
    for m1 in 1..=50i32 {
        for a1 in 0..m1 {
            for m2 in 1..=50 {
                for a2 in 0..m2 {
                    println!("{} {} {} {}", m1, a1, m2, a2);
                    if (a1 - a2).abs() % gcd(m1, m2) != 0 {
                        continue;
                    }
                    let mut k = 0;
                    while k % m1 != a1 || k % m2 != a2 {
                        k += 1;
                    }
                    assert_eq!(k, crt(a1, m1, a2, m2).unwrap());
                }
            }
        }
    }
}
