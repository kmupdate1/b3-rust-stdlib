use super::Bound;
use crate::relation::Ordering;

pub struct Threshold<T> {
    value: T,
    order: Ordering,
    bound: Bound,
}

impl<T> Threshold<T> {
    pub fn new(value: T, order: Ordering, bound: Bound) -> Self {
        Self { value, order, bound }
    }

    pub fn value(&self) -> &T { &self.value }
    pub fn order(&self) -> Ordering { self.order }
    pub fn bound(&self) -> Bound { self.bound }
}

impl<T> Threshold<T> {
    pub fn less_than(value: T) -> Self {
        Self {
            value,
            order: Ordering::Less,
            bound: Bound::Exclusive,
        }
    }

    pub fn less_equal(value: T) -> Self {
        Self {
            value,
            order: Ordering::Less,
            bound: Bound::Inclusive,
        }
    }

    pub fn greater_than(value: T) -> Self {
        Self {
            value,
            order: Ordering::Greater,
            bound: Bound::Exclusive,
        }
    }

    pub fn greater_equal(value: T) -> Self {
        Self {
            value,
            order: Ordering::Greater,
            bound: Bound::Inclusive,
        }
    }

    pub fn equal(value: T) -> Self {
        Self {
            value,
            order: Ordering::Equal,
            bound: Bound::Inclusive,
        }
    }
}

impl<T> Threshold<T>
where
    T: PartialOrd + PartialEq,
{
    pub fn contains(&self, value: &T) -> bool {
        match (self.order, self.bound) {
            (Ordering::Less, Bound::Inclusive) => value <= &self.value,
            (Ordering::Less, Bound::Exclusive) => value < &self.value,

            (Ordering::Greater, Bound::Inclusive) => value >= &self.value,
            (Ordering::Greater, Bound::Exclusive) => value > &self.value,

            (Ordering::Equal, _) => value == &self.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::Bound;

    #[test]
    fn threshold_less_exclusive() {
        let t = Threshold::new(
            30,
            Ordering::Less,
            Bound::Exclusive,
        );

        assert!(t.contains(&20));
        assert!(t.contains(&29));
        assert!(!t.contains(&30));
        assert!(!t.contains(&40));
    }

    #[test]
    fn threshold_less_inclusive() {
        let t = Threshold::new(
            30,
            Ordering::Less,
            Bound::Inclusive,
        );

        assert!(t.contains(&20));
        assert!(t.contains(&30));
        assert!(!t.contains(&31));
    }

    #[test]
    fn threshold_greater_exclusive() {
        let t = Threshold::new(
            30,
            Ordering::Greater,
            Bound::Exclusive,
        );

        assert!(!t.contains(&30));
        assert!(t.contains(&31));
    }

    #[test]
    fn threshold_greater_inclusive() {
        let t = Threshold::new(
            30,
            Ordering::Greater,
            Bound::Inclusive,
        );

        assert!(t.contains(&30));
        assert!(t.contains(&31));
    }

    #[test]
    fn threshold_equal() {
        let t = Threshold::new(
            30,
            Ordering::Equal,
            Bound::Inclusive,
        );

        assert!(t.contains(&30));
        assert!(!t.contains(&29));
    }

    #[test]
    fn threshold_less_than_constructor() {
        let t = Threshold::less_than(30);

        assert!(t.contains(&29));
        assert!(!t.contains(&30));
        assert!(!t.contains(&31));
    }

    #[test]
    fn threshold_less_equal_constructor() {
        let t = Threshold::less_equal(30);

        assert!(t.contains(&29));
        assert!(t.contains(&30));
        assert!(!t.contains(&31));
    }

    #[test]
    fn threshold_greater_than_constructor() {
        let t = Threshold::greater_than(30);

        assert!(!t.contains(&29));
        assert!(!t.contains(&30));
        assert!(t.contains(&31));
    }

    #[test]
    fn threshold_greater_equal_constructor() {
        let t = Threshold::greater_equal(30);

        assert!(!t.contains(&29));
        assert!(t.contains(&30));
        assert!(t.contains(&31));
    }

    #[test]
    fn threshold_equal_constructor() {
        let t = Threshold::equal(30);

        assert!(!t.contains(&29));
        assert!(t.contains(&30));
        assert!(!t.contains(&31));
    }
}
