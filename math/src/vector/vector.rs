use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl<T, const N: usize> AsMut<[T; N]> for Vector<T, N> {
    fn as_mut(&mut self) -> &mut [T; N] { &mut self.values }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(value: [T; N]) -> Self { Self::new(value) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_new() {
        let v = Vector::new([1, 2, 3]);

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn vector_index() {
        let v = Vector::new([10, 20, 30]);

        assert_eq!(v[0], 10);
        assert_eq!(v[1], 20);
        assert_eq!(v[2], 30);
    }

    #[test]
    fn vector_index_mut() {
        let mut v = Vector::new([1, 2, 3]);

        v[0] = 10;
        v[1] = 20;
        v[2] = 30;

        assert_eq!(v[0], 10);
        assert_eq!(v[1], 20);
        assert_eq!(v[2], 30);
    }

    #[test]
    fn vector_single_element() {
        let v = Vector::new([42]);

        assert_eq!(v[0], 42);
    }

    #[test]
    fn vector_float() {
        let mut v = Vector::new([1.0_f32, 2.0, 3.0]);

        v[1] = 5.0;

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 5.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn vector_from_array() {
        let v: Vector<i32, 3> = [1, 2, 3].into();

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn vector_as_mut() {
        let mut v = Vector::new([1, 2, 3]);

        let values = v.as_mut();
        values[1] = 10;

        assert_eq!(v[1], 10);
    }
}
