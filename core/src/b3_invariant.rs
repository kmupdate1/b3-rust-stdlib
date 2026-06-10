/// すべてのB3型が満たすべき契約
pub trait B3Invariant<T> {
    fn validate(value: &T) -> bool;
}
