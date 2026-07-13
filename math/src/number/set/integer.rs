use crate::algebra::identity::neutral::Neutral;
use crate::number::Signed;

pub trait Integer:
Neutral + Signed + PartialOrd
{}
