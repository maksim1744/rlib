use std::time::Instant;

use rlib_fft::{multiply_verify, precision::VALS_TO_CHECK, FFT};
use rlib_rand::{Rand, Rng};

fn main() {
    let lens = vec![
        10, 100, 1000, 10000, 100000, 200000, 300000, 500000, 1000000, 2000000, 5000000,
    ];
    let vals = &VALS_TO_CHECK;
    println!("vals: {:?}", vals);
    println!("lens: {:?}", lens);
    let backs: Vec<i32> = vec![1, 0, 10, 1000, i32::MAX];

    let mut result = vec![vec![0usize; vals.len()]; vals.len()];
    let mut finished_runs = 0;
    let start = Instant::now();

    let mut fft = FFT::<f32>::new();
    for (ai, &a_max) in vals.iter().enumerate().rev() {
        for (bi, &b_max) in vals.iter().enumerate().rev() {
            let mut rng = Rng::from_time();
            let current = Instant::now();
            let mut run_test = |len: usize, iter: usize| -> bool {
                let ba = backs[iter % backs.len()];
                let bb = backs[iter / backs.len() % backs.len()];
                let a = (0..len)
                    .map(|_| rng.next((a_max - ba).max(0)..=a_max))
                    .collect::<Vec<i32>>();
                let b = (0..len)
                    .map(|_| rng.next((b_max - bb).max(0)..=b_max))
                    .collect::<Vec<i32>>();

                let c = fft.multiply(&a, &b);
                multiply_verify(&a, &b, &c)
            };

            let mut len_max = 0;
            while len_max < lens.len() && run_test(len_max, 0) {
                len_max += 1;
            }
            if ai + 1 < VALS_TO_CHECK.len() {
                result[ai][bi] = result[ai][bi].max(result[ai + 1][bi]);
            }
            if bi + 1 < VALS_TO_CHECK.len() {
                result[ai][bi] = result[ai][bi].max(result[ai][bi + 1]);
            }

            for &len in lens[..len_max].iter().rev() {
                if len <= result[ai][bi] {
                    break;
                }
                let mut ok = true;
                for iter in 1..100 {
                    if !run_test(len, iter) {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    result[ai][bi] = len;
                    break;
                }
            }
            finished_runs += 1;
            println!(
                "test [{}/{}]: a={} b={}: {} in {:.2?} / total {:.2?}",
                finished_runs,
                VALS_TO_CHECK.len() * VALS_TO_CHECK.len(),
                a_max,
                b_max,
                result[ai][bi],
                current.elapsed(),
                start.elapsed()
            );
        }
    }

    println!("result:");
    for row in result.iter() {
        println!("{:?}", row);
    }

    let print_scientific = |mut x: usize| -> String {
        let mut e = 0;
        while x % 10 == 0 {
            x /= 10;
            e += 1;
        }
        format!("{}e{}", x, e)
    };
    let print_int = |x: usize| -> String {
        if x < 1000 {
            x.to_string()
        } else {
            print_scientific(x)
        }
    };
    let print_f64 = |x: usize| -> String {
        if x < 10 {
            format!("{}.", x)
        } else {
            print_scientific(x)
        }
    };

    let header = VALS_TO_CHECK
        .iter()
        .map(|&x| format!("{: >4}", print_int(x as usize)))
        .collect::<Vec<_>>()
        .join(",");
    println!("//         {}", header);
    for (i, &val) in VALS_TO_CHECK.iter().enumerate() {
        println!(
            "/* {: >3} */  [{}],",
            print_int(val as usize),
            result[i]
                .iter()
                .map(|&x| format!("{: >3}", print_f64(x)))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
