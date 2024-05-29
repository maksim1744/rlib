use rlib_lambda::rec_lambda;

#[test]
fn simple() {
    let g: Vec<Vec<usize>> = vec![vec![1], vec![3, 2, 0], vec![1], vec![1]];

    let dfs = rec_lambda!(dfs, |g: &Vec<Vec<usize>>| {
        |v: usize, p: usize| {
            for &k in g[v].iter() {
                if k != p {
                    dfs!(k, v);
                }
            }
        }
    });

    dfs(0, usize::MAX);
}

#[test]
fn mutable() {
    let g: Vec<Vec<usize>> = vec![vec![1], vec![3, 2, 0], vec![1], vec![1]];
    let mut par = vec![usize::MAX; 4];

    let mut dfs = rec_lambda!(dfs, |g: &Vec<Vec<usize>>, par: &mut Vec<usize>| {
        |v: usize, p: usize| {
            par[v] = p;
            for &k in g[v].iter() {
                if k != p {
                    dfs!(k, v);
                }
            }
        }
    });

    dfs(0, usize::MAX);

    assert_eq!(par, vec![usize::MAX, 0, 1, 1]);
}

#[test]
fn returnable() {
    let factorial = rec_lambda!(f, || {
        |n: i64| -> i64 {
            if n == 0 {
                1
            } else {
                n * f!(n - 1)
            }
        }
    });

    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(10), 3628800);
}

#[test]
fn fibonacci_mem() {
    let mut mem: Vec<u64> = vec![0, 1];

    let mut fib = rec_lambda!(f, |mem: &mut Vec<u64>| {
        |n: usize| -> u64 {
            if n < mem.len() {
                mem[n]
            } else {
                let res = f!(n - 1).wrapping_add(f!(n - 2));
                mem.push(res);
                res
            }
        }
    });

    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(10000), 15574651946073070043);
}
