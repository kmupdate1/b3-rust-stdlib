use core::marker::PhantomData;
use crate::b3_invariant::B3Invariant;

pub struct B3Validate<T, I> {
    value: T,
    _marker: PhantomData<I>,
}

impl<T, I: B3Invariant<T>> B3Validate<T, I> {
    pub fn new(value: T) -> Option<Self> {
        if I::validate(&value) { Some(Self { value, _marker: PhantomData }) }
        else { None }
    }
}
