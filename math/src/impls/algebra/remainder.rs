use crate::algebra::Remainder;

macro_rules! impl_remainder {
    ($($t:ty),* $(,)?) => {$(
        impl Remainder<Self> for $t {
            type Output = Self;
            fn remainder(&self, rhs: Self) -> Self::Output { *self % rhs }
        }
    )*}
}

impl_remainder!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remainder_i32() {
        assert_eq!(7i32.remainder(2), 1);
    }

    #[test]
    fn remainder_u32() {
        assert_eq!(8u32.remainder(2), 0);
    }

    #[test]
    fn remainder_negative() {
        assert_eq!((-7i32).remainder(2), -1);
    }
}
