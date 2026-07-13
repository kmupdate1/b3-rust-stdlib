use crate::number::{Parity, ParityCheck};

macro_rules! impl_parity {
    ($($t:ty),* $(,)?) => {
        $(
            impl ParityCheck for $t {
                fn parity(&self) -> Parity {
                    if *self % 2 == 0 { Parity::Even }
                    else { Parity::Odd }
                }
            }
        )*
    };
}

impl_parity!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
);
