use crate::algebra::{Add, AdditiveInverse, Div, Mul, MultiplicativeInverse, One, Sub, Zero};
use crate::number::gcd::GreatestCommonDivisor;
use crate::number::{ComplexError, Integer, Real};
use b3_core::error::Result;
use b3_core::validate::Validate;

pub struct Complex<T: Integer> {
    real: Real<T>,
    imaginary: Real<T>,
}

impl<T> Complex<T>
where
    T: Integer,
{
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(real: Real<T>, imaginary: Real<T>) -> Complex<T> {
        Complex { real, imaginary }
    }

    pub fn real(&self) -> &Real<T> { &self.real }
    pub fn imaginary(&self) -> &Real<T> { &self.imaginary }
    pub fn into_parts(self) -> (Real<T>, Real<T>) { (self.real, self.imaginary) }
}

impl<T> Complex<T>
where
    T: Integer,
{
    pub fn try_new(real: Real<T>, imaginary: Real<T>) -> Result<Self, ComplexError> {
        let complex = Complex { real, imaginary };
        complex.validate()?;
        Ok(complex)
    }
}

impl<T> Validate for Complex<T>
where
    T: Integer,
{
    type Error = ComplexError;

    fn validate(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T> Add for Complex<T>
where
    T: Integer + Add<Output = T> + Clone,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (a, b) = self.into_parts();
        let (c, d) = rhs.into_parts();

        Self { real: a + c, imaginary: b + d }
    }
}
impl<T> Sub for Complex<T>
where
    T: Integer + Add<Output = T> + Div<Output = T> + GreatestCommonDivisor + Clone,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { self + rhs.inverse() }
}
impl<T> Mul for Complex<T>
where
    T: Integer
    + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
    + GreatestCommonDivisor + Clone,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b) = self.into_parts();
        let (c, d) = rhs.into_parts();

        let real = a.clone() * c.clone() - b.clone() * d.clone();
        let imaginary = a * d + b * c;

        Self { real, imaginary }
    }
}
impl<T> Div for Complex<T>
where
    T: Integer
    + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
    + GreatestCommonDivisor + Clone,
{
    type Output = Result<Self, ComplexError>;
    fn div(self, rhs: Self) -> Self::Output { Ok(self * rhs.try_inverse()?) }
}

impl<T> Complex<T>
where
    T: Integer + Clone,
{
    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real.clone(),
            imaginary: self.imaginary.inverse(),
        }
    }
}

impl<T> AdditiveInverse for Complex<T>
where
    T: Integer + Clone,
{
    fn inverse(&self) -> Self {
        Self {
            real: self.real.inverse(),
            imaginary: self.imaginary.inverse(),
        }
    }
    fn invert(&mut self) { *self = self.inverse() }
}

impl<T> MultiplicativeInverse for Complex<T>
where
    T: Integer + Clone
    + Add<Output = T> + Div<Output = T> + Mul<Output = T>
    + GreatestCommonDivisor,
{
    type Output = Self;
    type Error = ComplexError;

    fn try_inverse(&self) -> Result<Self::Output, Self::Error> {
        let conj = self.conjugate();
        let deno = self.real.clone().exp() + self.imaginary.clone().exp();

        let real = (conj.real / deno.clone())?;
        let imaginary = (conj.imaginary / deno)?;

        Ok(Self { real, imaginary })
    }
    fn try_invert(&mut self) -> Result<(), Self::Error> {
        *self = self.try_inverse()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::One;
    use crate::number::Rational;

    #[test]
    fn complex_into_parts() {
        let complex = Complex::<i32>::new(Real::one(), Real::one());

        let (real, imaginary) = complex.into_parts();

        assert_eq!(real, Real::one());
        assert_eq!(imaginary, Real::one());
    }

    #[test]
    fn complex_try_new_ok() {
        let complex = Complex::<i32>::try_new(
            Real::one(),
            Real::one(),
        );

        assert!(complex.is_ok());
    }

    #[test]
    fn complex_validate_ok() {
        let complex = Complex::<i32>::new(
            Real::one(),
            Real::one(),
        );

        assert_eq!(complex.validate(), Ok(()));
    }

    #[test]
    fn complex_new() {
        let complex = Complex::<i32>::new(
            Real::one(),
            Real::one(),
        );

        assert_eq!(complex.real(), &Real::one());
        assert_eq!(complex.imaginary(), &Real::one());
    }

    #[test]
    fn complex_add() {
        let a = Complex::new(
            Real::from(Rational::from_parts(1, 1).unwrap()),
            Real::from(Rational::from_parts(2, 1).unwrap()),
        );

        let b = Complex::new(
            Real::from(Rational::from_parts(3, 1).unwrap()),
            Real::from(Rational::from_parts(4, 1).unwrap()),
        );

        let c = a + b;

        assert_eq!(*c.real(), Real::from(Rational::from_parts(4, 1).unwrap()));
        assert_eq!(*c.imaginary(), Real::from(Rational::from_parts(6, 1).unwrap()));
    }

    #[test]
    fn complex_inverse() {
        let a = Complex::new(
            Real::from(Rational::from_parts(1, 1).unwrap()),
            Real::from(Rational::from_parts(2, 1).unwrap()),
        );

        let b = a.inverse();

        assert_eq!(*b.real(), Real::from(Rational::from_parts(-1, 1).unwrap()));
        assert_eq!(*b.imaginary(), Real::from(Rational::from_parts(-2, 1).unwrap()));
    }

    #[test]
    fn complex_sub() {
        let a = Complex::new(
            Real::from(Rational::from_parts(3, 1).unwrap()),
            Real::from(Rational::from_parts(4, 1).unwrap()),
        );

        let b = Complex::new(
            Real::from(Rational::from_parts(1, 1).unwrap()),
            Real::from(Rational::from_parts(2, 1).unwrap()),
        );

        let c = a - b;

        assert_eq!(*c.real(), Real::from(Rational::from_parts(2, 1).unwrap()));
        assert_eq!(*c.imaginary(), Real::from(Rational::from_parts(2, 1).unwrap()));
    }

    #[test]
    fn complex_conjugate() {
        let a = Complex::new(
            Real::from(Rational::from_parts(3, 1).unwrap()),
            Real::from(Rational::from_parts(4, 1).unwrap()),
        );

        let b = a.conjugate();

        assert_eq!(*b.real(), Real::from(Rational::from_parts(3, 1).unwrap()));
        assert_eq!(*b.imaginary(), Real::from(Rational::from_parts(-4, 1).unwrap()));
    }

    #[test]
    fn complex_mul() {
        let a = Complex::new(
            Real::from(Rational::from_parts(1, 1).unwrap()),
            Real::from(Rational::from_parts(1, 1).unwrap()),
        );

        let b = Complex::new(
            Real::from(Rational::from_parts(1, 1).unwrap()),
            Real::from(Rational::from_parts(-1, 1).unwrap()),
        );

        let c = a * b;

        assert_eq!(*c.real(), Real::from(Rational::from_parts(2, 1).unwrap()));
        assert_eq!(*c.imaginary(), Real::from(Rational::from_parts(0, 1).unwrap()));
    }
}
