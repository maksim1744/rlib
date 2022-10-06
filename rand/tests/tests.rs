use rlib_rand::*;

#[test]
fn from_time() {
    // surely it takes at least one nanosecond
    let mut r1 = Rng::from_time();
    let mut r2 = Rng::from_time();
    assert_ne!(r1.next_raw(), r2.next_raw());
}

macro_rules! test_minmax {
    ($t:ty, $r:expr) => {{
        const ITS: usize = 100000;
        // probability of not getting a border is at most (1 - 1/256)^ITS = 0
        let r: std::ops::Range<$t> = $r;
        let mut min = <$t>::MAX;
        let mut max = <$t>::MIN;
        let mut rng = Rng::from_seed(42);
        for _ in 0..ITS {
            let val = rng.next(r.clone());
            min = min.min(val);
            max = max.max(val);
        }
        assert_eq!(min, r.start);
        assert_eq!(max, r.end - 1);
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
fn small_overflows() {
    test_minmax!(i8, 0..1);
    test_minmax!(i8, 10..11);
    test_minmax!(i8, 120..127);
    test_minmax!(i8, 126..127);
    test_minmax!(i8, -10..-9);
    test_minmax!(i8, -128..-127);
    test_minmax!(i8, -128..127);
    test_minmax!(i8, -80..80);

    test_minmax!(u8, 0..1);
    test_minmax!(u8, 254..255);
    test_minmax!(u8, 0..255);
    test_minmax!(u8, 10..20);
    test_minmax!(u8, 150..154);
    test_minmax!(u8, 42..250);
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