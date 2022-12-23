use rlib_integer::*;

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != T::ZERO {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn lcm<T: Integer>(a: T, b: T) -> T {
    a.abs() / gcd(a, b) * b.abs()
}

pub fn egcd<T: Integer>(a: T, b: T, c: T) -> (T, T) {
    if a == T::ZERO {
        assert!(c % b == T::ZERO);
        return (T::ZERO, c / b);
    }
    let (y0, x0) = egcd(b % a, a, c);
    (x0 - (b / a) * y0, y0)
}

pub fn crt<T: Integer + std::ops::Neg<Output = T>>(a1: T, m1: T, a2: T, m2: T) -> T {
    let g = gcd(m1, m2);
    assert!(a1 % g == a2 % g);
    let (x, _) = egcd(m1, -m2, a2 - a1);
    let m2 = m2 / g;
    let x = (x % m2 + m2) % m2;
    m1 * x + a1
}
