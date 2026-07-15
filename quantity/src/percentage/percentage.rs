use crate::percentage::PercentageError;
use crate::Ratio;
use b3_core::validate::Validate;
use b3_math::algebra::Zero;

/**
 * 百分率による表現
 */

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Percentage<T> {
    ratio: Ratio<T>,
}

impl<T> Percentage<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(ratio: Ratio<T>) -> Self {
        Self { ratio }
    }

    pub fn ratio(&self) -> &Ratio<T> { &self.ratio }
    pub fn ratio_mut(&mut self) -> &mut Ratio<T> { &mut self.ratio }
    pub fn into_ratio(self) -> Ratio<T> { self.ratio }
}

impl<T> Percentage<T> {
    pub fn try_new(ratio: Ratio<T>) -> Result<Self, PercentageError> {
        let percentage = Self { ratio };
        percentage.validate()?;
        Ok(percentage)
    }
}

impl<T> Validate for Percentage<T> {
    type Error = PercentageError;

    fn validate(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T> Percentage<T>
where
    T: Zero,
{
    pub fn from_percent(value: T) -> Self { todo!() }
    pub fn from_parts(compared: T, base: T) -> Result<Self, PercentageError> {
        let ratio = Ratio::from_parts(compared, base)?;
        Self::try_new(ratio)
    }
}

impl<T> Percentage<T> {
    /// Percentage -> percent value.
    pub fn to_percent(&self) -> T {
        todo!()
    }

    /// Percentage -> ratio value ([0,1] etc.)
    pub fn to_ratio(&self) -> T {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use b3_core::validate::Validate;
    use crate::Percentage;
    use crate::Ratio;

    #[test]
    fn percentage_try_new_ok() {
        let ratio = Ratio::from_parts(7, 10).unwrap();

        let percentage = Percentage::try_new(ratio);

        assert!(percentage.is_ok());
    }

    #[test]
    fn percentage_validate_ok() {
        let ratio = Ratio::new(
            b3_math::number::Fraction::new(7, 10)
        );

        let percentage = Percentage::new(ratio);

        assert_eq!(percentage.validate(), Ok(()));
    }

    #[test]
    fn percentage_from_parts() {
        let percentage = Percentage::from_parts(7, 10);

        assert!(percentage.is_ok());

        let percentage = percentage.unwrap();

        assert_eq!(
            percentage.ratio().fraction().numerator(),
            &7
        );

        assert_eq!(
            percentage.ratio().fraction().denominator(),
            &10
        );
    }

    #[test]
    fn percentage_from_parts_zero_denominator() {
        let percentage = Percentage::from_parts(7, 0);

        assert!(percentage.is_err());
    }

    #[test]
    fn percentage_into_ratio() {
        let ratio = Ratio::from_parts(7, 10).unwrap();

        let percentage = Percentage::new(ratio);

        assert_eq!(percentage.into_ratio(), ratio);
    }

    #[test]
    fn percentage_ratio() {
        let ratio = Ratio::from_parts(7, 10).unwrap();

        let percentage = Percentage::new(ratio);

        assert_eq!(percentage.ratio(), &ratio);
    }
}
