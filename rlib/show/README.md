# Show macro

[![](https://github.com/maksim1744/rlib/actions/workflows/ci.yml/badge.svg)](https://github.com/maksim1744/rlib/tree/main/rlib/show/tests) [![](https://img.shields.io/badge/Docs-github--pages-blue)](https://maksim1744.github.io/rlib/rlib_show/index.html)

Provides macro [show!](show) for debug prints. By default does nothing, to enable define environment variable `HOUSE` during compilation. Works for types that implement trait [Show](Show).

```rust
use rlib_show::*;

let a: i32 = 10;
show!(a);  // [line_number]  [a: 10]
let b: i32 = 20;
show!(a, b);  // [line_number]  [a: 10] [b: 20]
show!(a + b);  // [line_number]  [a + b: 30]
let c: i32 = 10i32.pow(9) * 2;
show!(a, b, c);  // [line_number]  [a: 10] [b: 20] [c: inf]

let v = vec![vec![1, 2], vec![3, 4]];
show!(v);  // [line_number]  [v: [[1, 2], [3, 4]]]
```

Some types also implement trait [ShowPretty](ShowPretty) for pretty printing using macro [show_pretty!](show_pretty).

```rust
use rlib_show::*;

let v = vec![vec![1, 2], vec![3, 4]];
show_pretty!(v);  // [line_number]  [v: [[1, 2],
                  //                     [3, 4]]]
let m = std::collections::BTreeMap::from([(1, 2), (3, 4)]);
show_pretty!(v);  // [line_number]  [m: {1: 2,
                  //                     3: 4}]
```

For your own types you can either just implement [Show](Show), or use [show_struct](show_struct) which essentially acts as `derive`, or use [show_struct_debug](show_struct_debug) which uses `Debug` implementation to implement [Show](Show).

```rust
use rlib_show::*;

struct S {
    a: i32,
    b: i64,
}

show_struct!(S, a, b);

#[derive(Debug)]
struct A {
    a: i32,
}

#[derive(Debug)]
struct B {
    b: i32,
}

show_struct_debug!(A, B);
```

To configure how variables of some types are printed, use macro [show_cfg](show_cfg) to edit [settings](ShowSettings).
```rust
use rlib_show::*;

let a: i32 = 20;
show!(a);  // [line_number]  [a: 20]
show_cfg!(inf_32, 10);
show!(a);  // [line_number]  [a: inf]
```
