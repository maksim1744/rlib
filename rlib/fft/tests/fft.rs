use rlib_fft::FFT;
use rlib_rand::{Rand, Rng};

#[test]
fn simple() {
    let mut fft = FFT::<f64>::new();

    assert_eq!(fft.multiply(&[], &[]), vec![]);
    assert_eq!(fft.multiply(&[], &[1, 2, 3]), vec![]);
    assert_eq!(fft.multiply(&[1, 2, 3], &[]), vec![]);
    assert_eq!(fft.multiply(&[3], &[4]), vec![12]);

    assert_eq!(fft.multiply(&[2, 3], &[4, 5]), vec![8, 22, 15]);
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
            assert_eq!(fft.multiply(&a, &b), c);
        }
    }
}
