pub trait SegtreeItem: Sized {
    fn merge(left: &Self, right: &Self) -> Self;

    fn update(&mut self, left: &Self, right: &Self) {
        *self = Self::merge(left, right);
    }
}

pub struct Segtree<T: SegtreeItem> {
    n: usize,
    data: Vec<T>,
}

impl<T: Clone + SegtreeItem> Segtree<T> {
    pub fn new_raw(n: usize, value: T) -> Self {
        assert!(n != 0);
        let mut p2: usize = 1;
        while p2 < n {
            p2 *= 2;
        }
        Self {
            n,
            data: vec![value; p2 * 2],
        }
    }

    pub fn new(n: usize, value: T) -> Self {
        let mut res = Self::new_raw(n, value);
        res.rebuild(0, 0, res.n - 1, None);
        res
    }

    pub fn from_slice(data: &[T]) -> Self {
        let mut res = Self::new_raw(data.len(), data[0].clone());
        res.rebuild(0, 0, res.n - 1, Some(data));
        res
    }

    fn rebuild(&mut self, i: usize, l: usize, r: usize, data: Option<&[T]>) {
        if l == r {
            if let Some(data) = data {
                self.data[i] = data[l].clone();
            }
            return;
        }
        let m = (l + r) / 2;
        self.rebuild(i * 2 + 1, l, m, data);
        self.rebuild(i * 2 + 2, m + 1, r, data);

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

        let m = (vl + vr) / 2;
        if ind <= m {
            self.set_internal(ind, value, i * 2 + 1, vl, m);
        } else {
            self.set_internal(ind, value, i * 2 + 2, m + 1, vr);
        }

        let (left, right) = self.data.split_at_mut(i * 2 + 1);
        left[i].update(&right[0], &right[1]);
    }

    pub fn ask(&self, l: usize, r: usize) -> T {
        assert!(l <= r);
        assert!(r < self.n);
        self.ask_internal(l, r, 0, 0, self.n - 1)
    }

    fn ask_internal(&self, l: usize, r: usize, i: usize, vl: usize, vr: usize) -> T {
        if l == vl && r == vr {
            return self.data[i].clone();
        }

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
}
