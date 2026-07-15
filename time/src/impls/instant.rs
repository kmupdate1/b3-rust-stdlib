use crate::instant::Instant;
use crate::traits::current::Current;

impl Current for Instant<std::time::Instant> {
    fn now() -> Self {
        Self::from_trusted(std::time::Instant::now())
    }
}

impl Current for Instant<std::time::SystemTime> {
    fn now() -> Self {
        Self::from_trusted(std::time::SystemTime::now())
    }
}

#[cfg(test)]
mod tests {
    use crate::instant::Instant;
    use crate::traits::current::Current;

    #[test]
    fn std_time_instant_now() {
        let _ = Instant::<std::time::Instant>::now();
    }

    #[test]
    fn std_time_system_time_now() {
        let _ = Instant::<std::time::SystemTime>::now();
    }
}
