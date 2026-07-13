use crate::algebra::identity::zero::Zero;
use crate::algebra::operation::inverse::AdditiveInverse;
use crate::algebra::operation::remainder::Remainder;
use crate::number::gcd::GreatestCommonDivisor;
use crate::number::property::signed::Signed;
use crate::number::Sign;

macro_rules! impl_gcd {
    ($t:ty, $normalize:expr) => {
        impl GreatestCommonDivisor for $t {
            fn gcd(&self, rhs: &Self) -> Self {
                let mut a = *self;
                let mut b = *rhs;

                while !b.is_zero() {
                    let tmp = a.remainder(b);
                    a = b;
                    b = tmp;
                }

                $normalize(a)
            }
        }
    };
}

macro_rules! impl_unsigned_gcd {
    ($($t:ty),* $(,)?) => { $(impl_gcd!($t, |x| x);)* };
}

macro_rules! impl_signed_gcd {
    ($($t:ty),* $(,)?) => {
        $(impl_gcd!($t, |x: $t| match x.sign() {
            Sign::Negative => x.inverse(),
            _ => x,
        });)*
    };
}

impl_unsigned_gcd!(u8, u16, u32, u64, u128, usize);
impl_signed_gcd!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_positive() {
        assert_eq!(12i32.gcd(&18), 6);
        assert_eq!(54i32.gcd(&24), 6);
    }

    #[test]
    fn gcd_coprime() {
        assert_eq!(17i32.gcd(&13), 1);
    }

    #[test]
    fn gcd_negative() {
        assert_eq!((-12i32).gcd(&18), 6);
        assert_eq!(12i32.gcd(&(-18)), 6);
        assert_eq!((-12i32).gcd(&(-18)), 6);
    }

    #[test]
    fn gcd_zero() {
        assert_eq!(0i32.gcd(&10), 10);
        assert_eq!(10i32.gcd(&0), 10);
    }

    #[test]
    fn gcd_both_zero() {
        assert_eq!(0i32.gcd(&0), 0);
    }
}
