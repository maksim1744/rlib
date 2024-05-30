use rlib_fft::Complex;
use rlib_num_traits::{Float, ZeroOne};

fn assert_eq_eps<F: Float>(a: Complex<F>, b: Complex<F>) {
    assert!((a.x - b.x).abs() < F::ONE / F::from_usize(10usize.pow(9)));
    assert!((a.y - b.y).abs() < F::ONE / F::from_usize(10usize.pow(9)));
}

#[test]
fn simple() {
    type C = Complex<f64>;

    assert_eq!(C::ZERO + C::ZERO, C::ZERO);
    assert_eq!(C::ONE * C::ONE, C::ONE);

    let a = C::new(3., -4.);
    let b = C::new(-2., 10.);

    assert_eq!(a + b, C::new(1., 6.));
    assert_eq!(a - b, C::new(5., -14.));
    assert_eq!(a * b, C::new(34., 38.));
    assert_eq_eps(a / b, C::new(23. / -52., 11. / -52.));

    let mut c = a;
    c += b;
    assert_eq!(c, a + b);
    let mut c = a;
    c -= b;
    assert_eq!(c, a - b);
    let mut c = a;
    c *= b;
    assert_eq!(c, a * b);
    let mut c = a;
    c /= b;
    assert_eq!(c, a / b);

    let b = 7.;
    assert_eq!(a * b, C::new(21., -28.));
    assert_eq_eps(a / b, C::new(3. / 7., 4. / -7.));

    let mut c = a;
    c *= b;
    assert_eq!(c, a * b);
    let mut c = a;
    c /= b;
    assert_eq!(c, a / b);

    assert_eq!(a.abs2(), 25.);
    assert!((a.abs() - 5.) < 1e-9);
}
