use core::marker::PhantomData;
use crate::b3_error::B3Error;
use crate::b3_invariant::B3Invariant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct B3Validate<T, I> {
    value: T,
    _marker: PhantomData<I>,
}

impl<T, I: B3Invariant<T>> B3Validate<T, I> {
    pub fn new(value: T) -> Result<Self, B3Error> {
        if I::validate(&value) {
            Ok(Self { value, _marker: PhantomData })
        } else {
            Err(B3Error::Invalid)
        }
    }

    /// 不変参照を返す
    pub fn get(&self) -> &T { &self.value }
    /// 所有権を返す
    pub fn into_inner(self) -> T { self.value }
}
