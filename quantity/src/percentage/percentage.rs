use crate::{PercentageError, Proportion};
use b3_core::error::Result;
use b3_math::number::Integer;
use std::fmt::{Display, Formatter};

pub struct Percentage<T: Integer> {
    proportion: Proportion<T>,
}

impl<T> Percentage<T>
where
    T: Integer,
{
    pub fn proportion(&self) -> &Proportion<T> {
        &self.proportion
    }
}

impl<T> Percentage<T>
where
    T: Integer + PartialOrd,
{
    pub fn from_parts(part: T, whole: T) -> Result<Self, PercentageError> {
        let proportion = Proportion::from_parts(part, whole)?;

        Ok(Self { proportion })
    }

    pub fn new(proportion: Proportion<T>) -> Self {
        Self { proportion }
    }
}

impl<T> Percentage<T>
where
    T: Integer + Into<f64> + Copy,
{
    pub fn percent(&self) -> f64 {
        self.proportion.ratio().to_f64() * 100.0
    }
}

impl<T> Display for Percentage<T>
where
    T: Integer + Into<f64> + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.percent())
    }
}

#[cfg(test)]
mod tests {
    use crate::Percentage;

    #[test]
    fn percentage_from_parts() {
        let percentage = Percentage::from_parts(7, 10).unwrap();

        assert_eq!(percentage.percent(), 70.0);
    }

    #[test]
    fn percentage_display() {
        let percentage = Percentage::from_parts(7, 10).unwrap();

        assert_eq!(format!("{}", percentage), "70%");
    }

    #[test]
    fn percentage_zero() {
        let percentage = Percentage::from_parts(0, 10).unwrap();

        assert_eq!(percentage.percent(), 0.0);
    }

    #[test]
    fn percentage_one_hundred() {
        let percentage = Percentage::from_parts(10, 10).unwrap();

        assert_eq!(percentage.percent(), 100.0);
    }

    #[test]
    fn percentage_half() {
        let percentage = Percentage::from_parts(1, 2).unwrap();

        assert_eq!(percentage.percent(), 50.0);
    }

    #[test]
    fn percentage_one_quarter() {
        let percentage = Percentage::from_parts(1, 4).unwrap();

        assert_eq!(percentage.percent(), 25.0);
    }

    #[test]
    fn percentage_three_quarters() {
        let percentage = Percentage::from_parts(3, 4).unwrap();

        assert_eq!(percentage.percent(), 75.0);
    }

    #[test]
    fn percentage_zero_denominator() {
        let result = Percentage::from_parts(7, 0);

        assert!(result.is_err());
    }

    #[test]
    fn percentage_greater_than_one() {
        let result = Percentage::from_parts(11, 10);

        assert!(result.is_err());
    }

    #[test]
    fn percentage_negative() {
        let result = Percentage::from_parts(-1, 10);

        assert!(result.is_err());
    }
}
