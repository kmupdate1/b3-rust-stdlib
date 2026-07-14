use b3_math::ratio::Ratio;
use crate::percentage::PercentageError;

/**
 * 百分率による表現
 */

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Percentage<T> {
    ratio: Ratio<T>,
}

impl<T> Percentage<T> {
    pub fn new(value: T) -> Self {
        Self { ratio: Ratio::new(value) }
    }

    pub fn ratio(&self) -> &Ratio<T> { &self.ratio }
    pub fn ratio_mut(&mut self) -> &mut Ratio<T> { &mut self.ratio }
    pub fn into_ratio(self) -> Ratio<T> { self.ratio }
}

impl<T> Percentage<T>
where
    T: PartialEq,
{
    pub fn is_valid(&self) -> bool { todo!() }

    pub fn try_new(&self, ratio: Ratio<T>) -> Result<Self, PercentageError> {
        todo!()
    }

    pub fn from_percentage(value: T) -> Self { todo!() }
    pub fn from_rational(value: T) -> Self { todo!() }
}
