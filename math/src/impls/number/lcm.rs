use crate::algebra::identity::zero::Zero;
use crate::algebra::operation::inverse::AdditiveInverse;
use crate::number::arithmetic::gcd::GreatestCommonDivisor;
use crate::number::lcm::LeastCommonMultiple;
use crate::number::property::signed::Signed;
use crate::number::Sign;

macro_rules! impl_lcm {
    ($t:ty, $normalize:expr) => {
        impl LeastCommonMultiple for $t {
            fn lcm(&self, rhs: &Self) -> Self {
                if self.is_zero() || rhs.is_zero() { return Self::zero(); }
                
                let gcd = self.gcd(rhs);
                let res = (*self / gcd) * *rhs;
                
                $normalize(res)
            }
        }
    };
}

macro_rules! impl_unsigned_lcm {
    ($($t:ty),* $(,)?) => { $(impl_lcm!($t, |x| x);)* };
}

macro_rules! impl_signed_lcm {
    ($($t:ty),* $(,)?) => {
        $(impl_lcm!($t, |x: $t| match x.sign() {
            Sign::Negative => x.inverse(),
            _ => x,
        });)*
    };
}

impl_unsigned_lcm!(u8, u16, u32, u64, u128, usize);
impl_signed_lcm!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm_positive() {
        assert_eq!(12i32.lcm(&18), 36);
        assert_eq!(21i32.lcm(&6), 42);
    }

    #[test]
    fn lcm_coprime() {
        assert_eq!(5i32.lcm(&7), 35);
    }

    #[test]
    fn lcm_negative() {
        assert_eq!((-12i32).lcm(&18), 36);
        assert_eq!(12i32.lcm(&(-18)), 36);
        assert_eq!((-12i32).lcm(&(-18)), 36);
    }

    #[test]
    fn lcm_zero() {
        assert_eq!(0i32.lcm(&10), 0);
        assert_eq!(10i32.lcm(&0), 0);
    }

    #[test]
    fn lcm_same_value() {
        assert_eq!(15i32.lcm(&15), 15);
    }

    #[test]
    fn lcm_divisible() {
        assert_eq!(4i32.lcm(&16), 16);
        assert_eq!(16i32.lcm(&4), 16);
    }
}
