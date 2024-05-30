pub const VALS_TO_CHECK: [i32; 20] = [
    1, 5, 10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000, 500000, 1000000, 5000000, 10000000, 50000000, 100000000,
    500000000, 1000000000, 2000000000,
];

/// For each pair of (A, B) from [VALS_TO_CHECK] this array stores (more or less maximum possible) value L such that multiplying
/// array a with values [0..=A] of len L with array b with values [0..=B] of len L will give correct result.
#[rustfmt::skip]
pub const CORRECT_F64_BOUNDS: [[f64; VALS_TO_CHECK.len()]; VALS_TO_CHECK.len()] = [
    //            1,   5,  10,  50, 100, 500, 1e3, 5e3, 1e4, 5e4, 1e5, 5e5, 1e6, 5e6, 1e7, 5e7, 1e8, 5e8, 1e9, 2e9
    /*   1 */  [5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e5, 1e5, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /*   5 */  [5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e5, 1e5, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /*  10 */  [5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e5, 1e5, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /*  50 */  [5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e5, 1e5, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 100 */  [5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e5, 1e5, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 500 */  [5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e5, 1e5, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 1e3 */  [5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e5, 1e5, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 5e3 */  [5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 2e6, 2e5, 1e5, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 1e4 */  [5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 5e6, 2e6, 1e6, 2e5, 1e4, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 5e4 */  [5e5, 5e5, 5e5, 5e5, 5e5, 5e5, 5e5, 3e5, 2e5, 1e4, 1e4, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 1e5 */  [1e5, 1e5, 1e5, 1e5, 1e5, 1e5, 1e5, 1e5, 1e5, 1e4, 1e4, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 5e5 */  [1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e2, 1e1, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 1e6 */  [1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e3, 1e2, 1e2, 1e1, 1e1,  0.,  0.,  0.,  0.,  0.],
    /* 5e6 */  [1e2, 1e2, 1e2, 1e2, 1e2, 1e2, 1e2, 1e2, 1e2, 1e2, 1e2, 1e2, 1e1, 1e1,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e7 */  [1e1, 1e1, 1e1, 1e1, 1e1, 1e1, 1e1, 1e1, 1e1, 1e1, 1e1, 1e1, 1e1, 1e1,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 5e7 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e8 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 5e8 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e9 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 2e9 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
];

#[rustfmt::skip]
pub const CORRECT_F32_BOUNDS: [[f64; VALS_TO_CHECK.len()]; VALS_TO_CHECK.len()] = [
    //            1,   5,  10,  50, 100, 500, 1e3, 5e3, 1e4, 5e4, 1e5, 5e5, 1e6, 5e6, 1e7, 5e7, 1e8, 5e8, 1e9, 2e9
    /*   1 */  [2e5, 1e4, 1e4, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /*   5 */  [1e4, 1e4, 1e3, 1e3, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /*  10 */  [1e4, 1e3, 1e3, 1e2, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /*  50 */  [1e3, 1e3, 1e2, 1e2, 1e2, 1e1,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 100 */  [1e2, 1e2, 1e2, 1e2, 1e1, 1e1,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 500 */  [1e1, 1e1, 1e1, 1e1, 1e1,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e3 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 5e3 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e4 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 5e4 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e5 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 5e5 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e6 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 5e6 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e7 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 5e7 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e8 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 5e8 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 1e9 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
    /* 2e9 */  [ 0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.,  0.],
];
