use std::ops::*;

use crate::bits_iter::BitsIter;

#[derive(Clone, Eq, PartialEq)]
pub struct Bitset<const N: usize> {
    data: [u64; N],
}

impl<const N: usize> Bitset<N> {
    pub fn new() -> Self {
        Self { data: [0; N] }
    }

    pub fn from_u64(x: u64) -> Self {
        let mut data = [0; N];
        data[0] = x;
        Self { data }
    }

    pub fn set(&mut self, x: usize) {
        self.data[x / 64] |= 1u64 << (x % 64);
    }

    pub fn remove(&mut self, x: usize) {
        self.data[x / 64] &= !(1u64 << (x % 64));
    }

    pub fn flip(&mut self, x: usize) {
        self.data[x / 64] ^= 1u64 << (x % 64);
    }

    pub fn test(&self, x: usize) -> bool {
        ((self.data[x / 64] >> (x % 64)) & 1) > 0
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    pub fn iter_bits(&self) -> BitsIter<N> {
        BitsIter::new(&self.data)
    }

    pub fn count(&self) -> usize {
        self.data.iter().map(|x| x.count_ones() as usize).sum::<usize>()
    }
}

macro_rules! bin_op {
    ($trait:ident, $func:ident) => {
        impl<const N: usize> $trait for &Bitset<N> {
            type Output = Bitset<N>;

            fn $func(self, rhs: &Bitset<N>) -> Self::Output {
                let mut result = Bitset::<N>::new();
                for (i, (x, y)) in self.data.iter().zip(rhs.data.iter()).enumerate() {
                    result.data[i] = x.$func(y);
                }
                result
            }
        }
    };
}

bin_op!(BitAnd, bitand);
bin_op!(BitOr, bitor);
bin_op!(BitXor, bitxor);

macro_rules! bin_op_assign {
    ($trait:ident, $func:ident) => {
        impl<const N: usize> $trait<&Bitset<N>> for Bitset<N> {
            fn $func(&mut self, rhs: &Bitset<N>) {
                for (x, y) in self.data.iter_mut().zip(rhs.data.iter()) {
                    x.$func(y);
                }
            }
        }
    };
}

bin_op_assign!(BitAndAssign, bitand_assign);
bin_op_assign!(BitOrAssign, bitor_assign);
bin_op_assign!(BitXorAssign, bitxor_assign);

impl<const N: usize> Not for Bitset<N> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        for x in self.data.iter_mut() {
            *x = !*x;
        }
        self
    }
}

impl<const N: usize> Default for Bitset<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> std::fmt::Display for Bitset<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..N * 64)
                .map(|i| (self.test(i) as i32).to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
impl<const N: usize> std::fmt::Debug for Bitset<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..N * 64)
                .map(|i| (self.test(i) as i32).to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
