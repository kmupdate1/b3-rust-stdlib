use std::ops::{Add, Mul};
use crate::algebra::Zero;
use crate::vector::Vector;

impl<T, const N: usize> Vector<T, N>
where
    T: Zero + Add<Output = T> + Mul<Output = T> + Copy,
{
    pub fn dot(&self, rhs: &Self) -> T {
        let mut sum = T::zero();

        for i in 0..N {
            sum = sum + self[i] * rhs[i];
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;

    #[test]
    fn vector_inner_i32() {
        let a = Vector::new([1, 2, 3]);
        let b = Vector::new([4, 5, 6]);

        let c = a.dot(&b);

        assert_eq!(c, 4 + 10 + 18);
    }

    #[test]
    fn vector_inner_f32() {
        let a = Vector::new([1.5_f32, 2.5, 3.5]);
        let b = Vector::new([0.5_f32, 1.5, 2.5]);

        let c = a.dot(&b);

        assert_eq!(c, 1.5f32 * 0.5f32 + 2.5f32 * 1.5f32 + 3.5f32 * 2.5f32);
    }

    #[test]
    fn vector_inner_zero() {
        let a = Vector::new([3, 6, 9]);
        let b = Vector::new([0, 0, 0]);

        assert_eq!(a.dot(&b), 0);
    }

    #[test]
    fn vector_inner_single_element() {
        let a = Vector::new([10]);
        let b = Vector::new([20]);

        assert_eq!(a.dot(&b), 200);
    }

    #[test]
    fn vector_inner_negative() {
        let a = Vector::new([-10, -2, 5]);
        let b = Vector::new([-20, 4, -10]);

        assert_eq!(a.dot(&b), 200 + -8 + -50);
    }
}
