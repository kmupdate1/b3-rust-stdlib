use crate::number::Integer;

macro_rules! impl_integer {
    ($($t:ty),* $(,)?) => {
        $(impl Integer for $t {})*
    };
}

impl_integer!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_integer<T: Integer>() {}

    #[test]
    fn signed_integer_types_implement_integer() {
        assert_integer::<i8>();
        assert_integer::<i16>();
        assert_integer::<i32>();
        assert_integer::<i64>();
        assert_integer::<i128>();
        assert_integer::<isize>();
    }
}
