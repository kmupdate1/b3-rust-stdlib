use std::ops::{Index, IndexMut};

pub struct Vector<T, const N: usize> {
    pub(crate) values: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    pub fn new(values: [T; N]) -> Self {
        Self { values }
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T, const N: usize> AsRef<[T; N]> for Vector<T, N> {
    fn as_ref(&self) -> &[T; N] { &self.values }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_creation() {
        let v = Vector::new([1, 2, 3]);

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn vector_index_mut() {
        let mut v = Vector::new([1, 2, 3]);

        v[1] = 10;

        assert_eq!(v[1], 10);
    }
}
