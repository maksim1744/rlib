use crate::{randomable::Randomable, Rand};

pub struct MersenneTwister<
    const W: u32,
    const N: usize,
    const M: usize,
    const R: u32,
    const A: u64,
    const B: u64,
    const C: u64,
    const S: u64,
    const T: u64,
    const U: u64,
    const D: u64,
    const L: u32,
    const F: u64,
> {
    state: [u64; N],
    index: usize,
}

impl<
        const W: u32,
        const N: usize,
        const M: usize,
        const R: u32,
        const A: u64,
        const B: u64,
        const C: u64,
        const S: u64,
        const T: u64,
        const U: u64,
        const D: u64,
        const L: u32,
        const F: u64,
    > MersenneTwister<W, N, M, R, A, B, C, S, T, U, D, L, F>
{
    const UMASK: u64 = u64::MAX << R;
    const LMASK: u64 = u64::MAX >> (W - R);

    pub const fn new(mut seed: u64) -> Self {
        let mut state = [0u64; N];
        state[0] = seed;
        let mut i = 1;
        while i < N {
            seed = F.wrapping_mul(seed ^ (seed >> (W - 2))).wrapping_add(i as u64);
            state[i] = seed;
            i += 1;
        }
        Self { state, index: 0 }
    }

    pub fn next_raw(&mut self) -> u64 {
        let mut k = self.index;
        let mut j = k + 1;
        if j >= N {
            j -= N;
        }

        let x = (self.state[k] & Self::UMASK) | (self.state[j] & Self::LMASK);
        let mut x_a = x >> 1;
        if (x & 1) != 0 {
            x_a ^= A;
        }

        j = k + M;
        if j >= N {
            j -= N;
        }

        let x = self.state[j] ^ x_a;
        self.state[k] = x;
        k += 1;

        if k >= N {
            k = 0;
        }
        self.index = k;

        let mut y = x ^ (x >> U);
        y ^= (y << S) & B;
        y ^= (y << T) & C;

        y ^ (y >> L)
    }
}

impl<
        const W: u32,
        const N: usize,
        const M: usize,
        const R: u32,
        const A: u64,
        const B: u64,
        const C: u64,
        const S: u64,
        const T: u64,
        const U: u64,
        const D: u64,
        const L: u32,
        const F: u64,
    > Rand for MersenneTwister<W, N, M, R, A, B, C, S, T, U, D, L, F>
{
    fn from_seed(seed: u64) -> Self {
        Self::new(seed)
    }

    fn next<Type, Range>(&mut self, range: Range) -> Type
    where
        Range: Randomable<Type>,
    {
        range.gen_from_u64(self.next_raw())
    }
}
