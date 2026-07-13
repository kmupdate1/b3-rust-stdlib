pub trait GreatestCommonDivisor<Rhs = Self> {
    fn gcd(&self, rhs: &Rhs) -> Self;
}
