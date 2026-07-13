pub trait Zero: PartialEq {
    fn zero() -> Self;

    fn is_zero(&self) -> bool
    where
        Self: Sized
    { self.eq(&Self::zero()) }
}
