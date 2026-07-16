use crate::algebra::Mul;
use crate::vector::Vector;

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
    use crate::vector::Vector;

    #[test]
    fn vector_mul_i32() {
        let a = Vector::new([1, 2, 3]);

        let ka = a * 3;

        assert_eq!(ka[0], 3);
        assert_eq!(ka[1], 6);
        assert_eq!(ka[2], 9);
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
    fn vector_mul_zero() {
        let a = Vector::new([3, 6, 9]);

        assert_eq!(a * 0, Vector::new([0, 0, 0]));
    }

    #[test]
    fn vector_mul_single_element() {
        let a = Vector::new([10]);

        assert_eq!(a * 2, Vector::new([20]));
    }

    #[test]
    fn vector_mul_negative() {
        let a = Vector::new([10, -2, 5]);

        assert_eq!(a * -2, Vector::new([-20, 4, -10]));
    }
}
