use crate::ratio::error::RatioError;
use b3_core::error::Result;
use b3_core::validate::Validate;
use b3_math::algebra::{Div, Zero};
use b3_math::number::gcd::GreatestCommonDivisor;
use b3_math::number::{Fraction, Integer};
use std::fmt::{Display, Formatter};

/**
 * 二つの値の比を表す。
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio<T: Integer> {
    fraction: Fraction<T>,
}

impl<T> Ratio<T>
where
    T: Integer,
{
    #[deprecated(since = "0.2.0", note = "use `from_parts` instead")]
    pub fn new(fraction: Fraction<T>) -> Self {
        Self { fraction }
    }

    pub fn left(&self) -> &T { self.fraction.numerator() }
    pub fn right(&self) -> &T { self.fraction.denominator() }
    pub fn into_parts(self) -> (T, T) { self.fraction.into_parts() }
}

impl<T> Ratio<T>
where
    T: Integer + Zero,
{
    pub fn from_parts(left: T, right: T) -> Result<Self, RatioError> {
        let fraction = Fraction::try_new(left, right)
            .map_err(RatioError::from)?;

        Ok(Self { fraction })
    }
}

impl<T> Ratio<T>
where
    T: Integer + GreatestCommonDivisor + Div<Output = T> + Clone,
{
    pub fn reduce(&mut self) { self.fraction.reduce(); }
    pub fn reduced(&self) -> Self {
        Self { fraction: self.fraction.reduced() }
    }
}

impl<T> Ratio<T>
where
    T: Integer + Into<f64> + Copy,
{
    pub fn to_f64(&self) -> f64 {
        (*self.left()).into() / (*self.right()).into()
    }
}

impl<T> Display for Ratio<T>
where
    T: Integer + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.left(), self.right())
    }
}

#[cfg(test)]
mod tests {
    use crate::ratio::Ratio;

    #[test]
    fn ratio_from_parts() {
        let ratio = Ratio::from_parts(2, 1);

        assert!(ratio.is_ok());

        let ratio = ratio.unwrap();

        assert_eq!(ratio.left(), &2);
        assert_eq!(ratio.right(), &1);
    }

    #[test]
    fn ratio_from_parts_zero_denominator() {
        let result = Ratio::from_parts(2, 0);

        assert!(result.is_err());
    }

    #[test]
    fn ratio_reduce() {
        let mut ratio = Ratio::from_parts(6, 8).unwrap();

        ratio.reduce();

        assert_eq!(ratio.into_parts(), (3, 4));
    }

    #[test]
    fn ratio_reduced() {
        let ratio = Ratio::from_parts(6, 8).unwrap();

        let reduced = ratio.reduced();

        assert_eq!(reduced.into_parts(), (3, 4));
    }

    #[test]
    fn ratio_to_value_f64() {
        let ratio = Ratio::from_parts(7_i32, 10_i32).unwrap();

        assert_eq!(ratio.to_f64(), 0.7f64);
    }

    #[test]
    fn ratio_parts() {
        let ratio = Ratio::from_parts(7, 10).unwrap();

        assert_eq!(ratio.left(), &7);
        assert_eq!(ratio.right(), &10);
    }

    #[test]
    fn ratio_into_parts() {
        let ratio = Ratio::from_parts(7, 10).unwrap();

        assert_eq!(ratio.into_parts(), (7, 10));
    }
}
