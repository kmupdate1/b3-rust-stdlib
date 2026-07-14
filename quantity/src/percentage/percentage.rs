/**
 * 百分率による表現
 */

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Percentage<T> {
    value: T,
}

impl<T> Percentage<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T { &self.value }
    pub fn value_mut(&mut self) -> &mut T { &mut self.value }
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
