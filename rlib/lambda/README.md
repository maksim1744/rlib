# Recursive lambda

[![](https://github.com/maksim1744/rlib/actions/workflows/ci.yml/badge.svg)](https://github.com/maksim1744/rlib/tree/main/rlib/lambda/tests) [![](https://img.shields.io/badge/Docs-github--pages-blue)](https://maksim1744.github.io/rlib/rlib_lambda/index.html)

Provides a macro to create recursive local functions without manually passing all parameters.

```rust
use rlib_lambda::rec_lambda;

let mut mem: Vec<u64> = vec![0, 1];
let md: u64 = 10u64.pow(9) + 7;

// first specify a name to use for recursive calls (fib) and a list of captured parameters (md, mem)
let mut fibonacci_mod = rec_lambda!(fib, |md: &u64, mem: &mut Vec<u64>| {
    // then just define a lambda
    |n: usize| -> u64 {
        if n < mem.len() {
            mem[n]
        } else {
            // recursive calls must be through macro
            let res = (fib!(n - 1) + fib!(n - 2)) % md;
            mem.push(res);
            res
        }
    }
});

assert_eq!(fibonacci_mod(0), 0);
assert_eq!(fibonacci_mod(1), 1);
assert_eq!(fibonacci_mod(10), 55);
assert_eq!(fibonacci_mod(100), 687995182);
```

Notice that the syntax inside macro makes a correct AST so autoformatting works.
