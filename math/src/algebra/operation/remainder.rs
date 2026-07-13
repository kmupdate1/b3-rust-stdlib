pub trait Remainder<Rhs = Self> {
    type Output;
    
    fn remainder(&self, rhs: Rhs) -> Self::Output;
}
