use crate::algebra::identity::identity::Identity;
use crate::number::Signed;

pub trait Integer:
Identity + Signed + PartialOrd
{}
