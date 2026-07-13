use crate::algebra::identity::Identity;

macro_rules! impl_identity {
    ($($t:ty),* $(,)?) => {
        $(impl Identity for $t {})*
    };
}

impl_identity!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
);
