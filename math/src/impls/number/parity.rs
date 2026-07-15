use crate::number::{Parity, ParityCheck};

macro_rules! impl_parity {
    ($($t:ty),* $(,)?) => {$(
        impl ParityCheck for $t {
            fn parity(&self) -> Parity {
                if *self % 2 == 0 { Parity::Even }
                else { Parity::Odd }
            }
        }
    )*};
}

impl_parity!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity_even() {
        assert_eq!(2u32.parity(), Parity::Even);
    }

    #[test]
    fn parity_odd() {
        assert_eq!(3u32.parity(), Parity::Odd);
    }

    #[test]
    fn parity_negative_even() {
        assert_eq!((-2i32).parity(), Parity::Even);
    }

    #[test]
    fn parity_negative_odd() {
        assert_eq!((-3i32).parity(), Parity::Odd);
    }

    #[test]
    fn parity_zero() {
        assert_eq!(0u32.parity(), Parity::Even);
    }
}
