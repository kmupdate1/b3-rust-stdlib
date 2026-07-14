use crate::ratio::Ratio;

/**
 * 全体に対する割合を表す。
 *
 * Ratio のうち 0 <= value <= 1 を満たすもの。
 */

pub struct Proportion<T> {
    value: Ratio<T>,
}
