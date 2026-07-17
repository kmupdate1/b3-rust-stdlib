use crate::{ProportionError, Ratio};
use b3_core::error::Result;
use b3_math::number::Integer;

/**
 * 全体に対する割合を表す。
 *
 * Ratio のうち 0 <= ratio <= 1 を満たすもの。
 *
 * 現時点では使用シーンを検討中。
 * 共通概念として必要性が明確に慣れば実装する。
 */

pub struct Proportion<T: Integer> {
    ratio: Ratio<T>,
}

impl<T> Proportion<T>
where
    T: Integer,
{
    pub fn ratio(&self) -> &Ratio<T> { &self.ratio }
}

impl<T> Proportion<T>
where
    T: Integer + PartialEq,
{
    pub fn from_parts(part: T, whole: T) -> Result<Self, ProportionError> {
        let ratio = Ratio::from_parts(part, whole)?;

        Self::try_new(ratio)
    }

    pub fn try_new(ratio: Ratio<T>) -> Result<Self, ProportionError> {
        if ratio.left() < &T::zero() {
            return Err(ProportionError::Negative);
        }

        if ratio.left() > ratio.right() {
            return Err(ProportionError::GraterThenOne);
        }

        Ok(Self { ratio })
    }
}

#[cfg(test)]
mod tests {
    use crate::Proportion;

    #[test]
    fn proportion_zero() {
        assert!(Proportion::from_parts(0, 10).is_ok());
    }

    #[test]
    fn proportion_one() {
        assert!(Proportion::from_parts(10, 10).is_ok());
    }

    #[test]
    fn proportion_less_than_one() {
        assert!(Proportion::from_parts(3, 10).is_ok());
    }

    #[test]
    fn proportion_greater_than_one() {
        assert!(Proportion::from_parts(11, 10).is_err());
    }

    #[test]
    fn proportion_negative() {
        assert!(Proportion::from_parts(-1, 10).is_err());
    }
}
