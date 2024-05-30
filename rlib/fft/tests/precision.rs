use rlib_fft::{
    multiply_verify,
    precision::{CORRECT_F32_BOUNDS, CORRECT_F64_BOUNDS, VALS_TO_CHECK},
    FFT,
};
use rlib_num_traits::Float;
use rlib_rand::{Rand, Rng};

fn test_precision<F: Float>(bounds: &[[f64; VALS_TO_CHECK.len()]; VALS_TO_CHECK.len()]) {
    const BACKS: [i32; 2] = [0, 1000];

    let mut rng = Rng::from_seed(42);
    let mut fft = FFT::<F>::new();
    for (ai, &amax) in VALS_TO_CHECK.iter().enumerate() {
        for (bi, &bmax) in VALS_TO_CHECK.iter().enumerate() {
            let len = bounds[ai][bi] as usize;
            if len == 0 {
                continue;
            }
            if cfg!(debug_assertions) && len > 1000 {
                continue;
            }
            // assume transitivity
            if ai + 1 < VALS_TO_CHECK.len() && bounds[ai + 1][bi] as usize == len {
                continue;
            }
            if bi + 1 < VALS_TO_CHECK.len() && bounds[ai][bi + 1] as usize == len {
                continue;
            }
            for &aback in BACKS.iter() {
                for &bback in BACKS.iter() {
                    let a = (0..len)
                        .map(|_| rng.next((amax - aback).max(0)..=amax))
                        .collect::<Vec<i32>>();
                    let b = (0..len)
                        .map(|_| rng.next((bmax - bback).max(0)..=bmax))
                        .collect::<Vec<i32>>();
                    let c = fft.multiply(&a, &b);
                    assert!(multiply_verify(&a, &b, &c));
                }
            }
        }
    }
}

#[test]
fn precision_f32() {
    test_precision::<f32>(&CORRECT_F32_BOUNDS);
}

#[test]
fn precision_f64() {
    test_precision::<f64>(&CORRECT_F64_BOUNDS);
}
