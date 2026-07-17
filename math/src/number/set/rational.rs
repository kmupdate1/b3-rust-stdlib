use crate::algebra::{Add, AdditiveInverse, Div, Mul, MultiplicativeInverse, Neg, One, Sub, Zero};
use crate::number::gcd::GreatestCommonDivisor;
use crate::number::set::error::RationalError;
use crate::number::{Fraction, Integer};
use b3_core::error::Result;
use b3_core::validate::Validate;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rational<T>
where
    T: Integer,
{
    fraction: Fraction<T>,
}

impl<T> Rational<T>
where
    T: Integer,
{
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(fraction: Fraction<T>) -> Self {
        Self { fraction }
    }

    pub fn fraction(&self) -> &Fraction<T> { &self.fraction }
    pub fn into_fraction(self) -> Fraction<T> { self.fraction }
}

impl<T> Rational<T>
where
    T: Integer + Zero + One + GreatestCommonDivisor + Div<Output = T> + Clone,
{
    pub fn try_new(fraction: Fraction<T>) -> Result<Self, RationalError> {
        let rational = Self { fraction: fraction.reduced() };
        rational.validate()?;
        Ok(rational)
    }

    pub fn from_parts(numerator: T, denominator: T) -> Result<Self, RationalError> {
        let fraction = Fraction::try_new(numerator, denominator)
            .map_err(RationalError::from)?;

        Self::try_new(fraction)
    }

    pub fn one() -> Self {
        Self {
            fraction: Fraction {
                numerator: One::one(),
                denominator: One::one()
            }
        }
    }

    pub(crate) fn from_fraction(fraction: Fraction<T>) -> Self {
        Rational { fraction: fraction.reduced() }
    }
}

impl<T> Validate for Rational<T>
where
    T: Integer + Zero,
{
    type Error = RationalError;

    fn validate(&self) -> Result<(), Self::Error> {
        self.fraction
            .validate()
            .map_err(RationalError::from)
    }
}

impl<T> Add for Rational<T>
where
    T: Integer + Add<Output = T> + Mul<Output = T> + Clone
    + Div<Output = T> + Zero + One + GreatestCommonDivisor,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_fraction(self.fraction + rhs.fraction)
    }
}

impl<T> Sub for Rational<T>
where
    T: Integer + Add<Output = T> + Mul<Output = T> + Neg<Output = T> + Clone
    + Div<Output = T> + Zero + One + GreatestCommonDivisor,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { self + rhs.inverse() }
}

impl<T> Mul for Rational<T>
where
    T: Integer + Mul<Output = T>
    + Div<Output = T> + Zero + One + GreatestCommonDivisor + Clone,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_fraction(self.fraction * rhs.fraction)
    }
}

impl<T> Div for Rational<T>
where
    T: Integer + Zero + Clone + Mul<Output = T>
    + Div<Output = T> + One + GreatestCommonDivisor,
{
    type Output = Result<Self, RationalError>;
    fn div(self, rhs: Self) -> Self::Output { Ok(self * rhs.try_inverse()?) }
}

impl<T> AdditiveInverse for Rational<T>
where
    T: Integer + Neg<Output = T> + Clone,
{
    fn inverse(&self) -> Self {
        Self { fraction: self.fraction.inverse() }
    }

    fn invert(&mut self) { self.fraction.invert(); }
}

impl<T> MultiplicativeInverse for Rational<T>
where
    T: Integer + Zero + Clone,
{
    type Output = Self;
    type Error = RationalError;

    fn try_inverse(&self) -> Result<Self::Output, Self::Error> {
        Ok(Self {
            fraction: self.fraction
                .try_inverse()
                .map_err(RationalError::from)?,
        })
    }

    fn try_invert(&mut self) -> Result<(), Self::Error> {
        Ok(self.fraction.try_invert()?)
    }
}

impl<T> Display for Rational<T>
where
    T: Integer + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fraction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_new() {
        let fraction = Fraction::new(7, 10);

        let rational = Rational::new(fraction);

        assert_eq!(rational.fraction(), &fraction);
    }

    #[test]
    fn rational_into_fraction() {
        let fraction = Fraction::new(7, 10);

        let rational = Rational::new(fraction);

        assert_eq!(rational.into_fraction(), fraction);
    }
    
    #[test]
    fn rational_try_new_ok() {
        let fraction = Fraction::new(7, 10);

        let rational = Rational::try_new(fraction);

        assert!(rational.is_ok());
    }

    #[test]
    fn rational_validate_ok() {
        let fraction = Fraction::new(7, 10);

        let rational = Rational::new(fraction);

        assert_eq!(rational.validate(), Ok(()));
    }

    #[test]
    fn rational_from_parts() {
        let rational = Rational::from_parts(7, 10);

        assert!(rational.is_ok());

        let rational = rational.unwrap();

        assert_eq!(rational.fraction().numerator(), &7);
        assert_eq!(rational.fraction().denominator(), &10);
    }

    #[test]
    fn rational_from_parts_zero_denominator() {
        let rational = Rational::from_parts(7, 0);

        assert!(rational.is_err());
    }

    #[test]
    fn rational_add_is_reduced() {
        let a = Rational::from_parts(1, 2).unwrap();
        let b = Rational::from_parts(1, 2).unwrap();

        let c = a + b;

        assert_eq!(format!("{}", c), "1/1");
    }

    #[test]
    fn rational_sub_is_reduced() {
        let a = Rational::from_parts(5, 6).unwrap();
        let b = Rational::from_parts(1, 6).unwrap();

        let c = a - b;

        assert_eq!(c, Rational::from_parts(2, 3).unwrap());
    }

    #[test]
    fn rational_mul_is_reduced() {
        let a = Rational::from_parts(2, 3).unwrap();
        let b = Rational::from_parts(3, 4).unwrap();

        let c = a * b;

        assert_eq!(format!("{}", c), "1/2");
    }

    #[test]
    fn div() {
        let lhs = Rational::from_parts(2, 3).unwrap();
        let rhs = Rational::from_parts(4, 9).unwrap();

        let result = (lhs / rhs).unwrap();

        assert_eq!(result, Rational::from_parts(3, 2).unwrap());
    }

    #[test]
    fn div_reduced() {
        let lhs = Rational::from_parts(2, 3).unwrap();
        let rhs = Rational::from_parts(8, 9).unwrap();

        let result = (lhs / rhs).unwrap();

        assert_eq!(result, Rational::from_parts(3, 4).unwrap());
    }

    #[test]
    fn div_by_zero() {
        let lhs = Rational::from_parts(1, 2).unwrap();
        let rhs = Rational::from_parts(0, 1).unwrap();

        assert!((lhs / rhs).is_err());
    }
}
