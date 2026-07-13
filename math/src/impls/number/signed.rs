use crate::number::{Sign, Signed};
use crate::relation::Ordering;

macro_rules! impl_signed {
    ($($t:ty),* $(,)?) => {
        $(
            impl Signed for $t {
                fn sign(&self) -> Sign {
                    match self.cmp(&0) {
                        Ordering::Less => Sign::Negative,
                        Ordering::Equal => Sign::Zero,
                        Ordering::Greater => Sign::Positive,
                    }
                }
            }
        )*
    };
}

impl_signed!(i8, i16, i32, i64, i128, isize);

impl Signed for f32 {
    fn sign(&self) -> Sign {
        match self.partial_cmp(&0.0) {
            Some(Ordering::Less) => Sign::Negative,
            Some(Ordering::Equal) => Sign::Zero,
            Some(Ordering::Greater) => Sign::Positive,
            None => Sign::Zero,
        }
    }
}

impl Signed for f64 {
    fn sign(&self) -> Sign {
        match self.partial_cmp(&0.0) {
            Some(Ordering::Less) => Sign::Negative,
            Some(Ordering::Equal) => Sign::Zero,
            Some(Ordering::Greater) => Sign::Positive,
            None => Sign::Zero,
        }
    }
}
