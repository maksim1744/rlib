use rlib_rand::Rng;
use rlib_treap::*;

struct ItemSized {
    x: u32,
    sm: u32,
    md: u32,
    sz: usize,
}

impl ItemSized {
    fn new(x: u32) -> Self {
        Self { x, sm: x, md: 0, sz: 1 }
    }

    fn modify(&mut self, m: u32) {
        self.md = self.md.wrapping_add(m);
        self.x = self.x.wrapping_add(m);
        self.sm = self.sm.wrapping_add(m.wrapping_mul(self.sz as u32));
    }
}

impl TreapItem for ItemSized {
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.sm = left
            .map(|i| i.sm)
            .unwrap_or(0)
            .wrapping_add(right.map(|i| i.sm).unwrap_or(0))
            .wrapping_add(self.x);
        self.sz = left.map(|i| i.sz).unwrap_or(0) + right.map(|i| i.sz).unwrap_or(0) + 1;
    }

    fn push(&mut self, left: Option<&mut Self>, right: Option<&mut Self>) {
        if let Some(left) = left {
            left.modify(self.md);
        }
        if let Some(right) = right {
            right.modify(self.md);
        }
        self.md = 0;
    }
}

impl TreapItemSized for ItemSized {
    fn size(&self) -> usize {
        self.sz
    }
}

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
fn item_sized() {
    let mut rng = Rng::from_seed(42);

    let mut a: Vec<u32> = Vec::new();
    let mut t: Treap<ItemSized> = Treap::new();

    const ITS: usize = 50000;
    for it in 0..ITS {
        let tp: i32 = rng.next(1..=8);

        if tp == 1 {
            assert_eq!(t.first().map(|i| &i.x), a.first());
        } else if tp == 2 {
            assert_eq!(t.last().map(|i| &i.x), a.last());
        } else if tp == 3 {
            let pos = rng.next(0..=a.len());
            let (l, r) = t.split_at(pos);
            assert_eq!(l.size(), pos);
            assert_eq!(r.size(), a.len() - pos);
            t = Treap::merge(r, l);
            a = a[pos..].iter().chain(a[..pos].iter()).cloned().collect();
        } else if tp == 4 && it > 100 {
            // first check everything else for empty treap
            let pos = rng.next(0..=a.len());
            let val = rng.next_raw() as u32;
            t.insert_at(pos, ItemSized::new(val));
            a.insert(pos, val);
        } else if tp == 5 {
            assert_eq!(t.collect().into_iter().map(|i| i.x).collect::<Vec<u32>>(), a);
        } else if tp == 6 && !a.is_empty() {
            let (l, r) = gen_lr(&mut rng, a.len());
            let (t12, t3) = t.split_at(r + 1);
            let (t1, mut t2) = t12.split_at(l);
            let val = rng.next_raw() as u32;
            t2.root_mut().unwrap().modify(val);
            for i in l..=r {
                a[i] = a[i].wrapping_add(val);
            }
            t = Treap::merge(t1, Treap::merge(t2, t3));
        } else if tp == 7 && !a.is_empty() {
            let (l, r) = gen_lr(&mut rng, a.len());
            let (t12, t3) = t.split_at(r + 1);
            let (t1, t2) = t12.split_at(l);
            assert_eq!(
                a[l..=r].iter().fold(0u32, |a, &b| a.wrapping_add(b)),
                t2.root().unwrap().sm
            );
            t = Treap::merge(t1, Treap::merge(t2, t3));
        } else if tp == 8 && !a.is_empty() && rng.next(..25) == 0 {
            let pos = rng.next(..a.len());
            assert_eq!(t.remove_at(pos).x, a.remove(pos));
        }

        assert_eq!(t.size(), a.len());
        assert_eq!(t.is_empty(), a.is_empty());
    }
}

struct Item {
    x: u8,
    sz: usize,
}

impl Item {
    fn new(x: u8) -> Self {
        Self { x, sz: 1 }
    }
}

impl TreapItem for Item {
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.sz = left.map(|i| i.sz).unwrap_or(0) + right.map(|i| i.sz).unwrap_or(0) + 1;
    }
}

impl TreapItemSized for Item {
    fn size(&self) -> usize {
        self.sz
    }
}

#[test]
fn set() {
    let mut rng = Rng::from_seed(42);

    let mut a: Vec<u8> = Vec::new();
    let mut t: Treap<Item> = Treap::new();

    const ITS: usize = 50000;
    for it in 0..ITS {
        let tp: i32 = rng.next(1..=5);

        if tp == 1 {
            assert_eq!(t.first().map(|i| &i.x), a.first());
        } else if tp == 2 {
            assert_eq!(t.last().map(|i| &i.x), a.last());
        } else if tp == 3 && it > 100 {
            // first check everything else for empty treap
            let val = rng.next_raw() as u8;
            a.push(val);
            a.sort();
            let (l, r) = t.split_by(|i| i.x < val);
            t = Treap::merge(l, Treap::merge(Treap::from_item(Item::new(val)), r));
        } else if tp == 4 {
            assert_eq!(t.collect().into_iter().map(|i| i.x).collect::<Vec<u8>>(), a);
        } else if tp == 5 && !a.is_empty() && rng.next(..25) == 0 {
            let pos = rng.next(..a.len());
            let val = a.remove(pos);
            let (l, mr) = t.split_by(|i| i.x < val);
            let (m, r) = mr.split_at(1);
            assert_eq!(m.root().unwrap().x, val);
            t = Treap::merge(l, r);
        }

        assert_eq!(t.size(), a.len());
        assert_eq!(t.is_empty(), a.is_empty());
    }
}
