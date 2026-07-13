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

impl Finite for f32 {
    fn finiteness(&self) -> Finiteness {
        if self.is_nan() { Finiteness::NaN }
        else if self.is_finite() { Finiteness::Finite }
        else { Finiteness::Infinite }
    }
}

impl Finite for f64 {
    fn finiteness(&self) -> Finiteness {
        if self.is_nan() { Finiteness::NaN }
        else if self.is_finite() { Finiteness::Finite }
        else { Finiteness::Infinite }
    }
}
