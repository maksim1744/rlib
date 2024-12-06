pub struct BitsIter<'a, const N: usize> {
    data: &'a [u64; N],
    idx: usize,
}

impl<'a, const N: usize> BitsIter<'a, N> {
    pub fn new(data: &'a [u64; N]) -> Self {
        Self { data, idx: 0 }
    }
}

impl<const N: usize> std::iter::Iterator for BitsIter<'_, N> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.data.len() * 64 && (self.data[self.idx / 64] >> (self.idx % 64)) == 0 {
            self.idx = (self.idx + 64) & !(63usize);
        }
        if self.idx >= self.data.len() * 64 {
            None
        } else {
            self.idx += (self.data[self.idx / 64] >> (self.idx % 64)).trailing_zeros() as usize;
            self.idx += 1;
            Some(self.idx - 1)
        }
    }
}
