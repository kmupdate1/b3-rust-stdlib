use b3_math::ratio::Ratio;

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
    pub fn value_mut(&mut self) -> &mut Ratio<T> { &mut self.ratio }
}

impl<T> Percentage<T>
where
    T: PartialEq,
{
    pub fn is_valid(&self) -> bool { todo!() }

    pub fn try_new(&self, value: T) -> Result<Self, T> { todo!() }

    pub fn from_percentage(value: T) -> Self { todo!() }
    pub fn from_rational(value: T) -> Self { todo!() }
}
