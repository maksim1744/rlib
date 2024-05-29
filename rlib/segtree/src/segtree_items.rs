//! Implementations of [SegtreeItem] for common operations.

use std::ops::{Add, AddAssign, Mul};

use crate::segtree::SegtreeItem;
use rlib_num_traits::{MinMax, ZeroOne};

/// Query min on a segment
#[derive(Clone, Debug)]
pub struct Min<T: PartialOrd + Clone> {
    pub v: T,
}

impl<T: PartialOrd + Clone> From<T> for Min<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T: PartialOrd + Clone> Min<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }
}

impl<T: PartialOrd + Clone + MinMax> Default for Min<T> {
    fn default() -> Self {
        Self { v: T::MAX }
    }
}

impl<T: PartialOrd + Clone> SegtreeItem for Min<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        if left.v < right.v {
            left.clone()
        } else {
            right.clone()
        }
    }
}

/// Query max on a segment
#[derive(Clone, Debug)]
pub struct Max<T: PartialOrd + Clone> {
    pub v: T,
}

impl<T: PartialOrd + Clone> From<T> for Max<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T: PartialOrd + Clone> Max<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }
}

impl<T: PartialOrd + Clone + MinMax> Default for Max<T> {
    fn default() -> Self {
        Self { v: T::MIN }
    }
}

impl<T: PartialOrd + Clone> SegtreeItem for Max<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        if left.v > right.v {
            left.clone()
        } else {
            right.clone()
        }
    }
}

/// Query sum on a segment
#[derive(Clone, Debug)]
pub struct Sum<T: Add<Output = T> + Clone> {
    pub v: T,
}

impl<T: Add<Output = T> + Clone> From<T> for Sum<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T: Add<Output = T> + Clone> Sum<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }
}

impl<T: Add<Output = T> + Clone + Default> Default for Sum<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Add<Output = T> + Clone> SegtreeItem for Sum<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            v: left.v.clone() + right.v.clone(),
        }
    }
}

/// Query min on a segment, += on a segment
#[derive(Clone, Debug)]
pub struct MinAdd<T: PartialOrd + AddAssign + Default + Clone> {
    pub v: T,
    pub md: T,
}

impl<T: PartialOrd + AddAssign + Default + Clone> From<T> for MinAdd<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T: PartialOrd + AddAssign + Default + Clone> MinAdd<T> {
    pub fn new(v: T) -> Self {
        Self { v, md: T::default() }
    }
}

impl<T: PartialOrd + AddAssign + Default + Clone + MinMax> Default for MinAdd<T> {
    fn default() -> Self {
        Self {
            v: T::MAX,
            md: T::default(),
        }
    }
}

impl<T: PartialOrd + AddAssign + Default + Clone> SegtreeItem<T> for MinAdd<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self::new(if left.v < right.v {
            left.v.clone()
        } else {
            right.v.clone()
        })
    }

    fn modify(&mut self, modifier: &T) {
        self.v += modifier.clone();
        self.md += modifier.clone();
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        left.modify(&self.md);
        right.modify(&self.md);
        self.md = T::default();
    }
}

/// Query max on a segment, += on a segment
#[derive(Clone, Debug)]
pub struct MaxAdd<T: PartialOrd + AddAssign + Default + Clone> {
    pub v: T,
    pub md: T,
}

impl<T: PartialOrd + AddAssign + Default + Clone> From<T> for MaxAdd<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T: PartialOrd + AddAssign + Default + Clone> MaxAdd<T> {
    pub fn new(v: T) -> Self {
        Self { v, md: T::default() }
    }
}

impl<T: PartialOrd + AddAssign + Default + Clone + MinMax> Default for MaxAdd<T> {
    fn default() -> Self {
        Self {
            v: T::MIN,
            md: T::default(),
        }
    }
}

impl<T: PartialOrd + AddAssign + Default + Clone> SegtreeItem<T> for MaxAdd<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self::new(if left.v > right.v {
            left.v.clone()
        } else {
            right.v.clone()
        })
    }

    fn modify(&mut self, modifier: &T) {
        self.v += modifier.clone();
        self.md += modifier.clone();
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        left.modify(&self.md);
        right.modify(&self.md);
        self.md = T::default();
    }
}

/// Query sum on a segment, += on a segment
#[derive(Clone, Debug)]
pub struct SumAdd<T: Add<Output = T> + Mul<Output = T> + Default + Clone> {
    pub v: T,
    pub len: T,
    pub md: T,
}

impl<T: Add<Output = T> + Mul<Output = T> + Default + Clone + ZeroOne> From<T> for SumAdd<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Default + Clone + ZeroOne> SumAdd<T> {
    pub fn new(v: T) -> Self {
        Self {
            v,
            len: T::ONE,
            md: T::default(),
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Default + Clone> Default for SumAdd<T> {
    fn default() -> Self {
        Self {
            v: T::default(),
            len: T::default(),
            md: T::default(),
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Default + Clone> SegtreeItem<T> for SumAdd<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            v: left.v.clone() + right.v.clone(),
            len: left.len.clone() + right.len.clone(),
            md: T::default(),
        }
    }

    fn modify(&mut self, modifier: &T) {
        self.v = self.v.clone() + modifier.clone() * self.len.clone();
        self.md = self.md.clone() + modifier.clone();
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        left.modify(&self.md);
        right.modify(&self.md);
        self.md = T::default();
    }
}

/// Combinator for two items that implement [SegtreeItem].
#[derive(Clone, Debug, Default)]
pub struct Combinator<U, V>(pub U, pub V);

impl<T: Copy, U: From<T>, V: From<T>> From<T> for Combinator<U, V> {
    fn from(v: T) -> Self {
        Self(U::from(v), V::from(v))
    }
}

impl<M, U: SegtreeItem<M>, V: SegtreeItem<M>> SegtreeItem<M> for Combinator<U, V> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self(U::merge(&left.0, &right.0), V::merge(&left.1, &right.1))
    }

    fn modify(&mut self, modifier: &M) {
        self.0.modify(modifier);
        self.1.modify(modifier);
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        self.0.push(&mut left.0, &mut right.0);
        self.1.push(&mut left.1, &mut right.1);
    }
}
