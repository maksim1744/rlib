use crate::treap_node::{TreapItem, TreapItemSized, TreapNode};

pub struct Treap<T> {
    pub root: Option<Box<TreapNode<T>>>,
}

impl<T> Treap<T>
where
    T: TreapItem,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn from_item(item: T) -> Self {
        Self {
            root: Some(Box::new(TreapNode::new(item))),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn merge(left: Self, right: Self) -> Self {
        Self {
            root: TreapNode::merge(left.root, right.root),
        }
    }

    pub fn split_by<P>(self, pred: P) -> (Self, Self)
    where
        P: FnMut(&T) -> bool,
    {
        let (left, right) = TreapNode::split_by(self.root, pred);
        (Treap { root: left }, Treap { root: right })
    }

    pub fn first(&mut self) -> Option<&T> {
        if self.root.is_none() {
            return None;
        }
        let mut node = self.root.as_mut().unwrap();
        while node.left.is_some() {
            node.push();
            node = node.left.as_mut().unwrap();
        }
        Some(&node.item)
    }

    pub fn last(&mut self) -> Option<&T> {
        if self.root.is_none() {
            return None;
        }
        let mut node = self.root.as_mut().unwrap();
        while node.right.is_some() {
            node.push();
            node = node.right.as_mut().unwrap();
        }
        Some(&node.item)
    }

    pub fn root(&self) -> Option<&T> {
        self.root.as_ref().map(|i| &i.item)
    }

    pub fn root_mut(&mut self) -> Option<&mut T> {
        self.root.as_mut().map(|i| &mut i.item)
    }

    pub fn collect(&mut self) -> Vec<&T> {
        match &mut self.root {
            Some(root) => {
                let mut v = Vec::new();
                root.collect_into(&mut v);
                v
            }
            None => Vec::new(),
        }
    }
}

impl<T> Treap<T>
where
    T: TreapItem + TreapItemSized,
{
    pub fn split_at(self, pos: usize) -> (Self, Self) {
        let (left, right) = TreapNode::split_at(self.root, pos);
        (Treap { root: left }, Treap { root: right })
    }

    pub fn insert_at(&mut self, pos: usize, item: T) {
        let (left, right) = TreapNode::split_at(self.root.take(), pos);
        self.root = TreapNode::merge(TreapNode::merge(left, Some(Box::new(TreapNode::new(item)))), right);
    }

    pub fn remove_at(&mut self, pos: usize) -> T {
        let (t1, t23) = TreapNode::split_at(self.root.take(), pos);
        let (t2, t3) = TreapNode::split_at(t23, 1);
        self.root = TreapNode::merge(t1, t3);
        t2.unwrap().item
    }

    pub fn size(&self) -> usize {
        self.root().map(|i| i.size()).unwrap_or(0)
    }
}
