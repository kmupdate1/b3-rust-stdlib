use crate::number::Magnitude;

macro_rules! impl_magnitude_for_unsigned {
    ($($t:ty),* $(,)?) => {
        $(impl Magnitude for $t {
            type Output = $t;
            fn magnitude(&self) -> Self::Output { *self }
        })*
    };
}

macro_rules! impl_magnitude_for_signed {
    ($($signed:ty => $unsigned:ty),* $(,)?) => {
        $(impl Magnitude for $signed {
            type Output = $unsigned;
            fn magnitude(&self) -> Self::Output { self.unsigned_abs() }
        })*
    };
}

macro_rules! impl_magnitude_for_float {
    ($($t:ty),* $(,)?) => {
        $(impl Magnitude for $t {
            type Output = $t;
            fn magnitude(&self) -> Self::Output { self.abs() }
        })*
    };
}

impl_magnitude_for_unsigned!(u8, u16, u32, u64, u128, usize);
impl_magnitude_for_signed!(
    i8 => u8,
    i16 => u16,
    i32 => u32,
    i64 => u64,
    i128 => u128,
    isize => usize,
);
impl_magnitude_for_float!(f32, f64);

#[cfg(test)]
mod tests {
    use crate::number::Magnitude;

    #[test]
    fn signed_integer_magnitude() {
        assert_eq!((-5i8).magnitude(), 5u8);
        assert_eq!((-100i32).magnitude(), 100u32);
        assert_eq!((-123i64).magnitude(), 123u64);
    }

    #[test]
    fn signed_integer_min_value() {
        assert_eq!(i8::MIN.magnitude(), 128u8);
        assert_eq!(i16::MIN.magnitude(), 32768u16);
    }

    #[test]
    fn unsigned_integer_magnitude() {
        assert_eq!(5u8.magnitude(), 5u8);
        assert_eq!(100u32.magnitude(), 100u32);
    }

    #[test]
    fn float_magnitude() {
        assert_eq!((-3.5f32).magnitude(), 3.5f32);
        assert_eq!((-7.25f64).magnitude(), 7.25f64);
    }
}
