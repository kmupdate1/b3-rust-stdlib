pub trait One: PartialEq {
    fn one() -> Self;

    fn is_one(&self) -> bool
    where
        Self: Sized
    { self.eq(&Self::one()) }
}
