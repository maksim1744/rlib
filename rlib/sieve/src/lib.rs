pub struct PrimeIter<'a> {
    sieve: &'a Sieve,
    n: i32,
}

impl Iterator for PrimeIter<'_> {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 1 {
            return None;
        }
        let mut cnt = 0;
        let p = self.sieve.min_prime(self.n);
        while self.sieve.min_prime(self.n) == p {
            cnt += 1;
            self.n /= p;
        }
        Some((p, cnt))
    }
}

pub struct Sieve {
    isp: Vec<bool>,
    mnp: Vec<i32>,
    primes: Vec<i32>,
}

impl Sieve {
    pub fn new(n: usize) -> Self {
        let n = n + 1;
        let mut isp = vec![false; n];
        let mut mnp = vec![0i32; n];
        let mut primes = Vec::new();

        for i in 2..n {
            if mnp[i] == 0 {
                isp[i] = true;
                mnp[i] = i as i32;
                primes.push(i as i32);
            }
            for j in 0..primes.len() {
                if primes[j] > mnp[i] || primes[j] as usize * i >= n {
                    break;
                }
                mnp[primes[j] as usize * i] = primes[j];
            }
        }

        Self { isp, mnp, primes }
    }

    pub fn min_prime(&self, n: i32) -> i32 {
        self.mnp[n as usize]
    }

    pub fn is_prime(&self, n: i32) -> bool {
        self.isp[n as usize]
    }

    pub fn primes(&self) -> &Vec<i32> {
        &self.primes
    }

    pub fn factorize(&self, n: i32) -> PrimeIter {
        PrimeIter { sieve: self, n }
    }
}
