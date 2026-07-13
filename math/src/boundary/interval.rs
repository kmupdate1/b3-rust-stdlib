use super::Threshold;

pub struct Interval<T> {
    lower: Threshold<T>,
    upper: Threshold<T>,
}
