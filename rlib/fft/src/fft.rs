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
        let mut res = Self {
            w: vec![Complex::ONE, Complex::ONE],
            reversed: vec![0],
            bufs: [vec![], vec![]],
        };
        res.update_n(4);
        res
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
        let n = v.len();
        if n == 1 {
            if !res.is_empty() {
                res[0] += v[0].x.round().to_i64();
            }
            return;
        }
        let buf = &mut self.bufs[0];
        buf.clear();
        buf.resize(v.len(), Complex::ZERO);
        for (i, &x) in v.iter().enumerate() {
            buf[i] = x;
        }
        let i2 = F::ONE / F::from_usize(2);
        let max_n = self.reversed.len();
        let step = max_n / n;
        let start = max_n - (max_n >> 2);
        for i in 0..(n >> 1) {
            let j = i + n / 2;
            buf[i] = (buf[i] + buf[j] - (buf[i] - buf[j]) * self.w[start - step * i]) * i2;
        }

        buf.truncate(n >> 1);
        self.fft_internal::<0>(0, n >> 1, true);
        res.iter_mut()
            .zip(
                self.bufs[0]
                    .iter()
                    .flat_map(|c| [c.x.round().to_i64(), c.y.round().to_i64()]),
            )
            .for_each(|(x, y)| *x += y);
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

        let i8 = Complex::I / F::from_usize(8);
        for i in 0..=(n >> 1) {
            // a --fft--> a1 + a2*i
            // b --fft--> b1 + b2*i
            // fact: FFT(a)[k] = FFT(a)[n - k].conj()
            // using this we can get formulas for FFT(a) and FFT(b) from FFT(a+bi)
            // (some calculations are moved from the next for-loop for the purpose of optimization)

            let j = (n - i) & (n - 1);
            let v = (buf[i] + buf[j].conj()) * (buf[j].conj() - buf[i]) * i8;

            buf[i] = v;
            buf[j] = v.conj();
        }

        // we know that Im(c)=0 and we know fft(c)
        // let c' be (c[0]+c[1]*i)*x^0 + (c[2]+c[3]*i)*x^1 + (c[4]+c[5]*i)*x^2
        // then c'(x^2) = (c(x) + c(-x)) / 2 + (c(x) - c(-x)) / 2x * i
        // and knowing values of c at some x-s (which is precisely fft(c)), we can calculate fft(c')

        let max_n = self.reversed.len();
        let step = max_n / n;
        let start = max_n - (max_n >> 2);
        for i in 0..(n >> 1) {
            let j = i + (n >> 1);
            buf[i] = buf[i] + buf[j] - (buf[i] - buf[j]) * self.w[start - step * i];
        }

        buf.truncate(n >> 1);
        self.fft_internal::<0>(0, n >> 1, true);

        res.iter_mut()
            .zip(
                self.bufs[0]
                    .iter()
                    .flat_map(|c| [c.x.round().to_i64(), c.y.round().to_i64()]),
            )
            .take(a.len() + b.len() - 1)
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
