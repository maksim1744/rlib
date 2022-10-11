use std::time::SystemTime;

use crate::randomable::*;

#[derive(Copy, Clone)]
pub struct LinearCongruentialGenerator64<const A: u64, const C: u64> {
    state: u64,
}

impl<const A: u64, const C: u64> LinearCongruentialGenerator64<A, C> {
    pub const fn from_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn from_time() -> Self {
        Self {
            state: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }

    pub fn next_raw(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }

    pub fn next<T, R>(&mut self, range: R) -> T
    where
        R: Randomable<T>,
    {
        range.gen_from_u64(self.next_raw())
    }
}
