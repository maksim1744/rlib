use rlib_num_traits::*;

#[test]
fn base_10_len() {
    assert_eq!(i8::BASE_10_LEN, 3);
    assert_eq!(u8::BASE_10_LEN, 3);
    assert_eq!(i16::BASE_10_LEN, 5);
    assert_eq!(u16::BASE_10_LEN, 5);
    assert_eq!(i32::BASE_10_LEN, 10);
    assert_eq!(u32::BASE_10_LEN, 10);
    assert_eq!(i64::BASE_10_LEN, 20);
    assert_eq!(u64::BASE_10_LEN, 20);
    assert_eq!(i128::BASE_10_LEN, 39);
    assert_eq!(u128::BASE_10_LEN, 39);

    #[cfg(target_pointer_width = "32")]
    assert_eq!(isize::BASE_10_LEN, i32::BASE_10_LEN);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(isize::BASE_10_LEN, i64::BASE_10_LEN);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(usize::BASE_10_LEN, u32::BASE_10_LEN);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(usize::BASE_10_LEN, u64::BASE_10_LEN);
}

#[test]
fn abs() {
    assert_eq!((-128_i8).unsigned_abs(), 128_u8);
    assert_eq!((-42_i8).unsigned_abs(), 42_u8);
    assert_eq!(42_i8.unsigned_abs(), 42_u8);
    assert_eq!(127_i8.unsigned_abs(), 127_u8);

    assert_eq!(0_u8.unsigned_abs(), 0_u8);
    assert_eq!(42_u8.unsigned_abs(), 42_u8);
    assert_eq!(255_u8.unsigned_abs(), 255_u8);
}
