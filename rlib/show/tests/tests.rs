use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use rlib_show::{show_struct, show_struct_debug, Show as _, ShowPretty, ShowSettings};

#[test]
fn ints() {
    let settings = ShowSettings::new();
    assert_eq!(10i32.show(&settings), "10");
    assert_eq!((-10i32).show(&settings), "-10");
    assert_eq!(i32::MAX.show(&settings), "inf");
    assert_eq!(i32::MIN.show(&settings), "-inf");

    assert_eq!(10i8.show(&settings), "10");
    assert_eq!(10u8.show(&settings), "10");
    assert_eq!(10i16.show(&settings), "10");
    assert_eq!(10u16.show(&settings), "10");
    assert_eq!(10i32.show(&settings), "10");
    assert_eq!(10u32.show(&settings), "10");
    assert_eq!(10i64.show(&settings), "10");
    assert_eq!(10u64.show(&settings), "10");
    assert_eq!(10i128.show(&settings), "10");
    assert_eq!(10u128.show(&settings), "10");
    assert_eq!(10isize.show(&settings), "10");
    assert_eq!(10usize.show(&settings), "10");

    assert_eq!(10i64.show(&settings), "10");
    assert_eq!(10i64.pow(10).show(&settings), "10000000000");
    assert_eq!((-10i64.pow(10)).show(&settings), "-10000000000");
    assert_eq!(i64::MAX.show(&settings), "inf");
    assert_eq!(i64::MIN.show(&settings), "-inf");

    assert_eq!(10i128.show(&settings), "10");
    assert_eq!(10i128.pow(20).show(&settings), "100000000000000000000");
    assert_eq!((-10i128.pow(20)).show(&settings), "-100000000000000000000");
    assert_eq!(i128::MAX.show(&settings), "inf");
    assert_eq!(i128::MIN.show(&settings), "-inf");

    let mut settings = ShowSettings::new();
    settings.inf_32 = 10;
    assert_eq!(20i32.show(&settings), "inf");
    assert_eq!((-20i32).show(&settings), "-inf");
    assert_eq!(20i64.show(&settings), "20");
    assert_eq!((-20i64).show(&settings), "-20");
    assert_eq!(20i128.show(&settings), "20");
    assert_eq!((-20i128).show(&settings), "-20");

    let mut settings = ShowSettings::new();
    settings.inf_64 = 10;
    assert_eq!(20i32.show(&settings), "20");
    assert_eq!((-20i32).show(&settings), "-20");
    assert_eq!(20i64.show(&settings), "inf");
    assert_eq!((-20i64).show(&settings), "-inf");
    assert_eq!(20i128.show(&settings), "20");
    assert_eq!((-20i128).show(&settings), "-20");

    let mut settings = ShowSettings::new();
    settings.inf_128 = 10;
    assert_eq!(20i32.show(&settings), "20");
    assert_eq!((-20i32).show(&settings), "-20");
    assert_eq!(20i64.show(&settings), "20");
    assert_eq!((-20i64).show(&settings), "-20");
    assert_eq!(20i128.show(&settings), "inf");
    assert_eq!((-20i128).show(&settings), "-inf");
}

#[test]
fn floats() {
    let settings = ShowSettings::new();
    assert_eq!(1f64.show(&settings), "1.000000000");
    assert_eq!((-1f64).show(&settings), "-1.000000000");
    assert_eq!(10f64.show(&settings), "10.000000000");
    assert_eq!((-10f64).show(&settings), "-10.000000000");
    assert_eq!((1.0f64 / 3.0f64).show(&settings), "0.333333333");

    let mut settings = ShowSettings::new();
    settings.float_precision = 3;
    assert_eq!(1f32.show(&settings), "1.000");
    assert_eq!((-1f32).show(&settings), "-1.000");
    assert_eq!(10f32.show(&settings), "10.000");
    assert_eq!((-10f32).show(&settings), "-10.000");
    assert_eq!((1.0f32 / 3.0f32).show(&settings), "0.333");
}

#[test]
fn strings() {
    let settings = ShowSettings::new();
    let fmt = |a: &str| -> String { format!("\"{}\"", a) };
    let a = String::from("qwerty");
    assert_eq!(a.show(&settings), fmt(&a));
    assert_eq!(a.as_str().show(&settings), fmt(&a));
    assert_eq!("hello".show(&settings), fmt("hello"));
    assert_eq!(a[2..4].show(&settings), fmt("er"));
    assert_eq!(['a', 'b'].show(&settings), "['a', 'b']");
}

#[test]
fn bools() {
    let settings = ShowSettings::new();
    assert_eq!(true.show(&settings), "true");
    assert_eq!(false.show(&settings), "false");
}

#[test]
fn tuples() {
    let settings = ShowSettings::new();
    assert_eq!((1, 2, 3).show(&settings), "(1, 2, 3)");
    assert_eq!((1, "hello", vec![4, 5]).show(&settings), "(1, \"hello\", [4, 5])");
}

