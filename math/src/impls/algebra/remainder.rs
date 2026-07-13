use crate::algebra::Remainder;

impl Remainder for i8 {
    type Output = i8;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for i16 {
    type Output = i16;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for i32 {
    type Output = i32;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for i64 {
    type Output = i64;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for i128 {
    type Output = i128;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for isize {
    type Output = isize;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for u8 {
    type Output = u8;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for u16 {
    type Output = u16;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for u32 {
    type Output = u32;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for u64 {
    type Output = u64;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for u128 {
    type Output = u128;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}

impl Remainder for usize {
    type Output = usize;
    fn remainder(&self, rhs: Self) -> Self::Output { self % rhs }
}
