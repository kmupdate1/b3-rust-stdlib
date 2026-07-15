#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Real<T> {
    value: T,
}

impl<T> Real<T> {
    #[deprecated(since = "0.2.0", note = "use `try_new` instead")]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T { &self.value }
    pub fn into_inner(self) -> T { self.value }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_new() {
        let real = Real::new(3.14);

        assert_eq!(real.value(), &3.14);
    }

    #[test]
    fn real_into_inner() {
        let real = Real::new(3.14);

        assert_eq!(real.into_inner(), 3.14);
    }
}
