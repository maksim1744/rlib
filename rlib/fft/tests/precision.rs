use rlib_fft::{
    multiply_verify,
    precision::{CORRECT_F64_BOUNDS, VALS_TO_CHECK},
    FFT,
};
use rlib_rand::{Rand, Rng};

#[test]
fn precision() {
    const BACKS: [i32; 2] = [0, 1000];

    let mut rng = Rng::from_seed(42);
    let mut fft = FFT::<f64>::new();
    for (ai, &amax) in VALS_TO_CHECK.iter().enumerate() {
        for (bi, &bmax) in VALS_TO_CHECK.iter().enumerate() {
            let len = CORRECT_F64_BOUNDS[ai][bi] as usize / 2;
            if len == 0 {
                continue;
            }
            if cfg!(debug_assertions) && len > 1000 {
                continue;
            }
            // assume transitivity
            if ai + 1 < VALS_TO_CHECK.len() && CORRECT_F64_BOUNDS[ai + 1][bi] as usize / 2 == len {
                continue;
            }
            if bi + 1 < VALS_TO_CHECK.len() && CORRECT_F64_BOUNDS[ai][bi + 1] as usize / 2 == len {
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
