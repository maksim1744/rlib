use rlib_num_traits::*;

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    let mut a = a.into_abs();
    let mut b = b.into_abs();
    while b != T::ZERO {
        a %= &b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn lcm<T: Integer>(a: T, b: T) -> T {
    let b_abs = b.abs();
    a.abs() / &gcd(a, b) * &b_abs
}

pub fn egcd<T: Integer>(a: T, b: T, c: T) -> Option<(T, T)> {
    if a == T::ZERO {
        if c.clone() % &b != T::ZERO {
            return None;
        }
        return Some((T::ZERO, c / &b));
    }
    let (y0, x0) = egcd(b.clone() % &a, a.clone(), c)?;
    Some((x0 - &((b / &a) * &y0), y0))
}

pub fn crt<T: Integer + std::ops::Neg<Output = T>>(a1: T, m1: T, a2: T, m2: T) -> Option<T> {
    let g = gcd(m1.clone(), m2.clone());
    let (x, _) = egcd(m1.clone(), -m2.clone(), a2 - &a1)?;
    let m2 = m2 / &g;
    let x = (x % &m2 + &m2) % &m2;
    Some(m1 * &x + &a1)
}
