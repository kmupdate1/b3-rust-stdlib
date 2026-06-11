use b3_core::b3_invariant::B3Invariant;
use b3_core::b3_validate::B3Validate;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct B3Int64(pub i64);
pub struct B3Int64Rule;

impl B3Invariant<B3Int64> for B3Int64Rule {
    fn validate(value: &B3Int64) -> bool {
        value.0 > 0
    }
}

pub type ValidatedB3Int64 = B3Validate<B3Int64, B3Int64Rule>;
