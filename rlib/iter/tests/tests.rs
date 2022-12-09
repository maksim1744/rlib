use rlib_iter::*;

#[test]
fn submasks() {
    assert_eq!(
        iter_submasks(u8::MAX).collect::<Vec<_>>(),
        (0..=u8::MAX).rev().collect::<Vec<_>>()
    );

    assert_eq!(
        iter_submasks(-1i8).collect::<Vec<_>>(),
        (0..=u8::MAX).map(|i| i as i8).rev().collect::<Vec<_>>()
    );

    assert_eq!(iter_submasks(0i32).collect::<Vec<_>>(), vec![0]);
    assert_eq!(iter_submasks(13i32).collect::<Vec<_>>(), vec![13, 12, 9, 8, 5, 4, 1, 0]);

    assert_eq!(
        iter_submasks(2u128.pow(100) + 2u128.pow(127)).collect::<Vec<_>>(),
        vec![2u128.pow(100) + 2u128.pow(127), 2u128.pow(127), 2u128.pow(100), 0u128]
    );

    assert_eq!(
        iter_submasks(u128::MAX).take(5).collect::<Vec<_>>(),
        vec![u128::MAX, u128::MAX - 1, u128::MAX - 2, u128::MAX - 3, u128::MAX - 4]
    );
}

#[test]
fn supermasks() {
    assert_eq!(
        iter_supermasks(0u8).collect::<Vec<_>>(),
        (0..=u8::MAX).collect::<Vec<_>>()
    );

    assert_eq!(
        iter_supermasks(0i8).collect::<Vec<_>>(),
        (0..=u8::MAX).map(|i| i as i8).collect::<Vec<_>>()
    );

    assert_eq!(iter_supermasks(u32::MAX).collect::<Vec<_>>(), vec![u32::MAX]);
    assert_eq!(iter_supermasks(-1i32).collect::<Vec<_>>(), vec![-1i32]);

    let a = 2u128.pow(127);
    let b = 2u128.pow(100);
    let c = 2u128.pow(42);

    assert_eq!(
        iter_supermasks(u128::MAX - a - b - c).collect::<Vec<_>>(),
        vec![
            u128::MAX - a - b - c,
            u128::MAX - a - b,
            u128::MAX - a - c,
            u128::MAX - a,
            u128::MAX - b - c,
            u128::MAX - b,
            u128::MAX - c,
            u128::MAX,
        ]
    );

    assert_eq!(iter_supermasks(0u128).take(5).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);
}
