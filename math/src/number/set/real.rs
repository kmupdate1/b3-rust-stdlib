use crate::algebra::{Add, AdditiveInverse, Div, Mul, MultiplicativeInverse, Neg, Sub, Zero};
use crate::number::RealError;
use b3_core::validate::Validate;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Real<T> {
    value: T,
}

impl<T> Real<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T { &self.value }
    pub fn into_inner(self) -> T { self.value }
}

impl<T> Real<T>
where
    T: Zero,
{
    pub fn try_new(value: T) -> Result<Self, RealError> {
        let real = Self { value };
        real.validate()?;
        Ok(real)
    }
}

impl<T> Validate for Real<T>
where
    T: Zero,
{
    type Error = RealError;

    fn validate(&self) -> b3_core::error::Result<(), Self::Error> {
        Ok(())
    }
}

impl<T> Add for Real<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { value: self.value + rhs.value }
    }
}

impl<T> Sub for Real<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { todo!() }
}

impl<T> Mul for Real<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Div for Real<T>
where
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T> Neg for Real<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl<T> AdditiveInverse for Real<T> {
    fn inverse(&self) -> Self { todo!() }
    fn invert(&mut self) { todo!() }
}

impl<T> MultiplicativeInverse for Real<T> {
    type Output = Self;
    type Error = RealError;

    fn try_inverse(&self) -> b3_core::error::Result<Self::Output, Self::Error> {
        todo!()
    }
    fn try_invert(&mut self) -> b3_core::error::Result<(), Self::Error> {
        todo!()
    }
}

impl<T> Display for Real<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_new() {
        let real = Real::new(3.14);

        assert_eq!(real.value(), &3.14);
    }

    #[test]
    fn real_into_inner() {
        let real = Real::new(3.14);

        assert_eq!(real.into_inner(), 3.14);
    }

    #[test]
    fn real_try_new_ok() {
        let real = Real::try_new(3.14);

        assert!(real.is_ok());
    }

    #[test]
    fn real_validate_ok() {
        let real = Real::new(3.14);

        assert_eq!(real.validate(), Ok(()));
    }

    #[test]
    fn real_add_f32() {
        let real_pi = Real::new(3.14f32);
        let real_napier = Real::new(2.71f32);

        let res = real_pi.add(real_napier);

        assert_ne!(res.value(), &5.85f32);
        assert_ne!(res.value().to_bits(), 5.85f32.to_bits());
    }
}
