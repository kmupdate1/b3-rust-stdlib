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

    // TODO:
    // Consider hiding Fraction from the public API.
    // Fraction is currently an implementation detail.
    pub fn fraction(&self) -> &Fraction<T> { &self.fraction }
    pub fn fraction_mut(&mut self) -> &mut Fraction<T> { &mut self.fraction }
    pub fn into_fraction(self) -> Fraction<T> { self.fraction }
    pub fn left(&self) -> &T { self.fraction.numerator() }
    pub fn right(&self) -> &T { self.fraction.denominator() }
    pub fn into_parts(self) -> (T, T) { self.fraction.into_parts() }
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

        Self::try_new(fraction) // or Self::from_fraction(fraction)
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

macro_rules! impl_ratio_eval {
    ($($t:ty),* $(,)?) => {$(
        impl Ratio<$t> {
            pub fn to_value(&self) -> $t { self.fraction.to_rational() }
        }
    )*};
}

impl_ratio_eval!(f32, f64);

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

    #[test]
    fn ratio_to_value_f32() {
        let ratio = Ratio::from_parts(7.0f32, 10.0f32).unwrap();

        assert_eq!(ratio.to_value(), 0.7f32);
    }

    #[test]
    fn ratio_to_value_f64() {
        let ratio = Ratio::from_parts(7.0f64, 10.0f64).unwrap();

        assert_eq!(ratio.to_value(), 0.7f64);
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
