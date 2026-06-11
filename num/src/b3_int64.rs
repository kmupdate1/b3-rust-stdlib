use b3_core::b3_invariant::B3Invariant;
use b3_core::b3_validate::B3Validate;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct B3Int64(pub i64);
#[derive(Debug)]
pub struct B3Int64Rule;

impl B3Invariant<B3Int64> for B3Int64Rule {
    fn validate(value: &B3Int64) -> bool {
        value.0 > 0
    }
}

pub type ValidateB3Int64 = B3Validate<B3Int64, B3Int64Rule>;

#[cfg(test)]
mod tests {
    use b3_core::b3_error::B3Error;
    use super::*;

    #[test]
    fn test_b3_int64() {
        let raw = B3Int64(10);
        let res = ValidateB3Int64::new(raw);

        assert!(res.is_ok());
        assert_eq!(res.unwrap().get().0, raw.0);
    }

    #[test]
    fn test_b3_int64_zero() {
        let raw_zero = B3Int64(0);
        let res = ValidateB3Int64::new(raw_zero);

        assert_eq!(res.unwrap_err(), B3Error::Invalid);
    }

    #[test]
    fn test_b3_int64_neg() {
        let raw_neg = B3Int64(-10);
        let res = ValidateB3Int64::new(raw_neg);

        assert_eq!(res.unwrap_err(), B3Error::Invalid);
    }
}