#[test]
fn vec() {
    let settings = ShowSettings::new();
    assert_eq!(Vec::<i32>::new().show(&settings), "[]");
    assert_eq!(vec![1].show(&settings), "[1]");
    assert_eq!(vec![1, 2, 3].show(&settings), "[1, 2, 3]");
    assert_eq!([1, 2, 3].show(&settings), "[1, 2, 3]");
    assert_eq!(vec![[1, 2], [3, 4], [5, 6]].show(&settings), "[[1, 2], [3, 4], [5, 6]]");
    assert_eq!(
        vec![[[1, 2], [3, 4]], [[5, 6], [7, 8]]].show(&settings),
        "[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]"
    );
    let a = vec![1, 2, 3, 4, 5];
    let b = &a[2..4];
    assert_eq!(b.show(&settings), "[3, 4]");

    let mut settings = ShowSettings::new();
    settings.item_width = 5;
    assert_eq!(vec![1, 200, 3000000].show(&settings), "[    1,   200, 3000000]");

    let settings = ShowSettings::new();
    assert_eq!(
        vec![vec![1, 2], vec![3, 4, 5]].show_pretty(&settings),
        "[[1, 2],\n [3, 4, 5]]"
    );
    assert_eq!(
        vec![vec![1, 200], vec![3, 4, 5]].show_pretty(&settings),
        "[[1, 200],\n [3,   4, 5]]"
    );
    assert_eq!(
        vec![vec![1, 2], vec![3, 400, 5]].show_pretty(&settings),
        "[[1,   2],\n [3, 400, 5]]"
    );

    let mut settings = ShowSettings::new();
    settings.item_width = 5;
    assert_eq!(
        vec![vec![1, 200], vec![3, 4, 5000000]].show_pretty(&settings),
        "[[    1,   200],\n [    3,     4, 5000000]]"
    );
}

#[test]
fn set() {
    let settings = ShowSettings::new();
    assert_eq!(BTreeSet::<i32>::new().show(&settings), "{}");
    assert_eq!(HashSet::<i32>::new().show(&settings), "{}");
    assert_eq!(BTreeSet::from([1]).show(&settings), "{1}");
    assert_eq!(HashSet::from([1]).show(&settings), "{1}");
    assert_eq!(BTreeSet::from([1, 2, 3]).show(&settings), "{1, 2, 3}");
    assert_eq!(
        BTreeSet::from([vec![1, 2], vec![3, 4]]).show(&settings),
        "{[1, 2], [3, 4]}"
    );
}

#[test]
fn map() {
    let settings = ShowSettings::new();
    assert_eq!(BTreeMap::<i32, i32>::new().show(&settings), "{}");
    assert_eq!(HashMap::<i32, i32>::new().show(&settings), "{}");
    assert_eq!(BTreeMap::from([(1, 2)]).show(&settings), "{(1, 2)}");
    assert_eq!(HashMap::from([(1, 2)]).show(&settings), "{(1, 2)}");
    assert_eq!(
        BTreeMap::from([(1, 1), (2, 4), (3, 9)]).show(&settings),
        "{(1, 1), (2, 4), (3, 9)}"
    );

    assert_eq!(
        BTreeMap::from([(1, 1), (2, 4), (3, 9)]).show_pretty(&settings),
        "{1: 1,\n 2: 4,\n 3: 9}"
    );
    assert_eq!(
        BTreeMap::from([(1, 1), (2, 400), (3, 9)]).show_pretty(&settings),
        "{1: 1  ,\n 2: 400,\n 3: 9  }"
    );
    assert_eq!(
        BTreeMap::from([(1, 1), (200, 4), (3, 9)]).show_pretty(&settings),
        "{1  : 1,\n 3  : 9,\n 200: 4}"
    );

    let mut settings = ShowSettings::new();
    settings.item_width = 5;
    assert_eq!(
        BTreeMap::from([(1, 1), (200, 4), (3, 9)]).show_pretty(&settings),
        "{1    : 1    ,\n 3    : 9    ,\n 200  : 4    }"
    );
}

struct S1 {
    a: i32,
    b: String,
}

struct S2 {
    a: f64,
    s: S1,
}

show_struct!(S1, a, b);
show_struct!(S2, a, s);

#[allow(dead_code)]
#[derive(Debug)]
struct S3 {
    a: i32,
    b: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct S4 {
    a: f64,
    s: S3,
}

show_struct_debug!(S4);

#[test]
fn structs() {
    let settings = ShowSettings::new();
    assert_eq!(
        S1 {
            a: 10,
            b: "hello".to_string()
        }
        .show(&settings),
        "{a: 10, b: \"hello\"}"
    );
    assert_eq!(
        S2 {
            a: -4.2,
            s: S1 {
                a: 10,
                b: "hello".to_string()
            }
        }
        .show(&settings),
        "{a: -4.200000000, s: {a: 10, b: \"hello\"}}"
    );

    let s = S4 {
        a: -4.2,
        s: S3 {
            a: 10,
            b: "hello".to_string(),
        },
    };
    assert_eq!(s.show(&settings), format!("{:?}", s));
}
