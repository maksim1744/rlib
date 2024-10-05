# Segment tree

[![](https://github.com/maksim1744/rlib/actions/workflows/ci.yml/badge.svg)](https://github.com/maksim1744/rlib/tree/main/rlib/segtree/tests) [![](https://img.shields.io/badge/Docs-github--pages-blue)](https://maksim1744.github.io/rlib/rlib_segtree/index.html)

## Implementing item for segtree

```rust
use rlib_segtree::{Segtree, SegtreeItem};

#[derive(Default, Clone)]
struct Item {
    x: i32,
}

impl SegtreeItem for Item {
    fn merge(left: &Self, right: &Self) -> Self {
        Self { x: left.x + right.x }
    }
}

let a = vec![1, 2, 3, 4, 5];
let mut tree = Segtree::from_iter(a.into_iter().map(|x| Item { x }));
assert_eq!(tree.ask(0, 2).x, 6); // ranges are inclusive: [l; r]
tree.set(1, Item { x: 10 });
assert_eq!(tree.ask(0, 2).x, 14);
```

There are also default items for [sum](https://maksim1744.github.io/rlib/rlib_segtree/segtree_items/struct.Sum.html), [min](https://maksim1744.github.io/rlib/rlib_segtree/segtree_items/struct.Min.html) and [max](https://maksim1744.github.io/rlib/rlib_segtree/segtree_items/struct.Max.html).

## Implementing item for lazy segtree

```rust
use rlib_segtree::{Segtree, SegtreeItem};

#[derive(Default, Clone)]
struct Item {
    x: i32,
    md: i16,
}

impl SegtreeItem<i16> for Item {
    fn merge(left: &Self, right: &Self) -> Self {
        Self { x: left.x.max(right.x), md: 0 }
    }

    fn modify(&mut self, modifier: &i16) {
        self.x += (*modifier) as i32;
        self.md += modifier;
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        left.modify(&self.md);
        right.modify(&self.md);
        self.md = 0;
    }
}

let a = vec![3, 1, 2, 4, 5];
let mut tree = Segtree::from_iter(a.into_iter().map(|x| Item { x, md: 0 }));
assert_eq!(tree.ask(0, 2).x, 3);
tree.modify(1, 3, &100);
assert_eq!(tree.ask(0, 2).x, 102);
assert_eq!(tree.ask(0, 4).x, 104);
```

There are also default items for the same set of queries as above plus addition on a segment: [SumAdd](https://maksim1744.github.io/rlib/rlib_segtree/segtree_items/struct.SumAdd.html), [MinAdd](https://maksim1744.github.io/rlib/rlib_segtree/segtree_items/struct.MinAdd.html) and [MaxAdd](https://maksim1744.github.io/rlib/rlib_segtree/segtree_items/struct.MaxAdd.html).

## Combining two items

There is [Combinator](https://maksim1744.github.io/rlib/rlib_segtree/segtree_items/struct.Combinator.html) which allows for combining two [SegtreeItem](https://maksim1744.github.io/rlib/rlib_segtree/segtree/trait.SegtreeItem.html) items into one.

```rust
use rlib_segtree::Segtree;
use rlib_segtree::segtree_items::{Combinator, MaxAdd, MinAdd};

type MinMax = Combinator<MinAdd<i32>, MaxAdd<i32>>;

let a = vec![3, 1, 2, 4, 5];
let mut tree = Segtree::from_iter(a.into_iter().map(MinMax::from)); // note that `from` is inherited from `MinAdd` and `MaxAdd`
let it = tree.ask(0, 2);
assert_eq!(it.0.v, 1);
assert_eq!(it.1.v, 3);
tree.modify(1, 4, &100);
let it = tree.ask(0, 4);
assert_eq!(it.0.v, 3);
assert_eq!(it.1.v, 105);
```

Note that one combinator can be used as a part for another combinator and so on. However, especially for lazy segtrees it comes at a cost of storing a separate modificator for each struct.
