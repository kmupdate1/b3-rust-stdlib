use b3_core::b3_invariant::B3Invariant;
use b3_core::b3_validate::B3Validate;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct B3UInt64(pub u64);
pub struct B3UInt64Rule;

impl B3Invariant<B3UInt64> for B3UInt64Rule {
    fn validate(value: &B3UInt64) -> bool {
        value.0 > 0
    }
}

pub type ValidateB3UInt64 = B3Validate<B3UInt64, B3UInt64Rule>;
