use rlib_rand::Rng;

pub trait TreapItem {
    fn update(&mut self, _left: Option<&Self>, _right: Option<&Self>) {}
    fn push(&mut self, _left: Option<&mut Self>, _right: Option<&mut Self>) {}
}

pub trait TreapItemSized {
    fn size(&self) -> usize;
}

static mut RNG: Rng = Rng::new(42);

type Priority = u32;

#[allow(static_mut_refs)]
fn gen_priority() -> Priority {
    unsafe { RNG.next_raw() as Priority }
}

pub struct TreapNode<T> {
    pub item: T,
    pub priority: Priority,
    pub left: Option<Box<TreapNode<T>>>,
    pub right: Option<Box<TreapNode<T>>>,
}

impl<T> TreapNode<T> {
    pub fn new(item: T) -> Self {
        Self {
            item,
            priority: gen_priority(),
            left: None,
            right: None,
        }
    }
}

impl<T> TreapNode<T>
where
    T: TreapItem,
{
    pub fn update(&mut self) {
        self.item.update(
            self.left.as_ref().map(|x| &x.item),
            self.right.as_ref().map(|x| &x.item),
        );
    }

    pub fn merge(mut left: Option<Box<Self>>, mut right: Option<Box<Self>>) -> Option<Box<Self>> {
        if left.is_none() {
            right
        } else if right.is_none() {
            left
        } else if left.as_ref().unwrap().priority < right.as_ref().unwrap().priority {
            left.as_mut().unwrap().push();
            let m = left.as_mut().unwrap().right.take();
            left.as_mut().unwrap().right = Self::merge(m, right);
            left.as_mut().unwrap().update();
            left
        } else {
            right.as_mut().unwrap().push();
            let m = right.as_mut().unwrap().left.take();
            right.as_mut().unwrap().left = Self::merge(left, m);
            right.as_mut().unwrap().update();
            right
        }
    }

    pub fn split_by<P>(mut root: Option<Box<Self>>, mut pred: P) -> (Option<Box<Self>>, Option<Box<Self>>)
    where
        P: FnMut(&T) -> bool,
    {
        if root.is_none() {
            return (None, None);
        }
        root.as_mut().unwrap().push();
        if pred(&root.as_ref().unwrap().item) {
            let (a, b) = Self::split_by(root.as_mut().unwrap().right.take(), pred);
            root.as_mut().unwrap().right = a;
            root.as_mut().unwrap().update();
            (root, b)
        } else {
            let (a, b) = Self::split_by(root.as_mut().unwrap().left.take(), pred);
            root.as_mut().unwrap().left = b;
            root.as_mut().unwrap().update();
            (a, root)
        }
    }

    pub fn push(&mut self) {
        self.item.push(
            self.left.as_mut().map(|i| &mut i.item),
            self.right.as_mut().map(|i| &mut i.item),
        );
    }

    pub fn collect_into<'a>(&'a mut self, res: &mut Vec<&'a T>) {
        self.push();

        if let Some(left) = &mut self.left {
            left.collect_into(res);
        }
        res.push(&self.item);
        if let Some(right) = &mut self.right {
            right.collect_into(res);
        }
    }
}

impl<T> TreapNode<T>
where
    T: TreapItem + TreapItemSized,
{
    pub fn split_at(mut root: Option<Box<Self>>, pos: usize) -> (Option<Box<Self>>, Option<Box<Self>>) {
        if root.is_none() {
            return (None, None);
        }
        root.as_mut().unwrap().push();
        if pos > root.as_ref().unwrap().left.as_ref().map(|i| i.item.size()).unwrap_or(0) {
            let (a, b) = Self::split_at(
                root.as_mut().unwrap().right.take(),
                pos - root.as_ref().unwrap().left.as_ref().map(|i| i.item.size()).unwrap_or(0) - 1,
            );
            root.as_mut().unwrap().right = a;
            root.as_mut().unwrap().update();
            (root, b)
        } else {
            let (a, b) = Self::split_at(root.as_mut().unwrap().left.take(), pos);
            root.as_mut().unwrap().left = b;
            root.as_mut().unwrap().update();
            (a, root)
        }
    }
}
