use super::Threshold;

pub struct Interval<T> {
    lower: Threshold<T>,
    upper: Threshold<T>,
}

impl<T> Interval<T> {
    pub fn new(lower: Threshold<T>, upper: Threshold<T>) -> Self {
        Self { lower, upper }
    }

    pub fn lower(&self) -> &Threshold<T> { &self.lower }
    pub fn upper(&self) -> &Threshold<T> { &self.upper }
}

impl<T> Interval<T>
where
    T: PartialOrd + PartialEq,
{
    pub fn contains(&self, value: &T) -> bool {
        self.lower.contains(value) && self.upper.contains(value)
    }
}

#[test]
fn interval_contains_inclusive() {
    let interval = Interval::new(
        Threshold::greater_equal(40),
        Threshold::less_equal(60),
    );

    assert!(!interval.contains(&39));
    assert!(interval.contains(&40));
    assert!(interval.contains(&50));
    assert!(interval.contains(&60));
    assert!(!interval.contains(&61));
}
