use crate::algebra::Zero;
use crate::number::ComplexError;
use b3_core::validate::Validate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> Complex<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(real: T, imaginary: T) -> Complex<T> {
        Complex { real, imaginary }
    }

    pub fn real(&self) -> &T { &self.real }
    pub fn imaginary(&self) -> &T { &self.imaginary }
    pub fn into_parts(self) -> (T, T) { (self.real, self.imaginary) }
}

impl<T> Complex<T>
where
    T: Zero,
{
    pub fn try_new(real: T, imaginary: T) -> Result<Self, ComplexError> {
        let complex = Complex { real, imaginary };
        complex.validate()?;
        Ok(complex)
    }
}

impl<T> Validate for Complex<T>
where
    T: Zero,
{
    type Error = ComplexError;

    fn validate(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_new() {
        let complex = Complex::new(1.0, 2.0);

        assert_eq!(complex.real(), &1.0);
        assert_eq!(complex.imaginary(), &2.0);
    }

    #[test]
    fn complex_into_parts() {
        let complex = Complex::new(1.0, 2.0);

        assert_eq!(complex.into_parts(), (1.0, 2.0));
    }

    #[test]
    fn complex_try_new_ok() {
        let complex = Complex::try_new(1.0, 2.0);

        assert!(complex.is_ok());
    }

    #[test]
    fn complex_validate_ok() {
        let complex = Complex::new(1.0, 2.0);

        assert_eq!(complex.validate(), Ok(()));
    }
}
