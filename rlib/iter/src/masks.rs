pub trait IterMasks: Copy {
    fn next_submask(&mut self, x: Self) -> Option<Self>;
    fn next_supermask(&mut self, x: Self) -> Option<Self>;
    fn zero() -> Self;
    fn ones() -> Self;
}

macro_rules! impl_iter_masks {
    ($($t:ty),*) => {$(
        impl IterMasks for $t {
            fn next_submask(&mut self, x: Self) -> Option<Self> {
                if *self == 0 {
                    None
                } else {
                    let cur = *self;
                    *self = self.wrapping_sub(1) & x;
                    Some(cur)
                }
            }

            fn zero() -> Self {
                0
            }

            fn next_supermask(&mut self, x: Self) -> Option<Self> {
                if self.count_zeros() == 0 {
                    None
                } else {
                    let cur = *self;
                    *self = self.wrapping_add(1) | x;
                    Some(cur)
                }
            }

            fn ones() -> Self {
                <$t>::from_le_bytes([0xff; <$t>::BITS as usize / 8])
            }
        }
    )*};
}

impl_iter_masks!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

pub fn iter_submasks<T>(x: T) -> impl Iterator<Item = T>
where
    T: IterMasks,
{
    let mut submask = x;
    std::iter::from_fn(move || submask.next_submask(x)).chain([T::zero()])
}

pub fn iter_supermasks<T>(x: T) -> impl Iterator<Item = T>
where
    T: IterMasks,
{
    let mut supermask = x;
    std::iter::from_fn(move || supermask.next_supermask(x)).chain([T::ones()])
}
