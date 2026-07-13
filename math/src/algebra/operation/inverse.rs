/**
 * Inverse:
 *
 * Algebraic inverse.
 *
 * This trait represents an inverse operation in an algebraic structure.
 *
 * Current implementations provide the additive inverse.
 * Multiplicative inverses may be introduced in the future as a separate trait.
 *
 * Additive inverse:
 * a + (-a) = 0
 *
 * Multiplicative inverse:
 * a * (1/a) = 1
 */

pub trait AdditiveInverse {
    fn inverse(&self) -> Self;
}

pub trait MultiplicativeInverse {
    type Output;
    fn inverse(&self) -> Self::Output;
}
