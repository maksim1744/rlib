use crate::randomable::*;

pub trait Rand {
    fn next<T, R>(&mut self, range: R) -> T
    where
        R: Randomable<T>;

    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in 1..v.len() {
            v.swap(i, self.next(0..=i));
        }
    }
}
