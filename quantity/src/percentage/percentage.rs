use b3_math::ratio::Ratio;
use crate::percentage::PercentageError;

/**
 * 百分率による表現
 */

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Percentage<T> {
    ratio: Ratio<T>, // -> Fraction<T>,
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

impl<T> Percentage<T>
where
    T: PartialEq,
{
    pub fn is_valid(&self) -> bool { todo!("validation logic") }

    pub fn try_new(&self, ratio: Ratio<T>) -> Result<Self, PercentageError> {
        todo!("validation")
    }

    pub fn from_percentage(value: T) -> Self { todo!("Ratio::new(value)") }
    pub fn from_rational(value: T) -> Self { todo!("Ratio::new(value)") }
}
