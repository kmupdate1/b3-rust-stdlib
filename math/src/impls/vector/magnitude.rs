use std::ops::{Add, Mul};
use crate::algebra::{Sqrt, Zero};
use crate::number::Magnitude;
use crate::vector::Vector;

impl<T, const N: usize> Magnitude for Vector<T, N>
where
    T: Zero + Add<Output = T> + Mul<Output = T> + Sqrt<Output = T> + Copy,
{
    type Output = T;
    fn magnitude(&self) -> Self::Output {
        self.dot(self).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::number::Magnitude;
    use crate::vector::Vector;

    #[test]
    fn vector_magnitude_f32() {
        let v = Vector::new([3.0_f32, 4.0]);

        assert_eq!(v.dot(&v), 25.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn vector_magnitude_f64() {
        let v = Vector::new([6.0_f64, 8.0]);

        assert_eq!(v.dot(&v), 100.0);
        assert_eq!(v.magnitude(), 10.0);
    }
}
