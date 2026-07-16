use std::fmt::{Display, Formatter};
use b3_core::error::Result;
use b3_core::validate::Validate;
use crate::algebra::{Add, Sub, Mul, Div, Zero, AdditiveInverse, MultiplicativeInverse, One};
use crate::number::Fraction;
use crate::number::gcd::GreatestCommonDivisor;
use crate::number::set::error::RationalError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rational<T> {
    fraction: Fraction<T>,
}

impl<T> Rational<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(fraction: Fraction<T>) -> Self {
        Self { fraction }
    }

    pub fn fraction(&self) -> &Fraction<T> { &self.fraction }
    pub fn fraction_mut(&mut self) -> &mut Fraction<T> { &mut self.fraction }
    pub fn into_fraction(self) -> Fraction<T> { self.fraction }
}

impl<T> Rational<T>
where
    T: Zero + One + GreatestCommonDivisor + Div<Output = T> + Clone,
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
}

impl<T> Validate for Rational<T>
where
    T: Zero,
{
    type Error = RationalError;

    fn validate(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T> Add for Rational<T>
where
    T: Add<Output = T> + Mul<Output = T> + Clone,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { fraction: self.fraction + rhs.fraction }
    }
}

impl<T> Sub for Rational<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Clone,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { fraction: self.fraction - rhs.fraction }
    }
}

impl<T> Mul for Rational<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self { fraction: self.fraction * rhs.fraction }
    }
}

impl<T> Div for Rational<T>
where
    T: Zero + Clone + Mul<Output = T>,
{
    type Output = Result<Self, RationalError>;
    fn div(self, rhs: Self) -> Self::Output {
        Ok(Self {
            fraction: (self.fraction / rhs.fraction)
                .map_err(RationalError::from)?,
        })
    }
}

impl<T> AdditiveInverse for Rational<T> {
    fn inverse(&self) -> Self {
        todo!()
    }

    fn invert(&mut self) {
        todo!()
    }
}

impl<T> MultiplicativeInverse for Rational<T>
where
    T: Zero + Clone,
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
    T: Display,
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
}
