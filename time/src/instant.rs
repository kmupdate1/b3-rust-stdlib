use b3_core::validate::Validate;
use b3_math::algebra::Zero;
use crate::error::InstantError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instant<T> {
    value: T,
}

impl<T> Instant<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn timestamp(&self) -> &T { &self.value }
    pub fn timestamp_mut(&mut self) -> &mut T { &mut self.value }
    pub fn into_timestamp(self) -> T { self.value }
}

impl<T> Instant<T>
where
    T: Zero,
{
    pub fn try_new(value: T) -> Result<Self, InstantError> {
        let instant = Self { value };
        instant.validate()?;
        Ok(instant)
    }
}

impl<T> Validate for Instant<T> {
    type Error = InstantError;

    fn validate(&self) -> b3_core::error::Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instant_new() {
        let instant = Instant::new(1000i16);

        assert_eq!(instant.timestamp(), &1000i16);
    }

    #[test]
    fn instant_into_timestamp() {
        let instant = Instant::new(1000i16);

        assert_eq!(instant.into_timestamp(), 1000i16);
    }

    #[test]
    fn instant_try_new_ok() {
        let instant = Instant::try_new(1000i16);

        assert!(instant.is_ok());
    }

    #[test]
    fn instant_try_new_err() {}

    #[test]
    fn instant_validate_ok() {
        let instant = Instant::new(1000i16);

        assert!(instant.validate().is_ok());
    }

    #[test]
    fn instant_validate_err() {}
}
