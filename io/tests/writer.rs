use rlib_io::writer::{Writable, Writer};

fn write<T: Writable>(t: T) -> String {
    let mut v = Vec::new();
    let mut writer = Writer::new(Box::new(&mut v));
    writer.write(&t);
    drop(writer);
    std::str::from_utf8(&v).unwrap().to_string()
}

#[test]
fn simple() {
    assert_eq!(&write(123), "123");
}

macro_rules! test_signed {
    ($name:ident, $t:ty, $min:expr, $max:expr) => {
        #[test]
        fn $name() {
            assert_eq!(&write(0 as $t), "0");
            assert_eq!(&write(42 as $t), "42");
            assert_eq!(&write(-42 as $t), "-42");
            assert_eq!(&write(<$t>::MIN), $min);
            assert_eq!(&write(<$t>::MAX), $max);
        }
    };
}

macro_rules! test_unsigned {
    ($name:ident, $t:ty, $max:expr) => {
        #[test]
        fn $name() {
            assert_eq!(&write(0 as $t), "0");
            assert_eq!(&write(42 as $t), "42");
            assert_eq!(&write(<$t>::MAX), $max);
        }
    };
}

test_signed!(test_i8, i8, "-128", "127");
test_signed!(test_i16, i16, "-32768", "32767");
test_signed!(test_i32, i32, "-2147483648", "2147483647");
test_signed!(test_i64, i64, "-9223372036854775808", "9223372036854775807");
test_signed!(
    test_i128,
    i128,
    "-170141183460469231731687303715884105728",
    "170141183460469231731687303715884105727"
);

test_unsigned!(test_u8, u8, "255");
test_unsigned!(test_u16, u16, "65535");
test_unsigned!(test_u32, u32, "4294967295");
test_unsigned!(test_u64, u64, "18446744073709551615");
test_unsigned!(test_u128, u128, "340282366920938463463374607431768211455");

#[test]
fn vec() {
    assert_eq!(&write(vec![1, 2, -5, 20]), "1 2 -5 20");
}

#[test]
fn tuple() {
    assert_eq!(&write((200 as u8, "hello", -111 as i8)), "200 hello -111");
}

#[test]
fn string() {
    assert_eq!(&write("hello  world"), "hello  world");

    let s = ".,-=`'/]\\qwerty123!@#$%^&*()\"";
    assert_eq!(write(s.clone()), s);

    // large string
    let s = (0..10_000_000)
        .map(|i| (i % 23 + 'a' as i32) as u8 as char)
        .collect::<String>();
    assert!(write(s.clone()) == s);

    let s2 = ["123 ", &s, " 456"].concat();
    assert!(write((123, s, 456)) == s2);
}
