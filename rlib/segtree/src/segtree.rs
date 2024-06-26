use std::fmt::Debug;

pub trait SegtreeItem<M = ()>: Sized {
    fn merge(left: &Self, right: &Self) -> Self;

    fn update(&mut self, left: &Self, right: &Self) {
        *self = Self::merge(left, right);
    }

    fn modify(&mut self, _modifier: &M) {}

    fn push(&mut self, _left: &mut Self, _right: &mut Self) {}
}

pub struct Segtree<T, M> {
    n: usize,
    data: Vec<T>,
    phantom: std::marker::PhantomData<M>,
}

impl<M, T: SegtreeItem<M> + Clone> Segtree<T, M> {
    pub fn new_raw(n: usize, value: T) -> Self {
        assert!(n != 0);
        let mut p2: usize = 1;
        while p2 < n {
            p2 *= 2;
        }
        Self {
            n,
            data: vec![value; p2 * 2],
            phantom: std::marker::PhantomData,
        }
    }

    pub fn new(n: usize, value: T) -> Self {
        let mut res = Self::new_raw(n, value);
        res.rebuild_empty(0, 0, res.n - 1);
        res
    }

    pub fn from_slice(data: &[T]) -> Self {
        let mut res = Self::new_raw(data.len(), data[0].clone());
        res.rebuild(0, 0, res.n - 1, &mut data.iter().cloned());
        res
    }

    // this requires ExactSizeIterator unlike std::iter::FromIterator
    #[allow(clippy::should_implement_trait)]
    pub fn from_iter<I>(mut iter: I) -> Self
    where
        I: std::iter::Iterator<Item = T> + std::iter::ExactSizeIterator,
        T: Default,
    {
        let mut res = Self::new_raw(iter.len(), T::default());
        res.rebuild(0, 0, res.n - 1, &mut iter);
        res
    }

    fn rebuild(&mut self, i: usize, l: usize, r: usize, data: &mut impl std::iter::Iterator<Item = T>) {
        if l == r {
            self.data[i] = data.next().unwrap();
            return;
        }
        let m = (l + r) / 2;
        self.rebuild(i * 2 + 1, l, m, data);
        self.rebuild(i * 2 + 2, m + 1, r, data);

        self.merge_at(i);
    }

    fn rebuild_empty(&mut self, i: usize, l: usize, r: usize) {
        if l == r {
            return;
        }
        let m = (l + r) / 2;
        self.rebuild_empty(i * 2 + 1, l, m);
        self.rebuild_empty(i * 2 + 2, m + 1, r);

        let (left, right) = self.data.split_at_mut(i * 2 + 1);
        left[i].update(&right[0], &right[1]);
    }

    pub fn set(&mut self, ind: usize, value: T) {
        assert!(ind < self.n);
        self.set_internal(ind, value, 0, 0, self.n - 1);
    }

    fn set_internal(&mut self, ind: usize, value: T, i: usize, vl: usize, vr: usize) {
        if vl == vr {
            self.data[i] = value;
            return;
        }
        self.push_at(i);

        let m = (vl + vr) / 2;
        if ind <= m {
            self.set_internal(ind, value, i * 2 + 1, vl, m);
        } else {
            self.set_internal(ind, value, i * 2 + 2, m + 1, vr);
        }

        self.merge_at(i);
    }

    pub fn ask(&mut self, l: usize, r: usize) -> T {
        assert!(l <= r);
        assert!(r < self.n);
        self.ask_internal(l, r, 0, 0, self.n - 1)
    }

    fn ask_internal(&mut self, l: usize, r: usize, i: usize, vl: usize, vr: usize) -> T {
        if l == vl && r == vr {
            return self.data[i].clone();
        }
        self.push_at(i);

        let m = (vl + vr) / 2;
        if r <= m {
            self.ask_internal(l, r, i * 2 + 1, vl, m)
        } else if l > m {
            self.ask_internal(l, r, i * 2 + 2, m + 1, vr)
        } else {
            T::merge(
                &self.ask_internal(l, m, i * 2 + 1, vl, m),
                &self.ask_internal(m + 1, r, i * 2 + 2, m + 1, vr),
            )
        }
    }

