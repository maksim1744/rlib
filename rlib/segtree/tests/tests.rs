use rlib_rand::*;
use rlib_segtree::*;

const SIZES: [usize; 17] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 31, 32, 33, 100, 127, 128, 129, 1];
const TOTAL_ITS: usize = 1_000_000;

fn gen_lr(rng: &mut Rng, n: usize) -> (usize, usize) {
    let l = rng.next(0..n);
    let r = rng.next(0..n);
    if l > r {
        (r, l)
    } else {
        (l, r)
    }
}

#[test]
fn min() {
    let mut rng = Rng::from_seed(42);
    type Min = segtree_items::Min<i32>;

    for &n in SIZES.iter() {
        let its = TOTAL_ITS / n; // each iteration will take O(n) in bruteforce

        let mut tree = Segtree::new_raw(n, Min::new(0));
        let mut ar: Vec<i32> = vec![0; n];

        for _ in 0..its {
            let tp = rng.next(0..4);
            if tp == 0 {
                let ind = rng.next(0..n);
                let val = rng.next(i32::MIN..i32::MAX);
                ar[ind] = val;
                tree.set(ind, Min::new(val));
            } else if tp == 1 {
                let (l, r) = gen_lr(&mut rng, n);
                let correct = *ar[l..r + 1].iter().min().unwrap();
                let result = tree.ask(l, r).v;
                assert_eq!(correct, result);
            } else if tp == 2 {
                tree = Segtree::from_slice(&ar.iter().map(|&x| Min::new(x)).collect::<Vec<_>>());
            } else if tp == 3 {
                tree = Segtree::from_iter(ar.iter().map(|&x| Min::new(x)));
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn max() {
    let mut rng = Rng::from_seed(42);
    type Max = segtree_items::Max<i32>;

    for &n in SIZES.iter() {
        let its = TOTAL_ITS / n; // each iteration will take O(n) in bruteforce

        let mut tree = Segtree::new_raw(n, Max::new(0));
        let mut ar: Vec<i32> = vec![0; n];

        for _ in 0..its {
            let tp = rng.next(0..4);
            if tp == 0 {
                let ind = rng.next(0..n);
                let val = rng.next(i32::MIN..i32::MAX);
                ar[ind] = val;
                tree.set(ind, Max::new(val));
            } else if tp == 1 {
                let (l, r) = gen_lr(&mut rng, n);
                let correct = *ar[l..r + 1].iter().max().unwrap();
                let result = tree.ask(l, r).v;
                assert_eq!(correct, result);
            } else if tp == 2 {
                tree = Segtree::from_slice(&ar.iter().map(|&x| Max::new(x)).collect::<Vec<_>>());
            } else if tp == 3 {
                tree = Segtree::from_iter(ar.iter().map(|&x| Max::new(x)));
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn sum() {
    let mut rng = Rng::from_seed(42);
    type Sum = segtree_items::Sum<i32>;

    for &n in SIZES.iter() {
        let its = TOTAL_ITS / n; // each iteration will take O(n) in bruteforce

        let mut tree = Segtree::new_raw(n, Sum::new(0));
        let mut ar: Vec<i32> = vec![0; n];

        for _ in 0..its {
            let tp = rng.next(0..4);
            if tp == 0 {
                let ind = rng.next(0..n);
                let val = rng.next(i32::MIN / n as i32..i32::MAX / n as i32);
                ar[ind] = val;
                tree.set(ind, Sum::new(val));
            } else if tp == 1 {
                let (l, r) = gen_lr(&mut rng, n);
                let correct = ar[l..r + 1].iter().sum::<i32>();
                let result = tree.ask(l, r).v;
                assert_eq!(correct, result);
            } else if tp == 2 {
                tree = Segtree::from_slice(&ar.iter().map(|&x| Sum::new(x)).collect::<Vec<_>>());
            } else if tp == 3 {
                tree = Segtree::from_iter(ar.iter().map(|&x| Sum::new(x)));
            } else {
                assert!(false);
            }
        }
    }
}

#[derive(Clone)]
struct StringSum {
    s: String,
}

impl StringSum {
    fn new(c: char) -> Self {
        Self { s: c.to_string() }
    }
}

impl SegtreeItem for StringSum {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            s: [left.s.clone(), right.s.clone()].concat(),
        }
    }
}

#[test]
fn substring() {
    let mut rng = Rng::from_seed(42);

    for &n in SIZES.iter() {
        let its = TOTAL_ITS / 10 / n;

        let mut tree = Segtree::new(n, StringSum::new('a'));
        let mut ar: Vec<String> = vec!['a'.to_string(); n];

        for _ in 0..its {
            if rng.next(0..2) == 0 {
                let ind = rng.next(0..n);
                let val = char::from_u32(rng.next(0..26 as u32) + ('a' as u32)).unwrap();
                ar[ind] = val.to_string();
                tree.set(ind, StringSum::new(val));
            } else {
                let (l, r) = gen_lr(&mut rng, n);
                let correct = ar[l..r + 1].iter().cloned().reduce(|a, b| [a, b].concat()).unwrap();
                let result = tree.ask(l, r).s;
                assert_eq!(correct, result);
            }
        }
    }
}

#[derive(Clone)]
struct SumAdd {
    len: i64,
    sum: i64,
    md: i64,
}

impl SumAdd {
    fn new(x: i64) -> Self {
        Self { len: 1, sum: x, md: 0 }
    }
}

impl SegtreeItem for SumAdd {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            len: left.len + right.len,
            sum: left.sum + right.sum,
            md: 0,
        }
    }
}

impl SegtreeItemLazy<i64> for SumAdd {
    fn modify(&mut self, md: &i64) {
        self.sum += md * self.len;
        self.md += md;
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        left.modify(&self.md);
        right.modify(&self.md);
        self.md = 0;
    }
}

#[test]
fn lazy_sum() {
    let mut rng = Rng::from_seed(42);

    for &n in SIZES.iter() {
        let its = TOTAL_ITS / n; // each iteration will take O(n) in bruteforce

        let mut tree = Segtree::new(n, SumAdd::new(0));
        let mut ar: Vec<i64> = vec![0; n];

        for _ in 0..its {
            let tp = rng.next(0..4);
            if tp == 0 {
                let ind = rng.next(0..n);
                let val = rng.next(i32::MIN / n as i32..i32::MAX / n as i32) as i64;
                ar[ind] = val;
                tree.set(ind, SumAdd::new(val));
            } else if tp == 1 {
                let (l, r) = gen_lr(&mut rng, n);
                let correct = ar[l..r + 1].iter().sum::<i64>();
                let result = tree.ask(l, r).sum;
                assert_eq!(correct, result);
            } else if tp == 2 {
                tree = Segtree::from_slice(&ar.iter().map(|&x| SumAdd::new(x)).collect::<Vec<_>>());
            } else if tp == 3 {
                let (l, r) = gen_lr(&mut rng, n);
                let val = rng.next(i32::MIN / n as i32..i32::MAX / n as i32) as i64;
                for i in l..=r {
                    ar[i] += val;
                }
                tree.modify(l, r, &val);
            } else {
                assert!(false);
            }
        }
    }
}
