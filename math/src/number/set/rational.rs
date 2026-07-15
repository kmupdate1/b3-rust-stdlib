use crate::number::Fraction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rational<T> {
    fraction: Fraction<T>,
}

impl<T> Rational<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(fraction: Fraction<T>) -> Self {
        Self { fraction }
    }

    pub fn fraction(&self) -> &Fraction<T> { &self.fraction }
    pub fn fraction_mut(&mut self) -> &mut Fraction<T> { &mut self.fraction }
    pub fn into_fraction(self) -> Fraction<T> { self.fraction }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_new() {
        let fraction = Fraction::new(7, 10);

        let rational = Rational::new(fraction);

        assert_eq!(rational.fraction(), &fraction);
    }

    #[test]
    fn rational_into_fraction() {
        let fraction = Fraction::new(7, 10);

        let rational = Rational::new(fraction);

        assert_eq!(rational.into_fraction(), fraction);
    }
}
