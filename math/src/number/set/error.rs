use crate::number::FractionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RationalError {
    Fraction(FractionError),
}

impl From<FractionError> for RationalError {
    fn from(e: FractionError) -> RationalError { Self::Fraction(e) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RealError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplexError {}
