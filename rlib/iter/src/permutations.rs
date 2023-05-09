pub struct PermutationIter<T> {
    data: Vec<T>,
    first: bool,
}

pub fn iter_permutations<T: Ord + Clone>(mut data: Vec<T>) -> impl Iterator<Item = Vec<T>> {
    data.sort();
    PermutationIter { data, first: true }
}

pub fn next_permutation<T: Ord>(data: &mut [T]) -> bool {
    for i in (1..data.len()).rev() {
        if data[i - 1] < data[i] {
            let mut j = i;
            while j + 1 < data.len() && data[j + 1] > data[i - 1] {
                j += 1;
            }
            data.swap(i - 1, j);
            data[i..].reverse();
            return true;
        }
    }
    data.reverse();
    false
}

impl<T: Ord + Clone> Iterator for PermutationIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.data.clone());
        }
        if next_permutation(&mut self.data) {
            Some(self.data.clone())
        } else {
            None
        }
    }
}
