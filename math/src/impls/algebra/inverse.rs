use crate::algebra::AdditiveInverse;

impl AdditiveInverse for i8 {
    fn inverse(&self) -> Self { -self }
}

impl AdditiveInverse for i16 {
    fn inverse(&self) -> Self { -self }
}

impl AdditiveInverse for i32 {
    fn inverse(&self) -> Self { -self }
}

impl AdditiveInverse for i64 {
    fn inverse(&self) -> Self { -self }
}

impl AdditiveInverse for i128 {
    fn inverse(&self) -> Self { -self }
}

impl AdditiveInverse for isize {
    fn inverse(&self) -> Self { -self }
}
