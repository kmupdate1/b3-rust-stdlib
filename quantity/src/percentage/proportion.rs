use b3_math::ratio::Ratio;

/**
 * 全体に対する割合を表す。
 *
 * Ratio のうち 0 <= ratio <= 1 を満たすもの。
 */

pub struct Proportion<T> {
    ratio: Ratio<T>,
}
