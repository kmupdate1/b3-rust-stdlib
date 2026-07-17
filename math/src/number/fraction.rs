use crate::algebra::{Add, AdditiveInverse, Div, Mul, MultiplicativeInverse, Neg, One, Sub, Zero};
use crate::number::gcd::GreatestCommonDivisor;
use crate::number::{FractionError, Integer};
use b3_core::error::Result;
use b3_core::validate::Validate;
use core::mem::swap;
use std::fmt::{Display, Formatter};

/**
 * 分子・分母による有理数の表現。
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction<T>
where
    T: Integer,
{
    numerator: T,
    denominator: T,
}

impl<T> Fraction<T>
where
    T: Integer,
{
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(numerator: T, denominator: T) -> Self {
        Self { numerator, denominator }
    }

    pub fn numerator(&self) -> &T { &self.numerator }
    pub fn denominator(&self) -> &T { &self.denominator }
    pub fn into_parts(self) -> (T, T) { (self.numerator, self.denominator) }
}

impl<T> Fraction<T>
where
    T: Zero + Integer,
{
    pub fn try_new(numerator: T, denominator: T) -> Result<Self, FractionError> {
        let fraction = Fraction { numerator, denominator };
        fraction.validate()?;
        Ok(fraction)
    }
}

impl<T> AdditiveInverse for Fraction<T>
where
    T: Integer + Neg<Output = T> + Clone,
{
    fn inverse(&self) -> Self {
        Self {
            numerator: -self.numerator.clone(),
            denominator: self.denominator.clone(),
        }
    }

    fn invert(&mut self) {
        self.numerator = -self.numerator.clone();
    }
}

impl<T> MultiplicativeInverse for Fraction<T>
where
    T: Integer + Zero + Clone,
{
    type Output = Self;
    type Error = FractionError;

    fn try_inverse(&self) -> Result<Self::Output, Self::Error> {
        let mut fraction = self.clone();
        fraction.try_invert()?;
        Ok(fraction)
    }

    fn try_invert(&mut self) -> Result<(), Self::Error> {
        if self.numerator.is_zero() {
            return Err(FractionError::ZeroNumeratorInverse)
        }

        Ok(swap(&mut self.numerator, &mut self.denominator))
    }
}

impl<T> Validate for  Fraction<T>
where
    T: Integer + Zero,
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

impl<T> Fraction<T>
where
    T: Integer + GreatestCommonDivisor + One + PartialEq,
{
    pub fn is_reduced(&self) -> bool {
        self.numerator.gcd(&self.denominator).is_one()
    }
}

impl<T> Fraction<T>
where
    T: Integer + GreatestCommonDivisor + One + Div<Output = T> + Clone,
{
    pub fn reduce(&mut self) {
        let gcd = self.numerator.gcd(&self.denominator);

        if gcd.is_one() { return; }

        self.numerator = self.numerator.clone() / gcd.clone();
        self.denominator = self.denominator.clone() / gcd;
    }

    pub fn reduced(&self) -> Self {
        let mut fraction = self.clone();
        fraction.reduce();
        fraction
    }
}

impl<T> Mul for Fraction<T>
where
    T: Integer + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl<T> Div for Fraction<T>
where
    T: Integer + Zero + Clone + Mul<Output = T>,
{
    type Output = Result<Self, FractionError>;

    fn div(self, rhs: Self) -> Self::Output {
        let rhs = rhs.try_inverse()?;

        Ok(Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        })
    }
}

impl<T> Add for Fraction<T>
where
    T: Integer + Add<Output = T> + Mul<Output = T> + Clone,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator.clone() * rhs.denominator.clone()
                + rhs.numerator.clone() * self.denominator.clone(),
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl<T> Sub for Fraction<T>
where
    T: Integer + Sub<Output = T> + Mul<Output = T> + Clone,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator.clone() * rhs.denominator.clone()
                - rhs.numerator.clone() * self.denominator.clone(),
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl<T> Display for Fraction<T>
where
    T: Integer + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::MultiplicativeInverse;
    use crate::number::{Fraction, FractionError};
    use b3_core::validate::Validate;

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

        let fraction = result.unwrap();

        assert_eq!(fraction.numerator(), &1);
        assert_eq!(fraction.denominator(), &2);
    }

    #[test]
    fn fraction_try_new_zero_denominator() {
        let result = Fraction::try_new(1, 0);

        assert_eq!(
            result,
            Err(FractionError::ZeroDenominator)
        );
    }

    #[test]
    fn fraction_is_reduced_true() {
        let fraction = Fraction::new(3, 4);

        assert!(fraction.is_reduced());
    }

    #[test]
    fn fraction_is_reduced_false() {
        let fraction = Fraction::new(6, 8);

        assert!(!fraction.is_reduced());
    }

    #[test]
    fn fraction_reduced() {
        let fraction = Fraction::new(6, 8);

        let reduced = fraction.reduced();

        assert_eq!(reduced.numerator(), &3);
        assert_eq!(reduced.denominator(), &4);
    }

    #[test]
    fn fraction_reduce() {
        let mut fraction = Fraction::new(6, 8);

        fraction.reduce();

        assert_eq!(fraction.numerator(), &3);
        assert_eq!(fraction.denominator(), &4);
    }

    #[test]
    fn fraction_try_inverse_ok() {
        let fraction = Fraction::new(2, 3);

        let inverse = fraction.try_inverse().unwrap();

        assert_eq!(inverse.numerator(), &3);
        assert_eq!(inverse.denominator(), &2);
    }

    #[test]
    fn fraction_try_inverse_zero_numerator() {
        let fraction = Fraction::new(0, 3);

        assert_eq!(
            fraction.try_inverse(),
            Err(FractionError::ZeroNumeratorInverse)
        );
    }

    #[test]
    fn fraction_try_invert_ok() {
        let mut fraction = Fraction::new(2, 3);

        fraction.try_invert().unwrap();

        assert_eq!(fraction.numerator(), &3);
        assert_eq!(fraction.denominator(), &2);
    }

    #[test]
    fn fraction_try_invert_zero_numerator() {
        let mut fraction = Fraction::new(0, 3);

        assert_eq!(
            fraction.try_invert(),
            Err(FractionError::ZeroNumeratorInverse)
        );
    }

    #[test]
    fn fraction_reduce_already_reduced() {
        let mut fraction = Fraction::new(3, 4);

        fraction.reduce();

        assert_eq!(fraction.numerator(), &3);
        assert_eq!(fraction.denominator(), &4);
    }

    #[test]
    fn fraction_reduced_already_reduced() {
        let fraction = Fraction::new(3, 4);

        let reduced = fraction.reduced();

        assert_eq!(reduced, fraction);
    }

    #[test]
    fn fraction_mul() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(2, 3);

        let c = a * b;

        assert_eq!(c.numerator(), &2);
        assert_eq!(c.denominator(), &6);
    }

    #[test]
    fn fraction_div() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(2, 3);

        let c = (a / b).unwrap();

        assert_eq!(c.numerator(), &3);
        assert_eq!(c.denominator(), &4);
    }

    #[test]
    fn fraction_add() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(1, 3);

        let c = a + b;

        assert_eq!(c.numerator(), &5);
        assert_eq!(c.denominator(), &6);
    }

    #[test]
    fn fraction_sub() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(1, 3);

        let c = a - b;

        assert_eq!(c.numerator(), &1);
        assert_eq!(c.denominator(), &6);
    }
}
