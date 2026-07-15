use crate::Ratio;

/**
 * 全体に対する割合を表す。
 *
 * Ratio のうち 0 <= ratio <= 1 を満たすもの。
 *
 * 現時点では使用シーンを検討中。
 * 共通概念として必要性が明確に慣れば実装する。
 */

pub struct Proportion<T> {
    ratio: Ratio<T>,
}
