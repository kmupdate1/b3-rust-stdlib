use crate::number::{Sign, Signed};
use crate::relation::Ordering;

macro_rules! impl_signed_for_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl Signed for $t {
                fn sign(&self) -> Sign {
                    match self.cmp(&0) {
                        Ordering::Less => Sign::Negative,
                        Ordering::Equal => Sign::Zero,
                        Ordering::Greater => Sign::Positive,
                    }
                }
            }
        )*
    };
}

macro_rules! impl_signed_for_float {
    ($($t:ty),* $(,)?) => {
        $(
            impl Signed for $t {
                fn sign(&self) -> Sign {
                    match self.partial_cmp(&0.0) {
                        Some(Ordering::Less) => Sign::Negative,
                        Some(Ordering::Equal) => Sign::Zero,
                        Some(Ordering::Greater) => Sign::Positive,
                        None => Sign::Zero,
                    }
                }
            }
        )*
    };
}

impl_signed_for_int!(i8, i16, i32, i64, i128, isize);
impl_signed_for_float!(f32, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_integer_positive() {
        assert_eq!(1i32.sign(), Sign::Positive);
        assert_eq!(100i64.sign(), Sign::Positive);
    }

    #[test]
    fn signed_integer_negative() {
        assert_eq!((-1i32).sign(), Sign::Negative);
        assert_eq!((-100i64).sign(), Sign::Negative);
    }

    #[test]
    fn signed_integer_zero() {
        assert_eq!(0i32.sign(), Sign::Zero);
        assert_eq!(0i64.sign(), Sign::Zero);
    }

    #[test]
    fn signed_small_integer_types() {
        assert_eq!((-128i8).sign(), Sign::Negative);
        assert_eq!(127i8.sign(), Sign::Positive);

        // assert_eq!(0u8.sign(), Sign::Zero);
        // unsigned has no Sign trait.
    }

    #[test]
    fn signed_float_positive() {
        assert_eq!(1.0f32.sign(), Sign::Positive);
        assert_eq!(100.5f64.sign(), Sign::Positive);
    }

    #[test]
    fn signed_float_negative() {
        assert_eq!((-1.0f32).sign(), Sign::Negative);
        assert_eq!((-100.5f64).sign(), Sign::Negative);
    }

    #[test]
    fn signed_float_zero() {
        assert_eq!(0.0f32.sign(), Sign::Zero);
        assert_eq!(0.0f64.sign(), Sign::Zero);
    }

    #[test]
    fn signed_float_nan() {
        assert_eq!(f32::NAN.sign(), Sign::Zero);
        assert_eq!(f64::NAN.sign(), Sign::Zero);
    }
}
