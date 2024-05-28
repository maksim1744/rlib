use crate::segtree::SegtreeItem;
use rlib_integer::Integer;

#[derive(Clone, Debug)]
pub struct Min<T> {
    pub v: T,
}

impl<T> From<T> for Min<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T> Min<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }
}
impl<T: Integer> Default for Min<T> {
    fn default() -> Self {
        Self { v: T::MAX }
    }
}

impl<T: std::cmp::PartialOrd + Clone> SegtreeItem for Min<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        if left.v < right.v {
            left.clone()
        } else {
            right.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Max<T> {
    pub v: T,
}

impl<T> From<T> for Max<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T> Max<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }
}
impl<T: Integer> Default for Max<T> {
    fn default() -> Self {
        Self { v: T::MIN }
    }
}

impl<T: std::cmp::PartialOrd + Clone> SegtreeItem for Max<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        if left.v > right.v {
            left.clone()
        } else {
            right.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sum<T> {
    pub v: T,
}

impl<T> From<T> for Sum<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T> Sum<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }
}
impl<T: Integer> Default for Sum<T> {
    fn default() -> Self {
        Self { v: T::ZERO }
    }
}

impl<T: std::ops::Add<Output = T> + Clone> SegtreeItem for Sum<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            v: left.v.clone() + right.v.clone(),
        }
    }
}

/// Min on a segment, += on a segment
#[derive(Clone, Debug)]
pub struct MinAdd<T> {
    pub v: T,
    pub md: T,
}

impl<T: Integer> From<T> for MinAdd<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T: Integer> MinAdd<T> {
    pub fn new(v: T) -> Self {
        Self { v, md: T::ZERO }
    }
}
impl<T: Integer> Default for MinAdd<T> {
    fn default() -> Self {
        Self { v: T::MAX, md: T::ZERO }
    }
}

impl<T: Integer + Clone> SegtreeItem<T> for MinAdd<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            v: left.v.min(right.v),
            md: T::ZERO,
        }
    }

    fn modify(&mut self, modifier: &T) {
        self.v += *modifier;
        self.md += *modifier;
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        left.modify(&self.md);
        right.modify(&self.md);
        self.md = T::ZERO;
    }
}

/// Max on a segment, += on a segment
#[derive(Clone, Debug)]
pub struct MaxAdd<T: std::cmp::PartialOrd> {
    pub v: T,
    pub md: T,
}

impl<T: Integer> From<T> for MaxAdd<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T: Integer> MaxAdd<T> {
    pub fn new(v: T) -> Self {
        Self { v, md: T::ZERO }
    }
}
impl<T: Integer> Default for MaxAdd<T> {
    fn default() -> Self {
        Self { v: T::MIN, md: T::ZERO }
    }
}

impl<T: Integer + Clone> SegtreeItem<T> for MaxAdd<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            v: left.v.max(right.v),
            md: T::ZERO,
        }
    }

    fn modify(&mut self, modifier: &T) {
        self.v += *modifier;
        self.md += *modifier;
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        left.modify(&self.md);
        right.modify(&self.md);
        self.md = T::ZERO;
    }
}

/// Sum on a segment, += on a segment
#[derive(Clone, Debug)]
pub struct SumAdd<T: std::cmp::PartialOrd> {
    pub v: T,
    pub len: T,
    pub md: T,
}

impl<T: Integer> From<T> for SumAdd<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T: Integer> SumAdd<T> {
    pub fn new(v: T) -> Self {
        Self {
            v,
            len: T::ONE,
            md: T::ZERO,
        }
    }
}
impl<T: Integer> Default for SumAdd<T> {
    fn default() -> Self {
        Self {
            v: T::ZERO,
            len: T::ZERO,
            md: T::ZERO,
        }
    }
}

impl<T: Integer + Clone> SegtreeItem<T> for SumAdd<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            v: left.v + right.v,
            len: left.len + right.len,
            md: T::ZERO,
        }
    }

    fn modify(&mut self, modifier: &T) {
        self.v += *modifier * self.len;
        self.md += *modifier;
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        left.modify(&self.md);
        right.modify(&self.md);
        self.md = T::ZERO;
    }
}

/// Combinator for two items that implement [SegtreeItem].
#[derive(Clone, Debug, Default)]
pub struct Combinator<U, V>(pub U, pub V);

impl<T: Integer, U: From<T>, V: From<T>> From<T> for Combinator<U, V> {
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
