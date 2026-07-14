/**
 * 分子・分母による有理数の表現。
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction<T> {
    numerator: T,
    denominator: T,
}
