use rlib_fft::FFT;
use rlib_rand::{Rand, Rng};

fn double_multiply(fft: &mut FFT<f64>, a: &[i32], b: &[i32]) -> Vec<i64> {
    let c = fft.multiply(a, b);
    if a.is_empty() || b.is_empty() {
        return c;
    }
    let mut n = 1;
    while n < a.len() + b.len() - 1 {
        n *= 2;
    }
    let fft_a = fft.fft(a, n);
    let fft_b = fft.fft(b, n);
    let fft_c = fft_a
        .into_iter()
        .zip(fft_b.into_iter())
        .map(|(x, y)| x * y)
        .collect::<Vec<_>>();
    let mut c2 = fft.fft_inv(&fft_c);
    c2.truncate(a.len() + b.len() - 1);
    assert_eq!(c, c2);
    c
}

#[test]
fn simple() {
    let mut fft = FFT::<f64>::new();

    assert_eq!(double_multiply(&mut fft, &[], &[]), vec![]);
    assert_eq!(double_multiply(&mut fft, &[], &[1, 2, 3]), vec![]);
    assert_eq!(double_multiply(&mut fft, &[1, 2, 3], &[]), vec![]);
    assert_eq!(double_multiply(&mut fft, &[3], &[4]), vec![12]);

    assert_eq!(double_multiply(&mut fft, &[2, 3], &[4, 5]), vec![8, 22, 15]);
}

#[test]
fn stress() {
    const LENS: [usize; 8] = [1, 2, 3, 4, 5, 10, 100, 1000];

    let mut rng = Rng::from_seed(42);
    let mut fft = FFT::<f64>::new();
    for &a_len in LENS.iter() {
        for &b_len in LENS.iter() {
            let a = (0..a_len).map(|_| rng.next(-100..=100)).collect::<Vec<i32>>();
            let b = (0..b_len).map(|_| rng.next(-100..=100)).collect::<Vec<i32>>();
            let mut c: Vec<i64> = vec![0; a.len() + b.len() - 1];
            for (i, x) in a.iter().enumerate() {
                for (j, y) in b.iter().enumerate() {
                    c[i + j] += (x * y) as i64;
                }
            }
            assert_eq!(double_multiply(&mut fft, &a, &b), c);
        }
    }
}
