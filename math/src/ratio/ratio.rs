use b3_core::error::Result;
use b3_core::validate::Validate;
use crate::algebra::Zero;
use crate::number::{Fraction, FractionError};
use crate::ratio::error::RatioError;

/**
 * 二つの値の比を表す。
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio<T> {
    value: T, // -> fraction: Fraction<T>,
}

impl<T> Ratio<T> {
    #[deprecated(since = "0.2.0", note = "Use Ratio::try_new() instead")]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T {
        todo!("&self.value.to_rational()")
    }
    pub fn value_mut(&mut self) -> &mut T { &mut self.value }
    pub fn into_inner(self) -> T { self.value }
}

impl<T> Ratio<T>
where
    T: Zero,
{
    pub fn try_new(value: T) -> Result<Self, RatioError> {
        let ratio = Ratio { value: value };
        ratio.validate()?;
        Ok(ratio)
    }

    pub fn from_fraction(fraction: Fraction<T>) -> Result<Self, RatioError> {
        let (n, d) = fraction.into_parts();
        Self::from_parts(n, d);
        todo!()
    }

    pub fn from_parts(left: T, right: T) -> Result<Self, FractionError> {
        let fraction = Fraction::try_new(left, right)?;
        Self::from_fraction(fraction);
        todo!()
    }
}

impl<T> Validate for Ratio<T> {
    type Error = RatioError;

    fn validate(&self) -> Result<(), Self::Error> {
        /// A. Nothing
        /// B. Must not be NaN
        /// C. Must not be Infinite
        /// D. Must not be Negative
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use b3_core::validate::Validate;
    use crate::ratio::Ratio;

    #[test]
    fn ratio_try_new_ok() {
        let ratio = Ratio::try_new(2.0);

        assert!(ratio.is_ok());
    }

    #[test]
    fn ratio_validate_ok() {
        let ratio = Ratio::new(2.0);

        assert_eq!(ratio.validate(), Ok(()));
    }
}
