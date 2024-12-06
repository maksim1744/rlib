use rlib_io::reader::{Readable, Reader};

fn make_reader(s: &str) -> Reader {
    Reader::new(Box::new(std::io::Cursor::new(s.as_bytes().to_vec())))
}

#[test]
fn simple() {
    let mut reader = make_reader("123");
    assert_eq!(reader.read::<i32>(), 123);
}

fn try_read<T: Readable>(s: &str) -> Option<T> {
    std::panic::catch_unwind(|| make_reader(s).read::<T>()).ok()
}

fn read<T: Readable>(s: &str) -> T {
    try_read(s).unwrap()
}

#[test]
#[cfg(debug_assertions)]
fn debug_assertions_in_debug() {
    // debug build should panic on incorrect integer
    assert_eq!(try_read::<i32>("1a3"), None);
    assert_eq!(try_read::<i32>("a"), None);
    assert_eq!(try_read::<i32>(""), None);
    assert_eq!(try_read::<i32>("a"), None);
    assert_eq!(try_read::<i32>("a1"), None);
    assert_eq!(try_read::<i32>("1a"), None);
    assert_eq!(try_read::<i32>("-a"), None);
    assert_eq!(try_read::<i32>("-"), None);
    assert_eq!(try_read::<i32>("--1"), None);
    assert_eq!(try_read::<i32>("2147483648"), None);
    assert_eq!(try_read::<i32>("-2147483649"), None);
    assert_eq!(try_read::<u32>("-1"), None);
    assert_eq!(try_read::<char>(""), None);
}

#[test]
#[cfg(not(debug_assertions))]
fn debug_assertions_in_release() {
    // release build should read some trash value
    assert_eq!(read::<i32>("1a3"), 593);
    assert_eq!(read::<i32>("a"), 49);
    assert_eq!(read::<i32>(""), 0);
    assert_eq!(read::<i32>("a"), 49);
    assert_eq!(read::<i32>("a1"), 491);
    assert_eq!(read::<i32>("1a"), 59);
    assert_eq!(read::<i32>("-a"), -49);
    assert_eq!(read::<i32>("-"), 0);
    assert_eq!(read::<i32>("--1"), -2531); // - ((-3 as u8)*10 + 1)
    assert_eq!(read::<i32>("2147483648"), -2147483648); // overflow
    assert_eq!(read::<i32>("-2147483649"), 2147483647);
    assert_eq!(read::<u32>("-1"), 2531);
}

macro_rules! test_signed {
    ($name:ident, $t:ty, $min:expr, $max:expr) => {
        #[test]
        fn $name() {
            assert_eq!(read::<$t>("0"), 0 as $t);
            assert_eq!(read::<$t>("-0"), 0 as $t);
            assert_eq!(read::<$t>("42"), 42 as $t);
            assert_eq!(read::<$t>("-42"), -42 as $t);
            assert_eq!(read::<$t>($min), <$t>::MIN);
            assert_eq!(read::<$t>($max), <$t>::MAX);
        }
    };
}

macro_rules! test_unsigned {
    ($name:ident, $t:ty, $max:expr) => {
        #[test]
        fn $name() {
            assert_eq!(read::<$t>("0"), 0 as $t);
            assert_eq!(read::<$t>("42"), 42 as $t);
            assert_eq!(read::<$t>($max), <$t>::MAX);
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

#[cfg(target_pointer_width = "32")]
test_signed!(test_isize, isize, "-2147483648", "2147483647");
#[cfg(target_pointer_width = "64")]
test_signed!(test_isize, isize, "-9223372036854775808", "9223372036854775807");
#[cfg(target_pointer_width = "32")]
test_unsigned!(test_usize, usize, "4294967295");
#[cfg(target_pointer_width = "64")]
test_unsigned!(test_usize, usize, "18446744073709551615");

#[test]
fn vec() {
    assert_eq!(make_reader("1 20 -3 0 abc").read_vec::<i32>(4), vec![1, 20, -3, 0]);
    assert_eq!(
        make_reader("  \n\n\t1\n20\t-3 \n\n  \t \r\n   0").read_vec::<i32>(4),
        vec![1, 20, -3, 0]
    );
}

#[test]
fn tuple() {
    assert_eq!(read::<(i8, u8)>("-15 222"), (-15, 222));
    // up to 8 items
    assert_eq!(
        read::<(i32, i32, i32, i32, i32, i32, i32, i32)>("1 2 3 4 5 6 7 8"),
        (1, 2, 3, 4, 5, 6, 7, 8)
    );
}

#[test]
fn char() {
    assert_eq!(read::<char>("  x  "), 'x');
}

#[test]
fn string() {
    #[cfg(debug_assertions)]
    assert_eq!(try_read::<String>(""), None);
    #[cfg(not(debug_assertions))]
    assert_eq!(try_read::<String>(""), Some("".to_string()));

    assert_eq!(read::<String>("hello"), "hello".to_string());
    let s = ".,-=`'/]\\qwerty123!@#$%^&*()\"";
    assert_eq!(read::<String>(s), s.to_string());

    // large string
    let s = (0..10_000_000)
        .map(|i| (i % 23 + 'a' as i32) as u8 as char)
        .collect::<String>();
    assert!(read::<String>(&s) == s.clone());

    let s2 = ["123\t", &s, "\n456"].concat();
    assert!(read::<(i32, String, i32)>(&s2) == (123, s, 456));
}

#[test]
fn read_line() {
    let mut reader = make_reader(" 123 \n 456 \n789\n\n0");
    assert_eq!(reader.read_lines(), vec![" 123 ", " 456 ", "789", "", "0"]);
    let mut reader = make_reader("\n\n 123 \n 456 \n789\n\n0\n\n");
    assert_eq!(reader.read_lines(), vec!["", "", " 123 ", " 456 ", "789", "", "0", ""]);

    let mut reader = make_reader(" 123 \r\n 456 \r\n789\r\n\r\n0");
    assert_eq!(reader.read_lines(), vec![" 123 ", " 456 ", "789", "", "0"]);
    let mut reader = make_reader("\r\n\r\n 123 \r\n 456 \r\n789\r\n\r\n0\r\n\r\n");
    assert_eq!(reader.read_lines(), vec!["", "", " 123 ", " 456 ", "789", "", "0", ""]);
}
