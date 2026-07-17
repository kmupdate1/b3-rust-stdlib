use crate::RatioError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PercentageError {
    Proportion(ProportionError),
}

impl From<ProportionError> for PercentageError {
    fn from(e: ProportionError) -> Self { Self::Proportion(e) }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProportionError {
    Ratio(RatioError),
    Negative,
    GraterThenOne,
}

impl From<RatioError> for ProportionError {
    fn from(e: RatioError) -> Self { Self::Ratio(e) }
}
