use crate::segtree::SegtreeItem;

#[derive(Clone, Debug)]
pub struct Min<T: std::cmp::PartialOrd + Clone> {
    pub v: T,
}
impl<T: std::cmp::PartialOrd + Clone> Min<T> {
    pub fn new(v: T) -> Self {
        Self { v }
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
pub struct Max<T: std::cmp::PartialOrd + Clone> {
    pub v: T,
}
impl<T: std::cmp::PartialOrd + Clone> Max<T> {
    pub fn new(v: T) -> Self {
        Self { v }
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
pub struct Sum<T: Clone> {
    pub v: T,
}
impl<T: std::cmp::PartialOrd + Clone> Sum<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }
}

impl<T: std::ops::Add<Output = T> + Clone> SegtreeItem for Sum<T> {
    fn merge(left: &Self, right: &Self) -> Self {
        Self {
            v: left.v.clone() + right.v.clone(),
        }
    }
}
