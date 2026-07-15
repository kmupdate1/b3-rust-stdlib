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

use b3_core::error::Result;

pub trait AdditiveInverse {
    fn inverse(&self) -> Self;
    // fn invert(&mut self);
}

pub trait MultiplicativeInverse {
    type Output;
    type Error;
    fn try_inverse(&self) -> Result<Self::Output, Self::Error>;
    fn try_invert(&mut self) -> Result<(), Self::Error>;
}
