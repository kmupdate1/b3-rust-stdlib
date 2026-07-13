use std::ops::Neg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
            Sign::Zero => Sign::Zero,
        }
    }
}

impl Sign {
    pub fn is_positive(&self) -> bool {
        matches!(self, Sign::Positive)
    }

    pub fn is_negative(&self) -> bool {
        matches!(self, Sign::Negative)
    }

    pub fn is_zero(&self) -> bool {
        matches!(self, Sign::Zero)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_check() {
        assert!(Sign::Positive.is_positive());
        assert!(!Sign::Positive.is_negative());
        assert!(!Sign::Positive.is_zero());

        assert!(Sign::Negative.is_negative());
        assert!(!Sign::Negative.is_positive());

        assert!(Sign::Zero.is_zero());
    }

    #[test]
    fn sign_inverse() {
        assert_eq!(-Sign::Positive, Sign::Negative);
        assert_eq!(-Sign::Negative, Sign::Positive);
        assert_eq!(-Sign::Zero, Sign::Zero);
    }
}
