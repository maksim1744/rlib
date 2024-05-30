use std::{
    io::Write,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread::available_parallelism,
    time::Instant,
};

use rlib_fft::{multiply_verify, FFT};
use rlib_rand::{Rand, Rng};
use threadpool::ThreadPool;

fn main() {
    let lens = vec![
        10, 100, 1000, 10000, 100000, 200000, 300000, 500000, 1000000, 2000000, 5000000,
    ];
    let vals: Vec<i32> = (0..=8)
        .flat_map(|pow| [1, 5].into_iter().map(move |x| x * 10i32.pow(pow)))
        .chain([10i32.pow(9), 10i32.pow(9) * 2])
        .collect();
    println!("vals: {:?}", vals);
    println!("lens: {:?}", lens);
    let backs: Vec<i32> = vec![1, 0, 10, 1000, i32::MAX];

    let result = Arc::new(Mutex::new(vec![vec![0usize; vals.len()]; vals.len()]));

    let threads = available_parallelism().unwrap().into();
    let pool = ThreadPool::new(threads);

    let ffts = Arc::new(Mutex::new(vec![FFT::<f64>::new(); threads]));
    let finished_run_atomic = Arc::new(AtomicUsize::new(0));
    let total_runs = vals.len() * vals.len();

    for (ai, &a_max) in vals.iter().enumerate().rev() {
        for (bi, &b_max) in vals.iter().enumerate().rev() {
            let lens = lens.clone();
            let backs = backs.clone();
            let result = result.clone();
            let ffts = ffts.clone();
            let finished_run_atomic = finished_run_atomic.clone();
            let now = Instant::now();
            pool.execute(move || {
                let mut fft = ffts.lock().unwrap().pop().unwrap();
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

                for &len in lens[..len_max].iter().rev() {
                    let mut ok = true;
                    for iter in 1..100 {
                        if !run_test(len, iter) {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        result.lock().unwrap()[ai][bi] = len;
                        break;
                    }
                }
                finished_run_atomic.fetch_add(1, Ordering::SeqCst);
                let finished_runs = finished_run_atomic.load(Ordering::SeqCst);
                println!(
                    "test [{}/{}]: a={} b={}: {} in {:.2?} / total {:.2?}",
                    finished_runs,
                    total_runs,
                    a_max,
                    b_max,
                    result.lock().unwrap()[ai][bi],
                    current.elapsed(),
                    now.elapsed()
                );
                std::io::stdout().flush().unwrap();
                ffts.lock().unwrap().push(fft);
            });
        }
    }

    pool.join();

    let result = Arc::into_inner(result).unwrap().into_inner().unwrap();
    println!("result:");
    for row in result.into_iter() {
        println!("{:?}", row);
    }
}
