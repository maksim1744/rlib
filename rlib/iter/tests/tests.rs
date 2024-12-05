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

#[test]
fn neighbours_4() {
    assert_eq!(
        iter_neighbours_4(10, 10, 5, 5).collect::<Vec<_>>(),
        vec![(5, 6), (4, 5), (5, 4), (6, 5)]
    );
    assert_eq!(
        iter_neighbours_4(10, 10, 0, 0).collect::<Vec<_>>(),
        vec![(0, 1), (1, 0)]
    );
    assert_eq!(
        iter_neighbours_4(10, 10, 9, 9).collect::<Vec<_>>(),
        vec![(8, 9), (9, 8)]
    );
    assert_eq!(iter_neighbours_4(1, 10, 0, 5).collect::<Vec<_>>(), vec![(0, 6), (0, 4)]);
    assert_eq!(iter_neighbours_4(10, 1, 0, 0).collect::<Vec<_>>(), vec![(1, 0)]);
    assert_eq!(iter_neighbours_4(1, 1, 0, 0).collect::<Vec<_>>(), Vec::new());
}

#[test]
fn neighbours_4d() {
    assert_eq!(
        iter_neighbours_4d(10, 10, 5, 5).collect::<Vec<_>>(),
        vec![(4, 6), (4, 4), (6, 4), (6, 6)]
    );
    assert_eq!(iter_neighbours_4d(10, 10, 0, 0).collect::<Vec<_>>(), vec![(1, 1)]);
    assert_eq!(iter_neighbours_4d(10, 10, 9, 9).collect::<Vec<_>>(), vec![(8, 8)]);
    assert_eq!(iter_neighbours_4d(1, 10, 0, 5).collect::<Vec<_>>(), Vec::new());
    assert_eq!(iter_neighbours_4d(10, 1, 0, 0).collect::<Vec<_>>(), Vec::new());
    assert_eq!(iter_neighbours_4d(1, 1, 0, 0).collect::<Vec<_>>(), Vec::new());
}

#[test]
fn neighbours_8() {
    assert_eq!(
        iter_neighbours_8(10, 10, 5, 5).collect::<Vec<_>>(),
        vec![(5, 6), (4, 6), (4, 5), (4, 4), (5, 4), (6, 4), (6, 5), (6, 6)]
    );
    assert_eq!(
        iter_neighbours_8(10, 10, 0, 0).collect::<Vec<_>>(),
        vec![(0, 1), (1, 0), (1, 1)]
    );
    assert_eq!(
        iter_neighbours_8(10, 10, 9, 9).collect::<Vec<_>>(),
        vec![(8, 9), (8, 8), (9, 8)]
    );
    assert_eq!(iter_neighbours_8(1, 10, 0, 5).collect::<Vec<_>>(), vec![(0, 6), (0, 4)]);
    assert_eq!(iter_neighbours_8(10, 1, 0, 0).collect::<Vec<_>>(), vec![(1, 0)]);
    assert_eq!(iter_neighbours_8(1, 1, 0, 0).collect::<Vec<_>>(), Vec::new());
}

#[test]
fn permutations() {
    assert_eq!(
        iter_permutations(Vec::<i32>::new()).collect::<Vec<_>>(),
        vec![vec![]] as Vec<Vec<i32>>,
    );
    assert_eq!(iter_permutations(vec![6]).collect::<Vec<_>>(), vec![vec![6]]);
    assert_eq!(
        iter_permutations(vec![1, 2, 3]).collect::<Vec<_>>(),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]
    );
    assert_eq!(
        iter_permutations(vec![1, 2, 3]).collect::<Vec<_>>(),
        iter_permutations(vec![3, 2, 1]).collect::<Vec<_>>()
    );
    let mut v = vec![1, 2, 3, 4, 5];
    assert!(next_permutation(&mut v));
    assert_eq!(v, vec![1, 2, 3, 5, 4]);
    let mut v = vec![3, 5, 1, 4, 2];
    assert!(next_permutation(&mut v));
    assert_eq!(v, vec![3, 5, 2, 1, 4]);
    let mut v = vec![5, 4, 3, 2, 1];
    assert!(!next_permutation(&mut v));
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}
