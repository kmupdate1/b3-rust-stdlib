use b3_math::number::FractionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RatioError {
    Fraction(FractionError),
}

impl From<FractionError> for RatioError {
    fn from(e: FractionError) -> Self { Self::Fraction(e) }
}
