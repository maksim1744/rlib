use rlib_num_traits::{Float, ZeroOne};

use crate::Complex;

#[derive(Clone)]
pub struct FFT<F: Float> {
    w: Vec<Complex<F>>,
    reversed: Vec<usize>,
    bufs: [Vec<Complex<F>>; 2],
}

impl<F: Float> FFT<F> {
    pub fn new() -> Self {
        Self {
            w: vec![Complex::ONE, Complex::ONE],
            reversed: vec![0],
            bufs: [vec![], vec![]],
        }
    }

    pub fn update_n(&mut self, n: usize) {
        assert_eq!(n & (n - 1), 0);
        let mut cur = self.reversed.len();
        if n <= cur {
            return;
        }
        self.reversed.resize(n, 0);
        self.w.resize(n + 1, Complex::ZERO);
        while cur < n {
            for i in 0..cur {
                self.reversed[i] <<= 1;
            }
            for i in cur..(cur << 1) {
                self.reversed[i] = self.reversed[i - cur] ^ 1;
            }
            (1..=(cur << 1) - 2).rev().step_by(2).for_each(|i| {
                self.w[i] = self.w[i / 2];
            });
            let icur = F::ONE / F::from_usize(cur);
            (1..(cur << 1)).step_by(2).for_each(|i| {
                let x = F::PI * F::from_usize(i) * icur;
                self.w[i] = Complex::new(x.cos(), x.sin())
            });
            cur *= 2;
        }
        *self.w.last_mut().unwrap() = Complex::ONE;
    }

    fn fft_internal<const B: usize>(&mut self, from: usize, n: usize, inv: bool) {
        self.update_n(n);
        let v = &mut self.bufs[B][from..from + n];
        let max_n = self.reversed.len();
        let d = max_n.ilog2() - n.ilog2();

        for i in 1..n {
            if i < (self.reversed[i] >> d) {
                v.swap(i, self.reversed[i] >> d);
            }
        }

        let mut ln = 1;
        while ln < n {
            let step: isize = (if inv { -(max_n as isize) } else { max_n as isize }) / (ln as isize * 2);
            (0..n).step_by(ln << 1).for_each(|i| {
                let mut ind: isize = if inv { max_n as isize } else { 0 };
                for j in 0..ln {
                    let y = v[i + j + ln] * self.w[ind as usize];
                    ind += step;
                    v[i + j + ln] = v[i + j] - y;
                    v[i + j] += y;
                }
            });
            ln <<= 1;
        }

        if inv {
            let invn = F::ONE / F::from_usize(n);
            for x in v.iter_mut() {
                *x *= invn;
            }
        }
    }

    pub fn fft(&mut self, v: &[i32], mut n: usize) -> Vec<Complex<F>> {
        if n == 0 {
            n = 1;
            while n < v.len() {
                n <<= 1;
            }
        }

        let mut res = vec![Complex::ZERO; n];
        self.fft_into(v, n, &mut res);
        res
    }

    pub fn fft_into(&mut self, v: &[i32], mut n: usize, res: &mut [Complex<F>]) {
        if n == 0 {
            n = 1;
            while n < v.len() {
                n <<= 1;
            }
        }
        debug_assert!(v.len() <= n);
        self.bufs[0].clear();
        self.bufs[0].resize(n, Complex::ZERO);
        for (i, &x) in v.iter().enumerate() {
            self.bufs[0][i].x = F::from_i32(x);
        }
        self.fft_internal::<0>(0, n, false);
        res.iter_mut().zip(self.bufs[0].iter()).for_each(|(x, &y)| *x += y);
    }

    pub fn fft_inv(&mut self, v: &[Complex<F>]) -> Vec<i64> {
        let mut res = vec![0; v.len()];
        self.fft_inv_into(v, &mut res);
        res
    }

    pub fn fft_inv_into(&mut self, v: &[Complex<F>], res: &mut [i64]) {
        debug_assert!(!v.is_empty());
        debug_assert!((v.len() & (v.len() - 1)) == 0);
        self.bufs[0].clear();
        self.bufs[0].resize(v.len(), Complex::ZERO);
        for (i, &x) in v.iter().enumerate() {
            self.bufs[0][i] = x;
        }
        self.fft_internal::<0>(0, v.len(), true);
        res.iter_mut()
            .zip(self.bufs[0].iter())
            .for_each(|(x, &y)| *x += y.x.round().to_i64());
    }

    pub fn multiply(&mut self, a: &[i32], b: &[i32]) -> Vec<i64> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut res = vec![0; a.len() + b.len() - 1];
        self.multiply_into(a, b, &mut res);
        res
    }

    pub fn multiply_into(&mut self, a: &[i32], b: &[i32], res: &mut [i64]) {
        if a.is_empty() || b.is_empty() {
            return;
        }
        let mut n = 2;
        while n < a.len() + b.len() - 1 {
            n *= 2;
        }

        self.bufs[0].clear();
        self.bufs[0].resize(n, Complex::ZERO);

        for (i, &x) in a.iter().enumerate() {
            self.bufs[0][i].x = F::from_i32(x);
        }
        for (i, &x) in b.iter().enumerate() {
            self.bufs[0][i].y = F::from_i32(x);
        }

        self.fft_internal::<0>(0, n, false);
        let buf = &mut self.bufs[0];

        let i4 = F::ONE / F::from_usize(4);
        for i in 0..=(n >> 1) {
            // a --fft--> a1 + a2*i
            // b --fft--> b1 + b2*i
            // fact: FFT(a)[k] = FFT(a)[n - k].conj()
            // using this we can get formulas for FFT(a) and FFT(b) from FFT(a+bi)

            let j = (n - i) & (n - 1);
            let mut v = (buf[i] + buf[j].conj()) * (buf[i] - buf[j].conj()) * i4;
            std::mem::swap(&mut v.x, &mut v.y);

            buf[i] = v.conj();
            buf[j] = v;
        }

        self.fft_internal::<0>(0, n, true);

        res.iter_mut()
            .zip((0..(a.len() + b.len() - 1)).map(|i| self.bufs[0][i].x.round().to_i64()))
            .for_each(|(x, y)| *x += y);
    }
}

/// Try some x-s and check that a(x) * b(x) == c(x)
pub fn multiply_verify(a: &[i32], b: &[i32], c: &[i64]) -> bool {
    let calc_32_at = |v: &[i32], x: i64| -> i64 {
        let mut px = 1i64;
        let mut res = 0i64;
        for &k in v.iter() {
            res = res.wrapping_add((k as i64).wrapping_mul(px));
            px = px.wrapping_mul(x);
        }
        res
    };
    let calc_64_at = |v: &[i64], x: i64| -> i64 {
        let mut px = 1i64;
        let mut res = 0i64;
        for &k in v.iter() {
            res = res.wrapping_add(k.wrapping_mul(px));
            px = px.wrapping_mul(x);
        }
        res
    };

    for x in [0i64, 1i64, -1i64, 42i64, -42i64, (1i64 << 31) + 1, -(1i64 << 31) + 1] {
        if calc_32_at(a, x).wrapping_mul(calc_32_at(b, x)) != calc_64_at(c, x) {
            return false;
        }
    }
    true
}

impl<F: Float> Default for FFT<F> {
    fn default() -> Self {
        Self::new()
    }
}
