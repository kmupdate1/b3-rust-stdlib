use b3_core::error::Result;
use b3_core::validate::Validate;
use crate::algebra::Zero;
use crate::rational::error::FractionError;

/**
 * 分子・分母による有理数の表現。
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction<T> {
    numerator: T,
    denominator: T,
}

impl<T> Fraction<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(numerator: T, denominator: T) -> Self {
        Self { numerator, denominator }
    }

    pub fn numerator(&self) -> &T { todo!("Fraction -> T") }
    pub fn denominator(&self) -> &T { todo!("Fraction -> T") }
    pub fn into_parts(self) -> (T, T) { todo!("Fraction -> T") }
}

impl<T> Fraction<T>
where
    T: Zero,
{
    pub fn try_new(numerator: T, denominator: T) -> Result<Self, FractionError> {
        let fraction = Fraction { numerator, denominator };
        fraction.validate()?;
        Ok(fraction)
    }
}

impl<T> Validate for  Fraction<T>
where
    T: Zero + PartialEq,
{
    type Error = FractionError;

    fn validate(&self) -> Result<(), Self::Error> {
        if self.denominator.is_zero() {
            return  Err(FractionError::ZeroDenominator)
        }

        /// TODO:
        /// NaN / Infinite の検証は、Float 系 trait 設計後に追加する。
        ///
        /// if self.numerator.is_nan() {
        ///     return  Err(RationalError::NaN)
        /// }
        ///
        /// if self.denominator.is_nan() {
        ///     return  Err(RationalError::NaN)
        /// }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use b3_core::validate::Validate;
    use crate::rational::error::FractionError;
    use crate::rational::Fraction;

    #[test]
    fn fraction_validate_ok() {
        let fraction = Fraction::new(1, 2);

        assert_eq!(fraction.validate(), Ok(()));
    }

    #[test]
    fn fraction_validate_zero_denominator() {
        let fraction = Fraction::new(1, 0);

        assert_eq!(
            fraction.validate(),
            Err(FractionError::ZeroDenominator)
        );
    }

    #[test]
    fn fraction_try_new_ok() {
        let result = Fraction::try_new(1, 2);

        assert!(result.is_ok());
    }

    #[test]
    fn fraction_try_new_zero_denominator() {
        let result = Fraction::try_new(1, 0);

        assert_eq!(
            result,
            Err(FractionError::ZeroDenominator)
        );
    }
}
