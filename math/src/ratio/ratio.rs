use crate::rational::Fraction;
use b3_core::error::Result;
use crate::ratio::error::RatioError;

/**
 * 二つの値の比を表す。
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio<T> {
    value: T,
}

impl<T> Ratio<T> {
    #[deprecated(since = "0.2.0", note = "Use Ratio::try_new() instead")]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T { &self.value }
    pub fn value_mut(&mut self) -> &mut T { &mut self.value }
    pub fn into_inner(self) -> T { self.value }
}

impl<T> Ratio<T> {
    pub fn try_new(value: T) -> Result<Self, RatioError> {
        todo!("validation")
    }

    pub fn from_fraction(fraction: Fraction<T>) -> Result<Self, RatioError> {
        Self::try_new(todo!("fraction -> value"))
    }

    pub fn from_parts(left: T, right: T) -> Result<Self, RatioError> {
        Self::try_new(todo!("left / right"))
    }
}
