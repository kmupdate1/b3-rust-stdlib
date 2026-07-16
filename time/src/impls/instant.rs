use crate::duration::Duration;
use crate::instant::Instant;
use crate::traits::current::Current;
use std::ops::{Add, Sub};
use std::time::SystemTime;

impl Current for Instant<std::time::Instant> {
    fn now() -> Self {
        Self::from_trusted(std::time::Instant::now())
    }
}

impl Current for Instant<SystemTime> {
    fn now() -> Self {
        Self::from_trusted(SystemTime::now())
    }
}

impl Add<Duration<std::time::Duration>>
for Instant<std::time::Instant> {
    type Output = Instant<std::time::Instant>;

    fn add(self, rhs: Duration<std::time::Duration>) -> Self::Output {
        todo!()
    }
}

impl Add<Duration<std::time::Duration>>
for Instant<SystemTime> {
    type Output = Instant<SystemTime>;

    fn add(self, rhs: Duration<std::time::Duration>) -> Self::Output {
        todo!()
    }
}

impl Sub<Duration<std::time::Duration>>
for Instant<std::time::Instant> {
    type Output = Instant<std::time::Instant>;

    fn sub(self, rhs: Duration<std::time::Duration>) -> Self::Output {
        todo!()
    }
}

impl Sub<Duration<std::time::Duration>>
for Instant<SystemTime> {
    type Output = Instant<SystemTime>;

    fn sub(self, rhs: Duration<std::time::Duration>) -> Self::Output {
        todo!()
    }
}

impl Sub<Instant<std::time::Instant>>
for Instant<std::time::Instant> {
    type Output = Duration<std::time::Duration>;

    fn sub(self, rhs: Instant<std::time::Instant>) -> Self::Output {
        todo!()
    }
}

impl Sub<Instant<SystemTime>>
for Instant<SystemTime>{
    type Output = Duration<SystemTime>;

    fn sub(self, rhs: Instant<SystemTime>) -> Self::Output {
        todo!()
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
