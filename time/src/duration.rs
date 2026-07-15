use b3_core::error::Result;
use b3_core::validate::Validate;
use b3_math::algebra::Zero;
use crate::error::DurationError;

pub struct Duration<T> {
    value: T,
}

impl<T> Duration<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T { &self.value }
    pub fn value_mut(&mut self) -> &mut T { &mut self.value }
    pub fn into_inner(self) -> T { self.value }
}

impl<T> Duration<T>
where
    T: Zero,
{
    pub fn try_new(value: T) -> Result<Self, DurationError> {
        let duration = Duration { value };
        duration.validate()?;
        Ok(duration)
    }
}

impl<T> Validate for Duration<T> {
    type Error = DurationError;

    fn validate(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_new() {
        let duration = Duration::new(5i32);

        assert_eq!(duration.value(), &5i32);
    }

    #[test]
    fn duration_into_inner() {
        let duration = Duration::new(5i32);

        assert_eq!(duration.into_inner(), 5i32);
    }

    #[test]
    fn duration_try_new_ok() {
        let duration = Duration::try_new(5i32);

        assert!(duration.is_ok());
    }

    #[test]
    fn duration_try_new_err() {}

    #[test]
    fn duration_validate_ok() {
        let duration = Duration::try_new(5i32);

        assert!(duration.is_ok());
    }

    #[test]
    fn duration_validate_err() {}
}
