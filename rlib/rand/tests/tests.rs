use rlib_rand::*;

macro_rules! test_minmax {
    ($t:ty, $range:expr, $l:expr, $r:expr) => {{
        const ITS: usize = 10000;
        // probability of not getting a border is at most (1 - 1/256)^ITS = 1e-17
        let mut min = <$t>::MAX;
        let mut max = <$t>::MIN;
        let mut rng = Rng::from_seed(42);
        for _ in 0..ITS {
            let val = rng.next::<$t, _>($range);
            min = min.min(val);
            max = max.max(val);
        }
        assert_eq!(min, $l);
        assert_eq!(max, $r);
    }};
}

macro_rules! test_minmax_approx {
    ($t:ty, $r:expr, $delta:expr) => {{
        const ITS: usize = 100000;
        let r: std::ops::Range<$t> = $r;
        let mut min = <$t>::MAX;
        let mut max = <$t>::MIN;
        let mut rng = Rng::from_seed(42);
        for _ in 0..ITS {
            let val = rng.next(r.clone());
            min = min.min(val);
            max = max.max(val);
        }
        // EV for min is start + (end - start) / (ITS + 1), but for assert purposes we'll take ITS/10
        let k = (ITS / 10) as $t;
        assert!(min <= r.start + (r.end - r.start) / k);
        assert!(max >= r.end - (r.end - r.start) / k - $delta);
    }};
}

macro_rules! test_boundaries {
    ($t:ty, $r:expr) => {{
        const ITS: usize = 100000;
        let r: std::ops::Range<$t> = $r;
        let mut rng = Rng::from_seed(42);
        for _ in 0..ITS {
            let val = rng.next(r.clone());
            assert!(r.start <= val && val < r.end);
        }
    }};
}

#[test]
fn range() {
    for l in i8::MIN..i8::MAX {
        for r in l + 1..=i8::MAX {
            test_minmax!(i8, l..r, l, r - 1);
        }
    }
    for l in u8::MIN..u8::MAX {
        for r in l + 1..=u8::MAX {
            test_minmax!(u8, l..r, l, r - 1);
        }
    }
}

#[test]
fn range_inclusive() {
    for l in i8::MIN..=i8::MAX {
        for r in l..=i8::MAX {
            test_minmax!(i8, l..=r, l, r);
        }
    }
    for l in u8::MIN..=u8::MAX {
        for r in l..=u8::MAX {
            test_minmax!(u8, l..=r, l, r);
        }
    }
}

#[test]
fn range_inclusive_to() {
    for r in 0..=i8::MAX {
        test_minmax!(i8, ..=r, 0, r);
    }
    for r in 0..=u8::MAX {
        test_minmax!(u8, ..=r, 0, r);
    }
}

#[test]
fn range_to() {
    for r in 1..=i8::MAX {
        test_minmax!(i8, ..r, 0, r - 1);
    }
    for r in 1..=u8::MAX {
        test_minmax!(u8, ..r, 0, r - 1);
    }
}

#[test]
fn f64() {
    test_boundaries!(f64, 10.0..15.0);
    test_boundaries!(f64, -10.0..15.0);
    test_boundaries!(f64, -15.0..-10.0);

    test_minmax_approx!(f64, 10.0..15.0, 0.0);
    test_minmax_approx!(f64, -10.0..15.0, 0.0);
    test_minmax_approx!(f64, -15.0..-10.0, 0.0);
}

#[test]
fn i64() {
    test_boundaries!(i64, i64::MIN..i64::MAX);
    test_boundaries!(i64, i64::MIN + 10..i64::MAX - 10);
    test_boundaries!(i64, 42..420);
    test_boundaries!(i64, -420..-42);

    test_minmax_approx!(i64, 10..(1e15 as i64), 1);
    test_minmax_approx!(i64, -(1e15 as i64)..10, 1);
    test_minmax_approx!(i64, -10..10, 1);
    test_minmax_approx!(i64, -(1e15 as i64)..(1e15 as i64), 1);
}

#[test]
fn i32() {
    test_boundaries!(i32, i32::MIN..i32::MAX);
    test_boundaries!(i32, i32::MIN + 10..i32::MAX - 10);
    test_boundaries!(i32, 42..420);
    test_boundaries!(i32, -420..-42);

    test_minmax_approx!(i32, 10..(1e7 as i32), 1);
    test_minmax_approx!(i32, -(1e7 as i32)..10, 1);
    test_minmax_approx!(i32, -10..10, 1);
    test_minmax_approx!(i32, -(1e7 as i32)..(1e7 as i32), 1);
}

#[test]
fn u64() {
    test_boundaries!(u64, 0..u64::MAX);
    test_boundaries!(u64, 10..u64::MAX - 10);
    test_boundaries!(u64, 42..420);

    test_minmax_approx!(u64, 10..(1e15 as u64), 1);
    test_minmax_approx!(u64, 0..10, 1);
    test_minmax_approx!(u64, 42..420, 1);
}

#[test]
fn u32() {
    test_boundaries!(u32, 0..u32::MAX);
    test_boundaries!(u32, 10..u32::MAX - 10);
    test_boundaries!(u32, 42..420);

    test_minmax_approx!(u32, 10..(1e7 as u32), 1);
    test_minmax_approx!(u32, 0..10, 1);
    test_minmax_approx!(u32, 42..420, 1);
}
