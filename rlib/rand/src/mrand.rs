use std::time::SystemTime;

use crate::randomable::*;

pub trait Rand {
    fn from_seed(seed: u64) -> Self;

    fn from_time() -> Self
    where
        Self: Sized,
    {
        Self::from_seed(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        )
    }

    fn next<T, R>(&mut self, range: R) -> T
    where
        R: Randomable<T>;

    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in 1..v.len() {
            v.swap(i, self.next(0..=i));
        }
    }
}
