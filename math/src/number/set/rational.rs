use b3_core::error::Result;
use b3_core::validate::Validate;
use crate::algebra::Zero;
use crate::number::Fraction;
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
    T: Zero,
{
    pub fn try_new(fraction: Fraction<T>) -> Result<Self, RationalError> {
        let rational = Self { fraction };
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
