use rlib_sieve::*;

#[test]
fn primes() {
    assert_eq!(Sieve::new(17).primes(), &vec![2, 3, 5, 7, 11, 13, 17]);
    assert_eq!(Sieve::new(16).primes(), &vec![2, 3, 5, 7, 11, 13]);
    assert_eq!(*Sieve::new(1_000_000).primes().last().unwrap(), 999983);
}

#[test]
fn min_prime() {
    let sieve = Sieve::new(100);
    for i in 2..=100 {
        let mut p = 2;
        while i % p != 0 {
            p += 1;
        }
        assert_eq!(p, sieve.min_prime(i as i32));
    }
}

#[test]
fn is_prime() {
    let sieve = Sieve::new(100);
    assert!(!sieve.is_prime(0));
    assert!(!sieve.is_prime(1));
    for i in 2..=100 {
        let mut p = 2;
        while i % p != 0 {
            p += 1;
        }
        assert_eq!(p == i, sieve.is_prime(i as i32));
    }
}

#[test]
fn factorize() {
    let sieve = Sieve::new(100);
    assert_eq!(sieve.factorize(1).collect::<Vec<_>>(), Vec::new());
    assert_eq!(sieve.factorize(2).collect::<Vec<_>>(), vec![(2, 1)]);
    assert_eq!(sieve.factorize(12).collect::<Vec<_>>(), vec![(2, 2), (3, 1)]);
    assert_eq!(sieve.factorize(14).collect::<Vec<_>>(), vec![(2, 1), (7, 1)]);
    assert_eq!(sieve.factorize(97).collect::<Vec<_>>(), vec![(97, 1)]);
    assert_eq!(sieve.factorize(64).collect::<Vec<_>>(), vec![(2, 6)]);

    for i in 1..=100 {
        let mut x = i;
        let mut res: Vec<(i32, i32)> = Vec::new();
        for p in 2..=i {
            if x % p == 0 {
                let mut c = 0;
                while x % p == 0 {
                    c += 1;
                    x /= p;
                }
                res.push((p, c));
            }
        }

        assert_eq!(sieve.factorize(i).collect::<Vec<_>>(), res);
    }
}
