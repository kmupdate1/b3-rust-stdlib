use std::cmp::PartialEq;

/// Equality with tolerance.
pub trait ApproximateEq<Rhs = Self> {
    type Tolerance;
    
    fn approx_eq(&self, rhs: &Rhs, tolerance: Self::Tolerance) -> bool;
}
