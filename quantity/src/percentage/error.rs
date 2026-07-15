use crate::RatioError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PercentageError {
    Ratio(RatioError),
}

impl From<RatioError> for PercentageError {
    fn from(e: RatioError) -> Self { Self::Ratio(e) }
}
