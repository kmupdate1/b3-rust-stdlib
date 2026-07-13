use super::Bound;
use crate::relation::Ordering;

pub struct Threshold<T> {
    value: T,
    order: Ordering,
    bound: Bound,
}