    pub fn modify(&mut self, l: usize, r: usize, md: &M) {
        assert!(l <= r);
        assert!(r < self.n);
        self.modify_internal(l, r, md, 0, 0, self.n - 1)
    }

    fn modify_internal(&mut self, l: usize, r: usize, md: &M, i: usize, vl: usize, vr: usize) {
        if l == vl && r == vr {
            self.data[i].modify(md);
            return;
        }
        self.push_at(i);

        let m = (vl + vr) / 2;
        if r <= m {
            self.modify_internal(l, r, md, i * 2 + 1, vl, m);
        } else if l > m {
            self.modify_internal(l, r, md, i * 2 + 2, m + 1, vr);
        } else {
            self.modify_internal(l, m, md, i * 2 + 1, vl, m);
            self.modify_internal(m + 1, r, md, i * 2 + 2, m + 1, vr);
        }

        self.merge_at(i);
    }

    fn push_at(&mut self, i: usize) {
        let (left, right) = self.data.split_at_mut(i * 2 + 1);
        let (r1, r2) = right.split_at_mut(1);
        left[i].push(&mut r1[0], &mut r2[0]);
    }

    fn merge_at(&mut self, i: usize) {
        let (left, right) = self.data.split_at_mut(i * 2 + 1);
        left[i].update(&right[0], &right[1]);
    }
}

impl<M, T: SegtreeItem<M> + Clone + Default> Segtree<T, M> {
    /// Returns smallest r from `[l; n-1]` such that `f(ask(l, r)) == true`, or None if it's always false
    pub fn lower_bound<F>(&mut self, l: usize, f: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        self.lower_bound_internal(T::default(), &f, l, self.n - 1, 0, 0, self.n - 1)
            .1
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_bound_internal<F>(
        &mut self,
        mut item: T,
        f: &F,
        l: usize,
        r: usize,
        i: usize,
        vl: usize,
        vr: usize,
    ) -> (T, Option<usize>)
    where
        F: Fn(&T) -> bool,
    {
        if l == vl && r == vr {
            let next = T::merge(&item, &self.data[i]);
            if !f(&next) {
                return (next, None);
            }
            if vl == vr {
                return (next, Some(vl));
            }
        }
        self.push_at(i);

        let m = (vl + vr) / 2;
        if l <= m {
            let (left_item, left_res) = self.lower_bound_internal(item, f, l, m, i * 2 + 1, vl, m);
            if left_res.is_some() {
                return (left_item, left_res);
            }
            item = left_item;
        }
        self.lower_bound_internal(item, f, l.max(m + 1), r, i * 2 + 2, m + 1, vr)
    }
    /// Returns largest l from `[0; r]` such that `f(ask(l, r)) == true`, or None if it's always false
    pub fn lower_bound_rev<F>(&mut self, r: usize, f: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        self.lower_bound_rev_internal(T::default(), &f, 0, r, 0, 0, self.n - 1)
            .1
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_bound_rev_internal<F>(
        &mut self,
        mut item: T,
        f: &F,
        l: usize,
        r: usize,
        i: usize,
        vl: usize,
        vr: usize,
    ) -> (T, Option<usize>)
    where
        F: Fn(&T) -> bool,
    {
        if l == vl && r == vr {
            let next = T::merge(&self.data[i], &item);
            if !f(&next) {
                return (next, None);
            }
            if vl == vr {
                return (next, Some(vl));
            }
        }
        self.push_at(i);

        let m = (vl + vr) / 2;
        if r > m {
            let (right_item, right_res) = self.lower_bound_rev_internal(item, f, m + 1, r, i * 2 + 2, m + 1, vr);
            if right_res.is_some() {
                return (right_item, right_res);
            }
            item = right_item;
        }
        self.lower_bound_rev_internal(item, f, l, r.min(m), i * 2 + 1, vl, m)
    }
}

impl<T: Debug + Clone + SegtreeItem<M>, M: Debug> Segtree<T, M> {
    pub fn debug(&mut self) -> String {
        format!("{:?}", (0..self.n).map(|i| self.ask(i, i)).collect::<Vec<_>>())
    }
}
