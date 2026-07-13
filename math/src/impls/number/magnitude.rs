use crate::number::Magnitude;

impl Magnitude for i8 {
    type Output = u8;

    fn magnitude(&self) -> Self::Output { self.unsigned_abs() }
}

impl Magnitude for i16 {
    type Output = u16;
    fn magnitude(&self) -> Self::Output { self.unsigned_abs() }
}

impl Magnitude for i32 {
    type Output = u32;
    fn magnitude(&self) -> Self::Output { self.unsigned_abs() }
}

impl Magnitude for i64 {
    type Output = u64;
    fn magnitude(&self) -> Self::Output { self.unsigned_abs() }
}

impl Magnitude for i128 {
    type Output = u128;
    fn magnitude(&self) -> Self::Output { self.unsigned_abs() }
}

impl Magnitude for isize {
    type Output = usize;
    fn magnitude(&self) -> Self::Output { self.unsigned_abs() }
}

impl Magnitude for u8 {
    type Output = u8;
    fn magnitude(&self) -> Self::Output { *self }
}

impl Magnitude for u16 {
    type Output = u16;
    fn magnitude(&self) -> Self::Output { *self }
}

impl Magnitude for u32 {
    type Output = u32;
    fn magnitude(&self) -> Self::Output { *self }
}

impl Magnitude for u64 {
    type Output = u64;
    fn magnitude(&self) -> Self::Output { *self }
}

impl Magnitude for u128 {
    type Output = u128;
    fn magnitude(&self) -> Self::Output { *self }
}

impl Magnitude for usize {
    type Output = usize;
    fn magnitude(&self) -> Self::Output { *self }
}
