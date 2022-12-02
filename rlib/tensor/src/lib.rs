use std::ops::{Index, IndexMut};

use rlib_io::{Readable, Reader, Writable, Writer};

#[derive(Clone)]
pub struct Tensor<T, const D: usize> {
    dims: [usize; D],
    data: Vec<T>,
}

impl<T, const D: usize> Tensor<T, D> {
    pub fn from_vec(dims: [usize; D], data: Vec<T>) -> Self {
        assert!(!dims.contains(&0));
        assert_eq!(dims.iter().product::<usize>(), data.len());
        Self { dims, data }
    }

    pub fn get_index(&self, idx: [usize; D]) -> usize {
        let mut result = 0;
        let mut sz = 1;
        for i in (0..D).rev() {
            assert!(idx[i] < self.dims[i]);
            result += sz * idx[i];
            sz *= self.dims[i];
        }
        result
    }

    pub fn dims(&self) -> &[usize; D] {
        &self.dims
    }

    pub fn dim(&self, i: usize) -> usize {
        self.dims[i]
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.data.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<T: Clone, const D: usize> Tensor<T, D> {
    pub fn new(dims: [usize; D], value: T) -> Self {
        assert!(!dims.contains(&0));
        Self {
            dims,
            data: vec![value; dims.iter().product()],
        }
    }

    pub fn from_slice(dims: [usize; D], data: &[T]) -> Self {
        assert!(!dims.contains(&0));
        assert_eq!(dims.iter().product::<usize>(), data.len());
        Self {
            dims,
            data: data.iter().cloned().collect(),
        }
    }
}

impl<T, const D: usize> Index<[usize; D]> for Tensor<T, D> {
    type Output = T;

    fn index(&self, idx: [usize; D]) -> &Self::Output {
        &self.data[self.get_index(idx)]
    }
}

impl<T, const D: usize> IndexMut<[usize; D]> for Tensor<T, D> {
    fn index_mut(&mut self, idx: [usize; D]) -> &mut Self::Output {
        let idx = self.get_index(idx);
        &mut self.data[idx]
    }
}

impl<T: PartialEq, const D: usize> PartialEq for Tensor<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Readable, const D: usize> Tensor<T, D> {
    pub fn read(dims: [usize; D], reader: &mut Reader) -> Self {
        assert!(!dims.contains(&0));
        Self {
            dims,
            data: reader.read_vec(dims.iter().product()),
        }
    }
}

impl<T: Writable, const D: usize> Writable for Tensor<T, D> {
    fn write(&self, writer: &mut Writer) {
        let mut idx = [0; D];
        loop {
            writer.write(&self[idx]);
            if let Some(pos) = idx.iter().zip(self.dims.iter()).rposition(|(i1, i2)| i1 + 1 != *i2) {
                if pos + 1 == D {
                    writer.write_char(' ');
                } else {
                    for _ in 0..(D - pos - 1) {
                        writer.write_char('\n');
                    }
                }
                idx[pos] += 1;
                for i in pos + 1..D {
                    idx[i] = 0;
                }
            } else {
                break;
            }
        }
    }
}

impl<T: std::fmt::Debug, const D: usize> std::fmt::Debug for Tensor<T, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut idx = [0; D];
        write!(f, "{}", (0..D).map(|_| '[').collect::<String>())?;
        loop {
            write!(f, "{:?}", self[idx])?;
            if let Some(pos) = idx.iter().zip(self.dims.iter()).rposition(|(i1, i2)| i1 + 1 != *i2) {
                if pos + 1 == D {
                    write!(f, ", ")?;
                } else {
                    for _ in 0..(D - pos - 1) {
                        write!(f, "]")?;
                    }
                    write!(f, ", ")?;
                    for _ in 0..(D - pos - 1) {
                        write!(f, "[")?;
                    }
                }
                idx[pos] += 1;
                for i in pos + 1..D {
                    idx[i] = 0;
                }
            } else {
                break;
            }
        }
        write!(f, "{}", (0..D).map(|_| ']').collect::<String>())?;
        Ok(())
    }
}
