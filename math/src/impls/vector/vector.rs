use std::ops::Add;
use crate::vector::Vector;

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut values = self.values;

        for i in 0..N {
            values[i] = values[i] + rhs.values[i];
        }

        Self { values }
    }
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
}
