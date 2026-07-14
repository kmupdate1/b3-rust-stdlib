use super::{IntervalError, Threshold};
use b3_core::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Interval<T> {
    lower: Threshold<T>,
    upper: Threshold<T>,
}

impl<T> Interval<T> {
    #[deprecated(since = "0.2.0", note = "use Interval::try_new() instead")]
    pub fn new(lower: Threshold<T>, upper: Threshold<T>) -> Self {
        Self { lower, upper }
    }

    pub fn lower(&self) -> &Threshold<T> { &self.lower }
    pub fn upper(&self) -> &Threshold<T> { &self.upper }
}

impl<T> Interval<T>
where
    T: PartialOrd,
{
    pub fn is_valid(&self) -> bool {
        self.lower.value() <= self.upper.value()
    }
}

impl<T> Interval<T>
where
    T: PartialOrd,
{
    pub fn try_new(
        lower: Threshold<T>,
        upper: Threshold<T>,
    ) -> Result<Self, IntervalError> {
        if lower.value() > upper.value() {
            return Err(IntervalError::InvalidRange);
        }

        Ok(Self { lower, upper })
    }
}

impl<T> Interval<T>
where
    T: PartialOrd + PartialEq,
{
    pub fn contains(&self, value: &T) -> bool {
        self.lower.contains(value) && self.upper.contains(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::boundary::{Interval, IntervalError, Threshold};

    #[test]
    fn interval_contains_inclusive() {
        let interval = Interval::new(
            Threshold::greater_equal(40),
            Threshold::less_equal(60),
        );

        assert!(!interval.contains(&39));
        assert!(interval.contains(&40));
        assert!(interval.contains(&50));
        assert!(interval.contains(&60));
        assert!(!interval.contains(&61));
    }

    #[test]
    fn interval_valid() {
        let interval = Interval::new(
            Threshold::greater_equal(40),
            Threshold::less_equal(60),
        );

        assert!(interval.is_valid());
    }


    #[test]
    fn interval_invalid() {
        let interval = Interval::new(
            Threshold::greater_equal(60),
            Threshold::less_equal(40),
        );

        assert!(!interval.is_valid());
    }

    #[test]
    fn interval_try_new_valid_range() {
        let result = Interval::try_new(
            Threshold::greater_equal(40),
            Threshold::less_equal(60),
        );

        assert!(result.is_ok());

        let interval = result.unwrap();

        assert!(interval.contains(&40));
        assert!(interval.contains(&50));
        assert!(interval.contains(&60));
    }

    #[test]
    fn interval_try_new_invalid_range() {
        let result = Interval::try_new(
            Threshold::greater_equal(60),
            Threshold::less_equal(40),
        );

        assert_eq!(
            result,
            Err(IntervalError::InvalidRange)
        );
    }
}
