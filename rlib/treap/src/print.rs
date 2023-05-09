use std::fmt;

use crate::treap::Treap;
use crate::treap_node::{TreapItem, TreapNode};

impl<T> fmt::Debug for TreapNode<T>
where
    T: TreapItem + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(left) = self.left.as_ref() {
            left.fmt(f)?;
        }
        self.item.fmt(f)?;
        write!(f, " ")?;
        if let Some(right) = self.right.as_ref() {
            right.fmt(f)?;
        }
        Ok(())
    }
}

impl<T> fmt::Debug for Treap<T>
where
    T: TreapItem + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_empty() {
            self.root.as_ref().unwrap().fmt(f)?;
        }
        Ok(())
    }
}

pub struct TreePrinter<'a, T> {
    node: &'a Option<Box<TreapNode<T>>>,
    indent: usize,
}

impl<'a, T> TreePrinter<'a, T>
where
    T: TreapItem + fmt::Debug,
{
    pub fn new(t: &'a Treap<T>) -> Self {
        Self {
            node: &t.root,
            indent: 0,
        }
    }

    fn from_node(t: &'a Option<Box<TreapNode<T>>>, indent: usize) -> Self {
        Self { node: t, indent }
    }
}

impl<'a, T> fmt::Debug for TreePrinter<'a, T>
where
    T: TreapItem + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.node {
            None => writeln!(f, "{: >indent$}- [None]", "", indent = self.indent)?,
            Some(node) => {
                write!(f, "{: >indent$}- ", "", indent = self.indent)?;
                node.item.fmt(f)?;
                writeln!(f)?;
                TreePrinter::from_node(&node.left, self.indent + 3).fmt(f)?;
                TreePrinter::from_node(&node.right, self.indent + 3).fmt(f)?;
            }
        };
        Ok(())
    }
}
