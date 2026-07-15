use crate::ratio::error::RatioError;
use b3_core::error::Result;
use b3_core::validate::Validate;
use b3_math::algebra::Zero;
use b3_math::number::Fraction;

/**
 * 二つの値の比を表す。
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio<T> {
    fraction: Fraction<T>,
}

impl<T> Ratio<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(fraction: Fraction<T>) -> Self {
        Self { fraction }
    }

    pub fn fraction(&self) -> &Fraction<T> { &self.fraction }
    pub fn fraction_mut(&mut self) -> &mut Fraction<T> { &mut self.fraction }
    pub fn into_fraction(self) -> Fraction<T> { self.fraction }
}

impl<T> Ratio<T>
where
    T: Zero,
{
    pub fn try_new(fraction: Fraction<T>) -> Result<Self, RatioError> {
        let ratio = Self { fraction };
        ratio.validate()?;
        Ok(ratio)
    }

    // 存在意義を考慮して、今後削除するか検討中。
    #[deprecated(since = "0.2.0", note = "considering to deletion")]
    pub fn from_fraction(fraction: Fraction<T>) -> Result<Self, RatioError> {
        Self::try_new(fraction)
    }

    pub fn from_parts(left: T, right: T) -> Result<Self, RatioError> {
        let fraction = Fraction::try_new(left, right)
            .map_err(RatioError::from)?;

        Self::try_new(fraction)
        // or Self::from_fraction(fraction)
    }
}

impl<T> Validate for Ratio<T> {
    type Error = RatioError;

    fn validate(&self) -> Result<(), Self::Error> {
        /// TODO:
        /// A. Nothing
        /// B. Must not be NaN
        /// C. Must not be Infinite
        /// D. Must not be Negative
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ratio::Ratio;
    use b3_core::validate::Validate;
    use b3_math::number::Fraction;

    #[test]
    fn ratio_try_new_ok() {
        let fraction = Fraction::new(2, 1);
        let ratio = Ratio::try_new(fraction);

        assert!(ratio.is_ok());
    }

    #[test]
    fn ratio_validate_ok() {
        let fraction = Fraction::new(2, 1);
        let ratio = Ratio::new(fraction);

        assert_eq!(ratio.validate(), Ok(()));
    }

    #[test]
    fn ratio_from_fraction() {
        let fraction = Fraction::new(2, 1);

        let ratio = Ratio::from_fraction(fraction);

        assert!(ratio.is_ok());
    }

    #[test]
    fn ratio_from_parts() {
        let ratio = Ratio::from_parts(2, 1);

        assert!(ratio.is_ok());

        let ratio = ratio.unwrap();

        assert_eq!(ratio.fraction().numerator(), &2);
        assert_eq!(ratio.fraction().denominator(), &1);
    }

    #[test]
    fn ratio_from_parts_zero_denominator() {
        let result = Ratio::from_parts(2, 0);

        assert!(result.is_err());
    }
}
