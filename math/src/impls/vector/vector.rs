use std::ops::{Add, Mul, Sub};
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

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut values = self.values;

        for i in 0..N {
            values[i] = values[i] - rhs.values[i];
        }

        Self { values }
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut values = self.values;

        for i in 0..N {
            values[i] = values[i] * rhs;
        }

        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_add_i32() {
        let a = Vector::new([1, 2, 3]);
        let b = Vector::new([4, 5, 6]);

        let c = a + b;

        assert_eq!(c[0], 5);
        assert_eq!(c[1], 7);
        assert_eq!(c[2], 9);
    }

    #[test]
    fn vector_sub_i32() {
        let a = Vector::new([7, 2, 5]);
        let b = Vector::new([4, 5, 6]);

        let c = a - b;

        assert_eq!(c[0], 3);
        assert_eq!(c[1], -3);
        assert_eq!(c[2], -1);
    }

    #[test]
    fn vector_mul_i32() {
        let a = Vector::new([1, 2, 3]);

        let ka = a * 3;

        assert_eq!(ka[0], 3);
        assert_eq!(ka[1], 6);
        assert_eq!(ka[2], 9);
    }

    #[test]
    fn vector_add_f32() {
        let a = Vector::new([1.5_f32, 2.5, 3.5]);
        let b = Vector::new([0.5_f32, 1.5, 2.5]);

        let c = a + b;

        assert_eq!(c[0], 2.0);
        assert_eq!(c[1], 4.0);
        assert_eq!(c[2], 6.0);
    }

    #[test]
    fn vector_sub_f32() {
        let a = Vector::new([1.5_f32, -2.5, 3.5]);
        let b = Vector::new([0.5_f32, 1.5, -2.5]);

        let c = a - b;

        assert_eq!(c[0], 1.0);
        assert_eq!(c[1], -4.0);
        assert_eq!(c[2], 6.0);
    }

    #[test]
    fn vector_mul_f32() {
        let a = Vector::new([1.5_f32, 2.5, 3.5]);

        let ka = a * 3.4;

        assert_eq!(ka[0], 1.5f32 * 3.4);
        assert_eq!(ka[1], 2.5 * 3.4);
        assert_eq!(ka[2], 3.5 * 3.4);
    }

    #[test]
    fn vector_add_zero() {
        let a = Vector::new([3, 6, 9]);
        let z = Vector::new([0, 0, 0]);

        assert_eq!(a + z, Vector::new([3, 6, 9]));
    }

    #[test]
    fn vector_sub_zero() {
        let a = Vector::new([3, 6, 9]);
        let z = Vector::new([0, 0, 0]);

        assert_eq!(a - z, Vector::new([3, 6, 9]));
    }

    #[test]
    fn vector_mul_zero() {
        let a = Vector::new([3, 6, 9]);

        assert_eq!(a * 0, Vector::new([0, 0, 0]));
    }

    #[test]
    fn vector_add_single_element() {
        let a = Vector::new([10]);
        let b = Vector::new([20]);

        assert_eq!(a + b, Vector::new([30]));
    }

    #[test]
    fn vector_sub_single_element() {
        let a = Vector::new([10]);
        let b = Vector::new([20]);

        assert_eq!(a - b, Vector::new([-10]));
    }

    #[test]
    fn vector_mul_single_element() {
        let a = Vector::new([10]);

        assert_eq!(a * 2, Vector::new([20]));
    }

    #[test]
    fn vector_add_negative() {
        let a = Vector::new([10, -2, 5]);
        let b = Vector::new([-3, 8, -10]);

        assert_eq!(a + b, Vector::new([7, 6, -5]));
    }

    #[test]
    fn vector_sub_negative() {
        let a = Vector::new([10, -2, 5]);
        let b = Vector::new([-3, 8, -10]);

        assert_eq!(a - b, Vector::new([13, -10, 15]));
    }

    #[test]
    fn vector_mul_negative() {
        let a = Vector::new([10, -2, 5]);

        assert_eq!(a * -2, Vector::new([-20, 4, -10]));
    }
}
