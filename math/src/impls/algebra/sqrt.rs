use crate::algebra::Sqrt;

impl Sqrt for f32 {
    type Output = Self;
    fn sqrt(self) -> Self::Output { f32::sqrt(self) }
}

impl Sqrt for f64 {
    type Output = Self;
    fn sqrt(self) -> Self::Output { f64::sqrt(self) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sqrt_f32() {
        assert_eq!(9.0_f32.sqrt(), 3.0);
        assert_eq!(25.0_f32.sqrt(), 5.0);
    }

    #[test]
    fn sqrt_f64() {
        assert_eq!(9.0_f64.sqrt(), 3.0);
        assert_eq!(25.0_f64.sqrt(), 5.0);
    }

    #[test]
    fn sqrt_zero() {
        assert_eq!(0.0_f32.sqrt(), 0.0);
        assert_eq!(0.0_f64.sqrt(), 0.0);
    }
}
