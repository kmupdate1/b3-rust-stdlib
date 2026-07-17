use crate::computer::{Finite, Finiteness};

impl Finite for i8 {
    fn finiteness(&self) -> Finiteness { Finiteness::Finite }
}

impl Finite for i16 {
    fn finiteness(&self) -> Finiteness { Finiteness::Finite }
}

impl Finite for i32 {
    fn finiteness(&self) -> Finiteness { Finiteness::Finite }
}

impl Finite for i64 {
    fn finiteness(&self) -> Finiteness { Finiteness::Finite }
}

impl Finite for i128 {
    fn finiteness(&self) -> Finiteness { Finiteness::Finite }
}

impl Finite for isize {
    fn finiteness(&self) -> Finiteness { Finiteness::Finite }
}

macro_rules! impl_finite_for_float {
    ($($t:ty),* $(,)?) => {
        $(impl Finite for $t {
            fn finiteness(&self) -> Finiteness {
                if self.is_nan() { Finiteness::NaN }
                else if self.is_infinite() { Finiteness::Infinite }
                else { Finiteness::Finite }
            }
        })*
    };
}

impl_finite_for_float!(f32, f64);
