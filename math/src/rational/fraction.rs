#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction<T> {
    numerator: T,
    denominator: T,
}
