use rlib_bitset::*;
use rlib_rand::*;

#[test]
fn simple() {
    let mut b = Bitset::<10>::new();
    assert_eq!(b.count(), 0);
    b.set(0);
    b.set(63);
    b.set(64);
    b.set(639);
    b.set(638);
    assert_eq!(b.count(), 5);
    let mut b2 = b.clone();
    b2.clear();
    assert_eq!(b2.count(), 0);
    b.remove(0);
    b.remove(63);
    b.remove(64);
    b.remove(639);
    b.remove(638);
    assert_eq!(b.count(), 0);
    assert_eq!(b, b2);
}

#[test]
fn bin_ops() {
    let mut b1 = Bitset::<10>::new();
    let mut b2 = Bitset::<10>::new();

    b1.set(0);
    b1.set(125);

    b2.set(5);
    b2.set(125);

    assert_eq!((&b1 & &b2).iter_bits().collect::<Vec<_>>(), vec![125]);
    assert_eq!((&b1 | &b2).iter_bits().collect::<Vec<_>>(), vec![0, 5, 125]);
    assert_eq!((&b1 ^ &b2).iter_bits().collect::<Vec<_>>(), vec![0, 5]);

    let mut b3 = b1.clone();
    b3 &= &b2;
    assert_eq!(b3, &b1 & &b2);

    let mut b3 = b1.clone();
    b3 |= &b2;
    assert_eq!(b3, &b1 | &b2);

    let mut b3 = b1.clone();
    b3 ^= &b2;
    assert_eq!(b3, &b1 ^ &b2);

    assert_eq!(
        (!b1).iter_bits().collect::<Vec<_>>(),
        (0..640).filter(|&x| x != 0 && x != 125).collect::<Vec<_>>()
    );
}

#[test]
fn from() {
    let b = Bitset::<2>::from_u64(u64::MAX);
    assert!((0..64).all(|i| b.test(i)));
    assert!(!(65..128).any(|i| b.test(i)));

    let b = Bitset::<2>::from_u64(42);
    assert!((0..64).all(|i| b.test(i) == (((42u64 >> i) & 1) == 1)));
}

#[test]
fn stress() {
    const ITS: usize = 100000;
    const N: usize = 10 * 64;

    let mut b = Bitset::<{ N / 64 }>::new();
    let mut a = vec![false; N];
    let mut rng = Rng::from_seed(42);

    for _ in 0..ITS {
        let tp = rng.next(0..8);
        if tp == 0 {
            let ind = rng.next(..N);
            b.set(ind);
            a[ind] = true;
        } else if tp == 1 {
            let ind = rng.next(..N);
            b.remove(ind);
            a[ind] = false;
        } else if tp == 2 {
            let ind = rng.next(..N);
            b.flip(ind);
            a[ind] = !a[ind];
        } else if tp == 3 {
            let ind = rng.next(..N);
            assert_eq!(b.test(ind), a[ind]);
        } else if tp == 4 {
            assert_eq!(
                (0..N).filter(|&i| a[i]).collect::<Vec<_>>(),
                b.iter_bits().collect::<Vec<_>>()
            );
        } else if tp == 5 && rng.next(0..1000) == 0 {
            a = vec![false; N];
            b.clear();
        } else if tp == 6 {
            assert_eq!(b.count(), a.iter().filter(|x| **x).count());
        } else if tp == 7 {
            assert_eq!(
                format!("{}", b),
                a.iter().map(|&x| (x as i32).to_string()).collect::<Vec<_>>().join("")
            );
        }
    }
}
