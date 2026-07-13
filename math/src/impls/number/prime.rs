use crate::number::property::divisibility::Divisibility;
use crate::number::PrimeCheck;

macro_rules! impl_prime_check {
    ($($t:ty),* $(,)?) => {
        $(
            impl PrimeCheck for $t {
                fn is_prime(&self) -> bool {
                    if *self <= 1 { return false; }

                    let mut i: $t = 2;

                    while i <= *self / i {
                        if self.divisible_by(&i) { return false; }
                        i += 1;
                    }

                    true
                }
            }
        )*
    }
}

impl_prime_check!(u8, u16, u32, u64, u128, usize);
impl_prime_check!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::PrimeCheck;

    #[test]
    fn prime_check_basic() {
        assert!(!0i32.is_prime());
        assert!(!1i32.is_prime());

        assert!(2i32.is_prime());
        assert!(3i32.is_prime());
        assert!(!4i32.is_prime());
        assert!(5i32.is_prime());
    }

    #[test]
    fn prime_check_composite_numbers() {
        assert!(!6i32.is_prime());
        assert!(!8i32.is_prime());
        assert!(!9i32.is_prime());
        assert!(!10i32.is_prime());
        assert!(!12i32.is_prime());
        assert!(!15i32.is_prime());
        assert!(!25i32.is_prime());
        assert!(!100i32.is_prime());
    }

    #[test]
    fn prime_check_large_primes() {
        assert!(97i32.is_prime());
        assert!(101i32.is_prime());
        assert!(997i32.is_prime());
    }

    #[test]
    fn prime_check_large_composites() {
        assert!(!99i32.is_prime());
        assert!(!121i32.is_prime()); // 11 * 11
        assert!(!1001i32.is_prime()); // 7 * 11 * 13
    }

    #[test]
    fn prime_check_negative_numbers() {
        assert!(!(-1i32).is_prime());
        assert!(!(-2i32).is_prime());
        assert!(!(-3i32).is_prime());
        assert!(!(-97i32).is_prime());
    }

    #[test]
    fn prime_check_unsigned() {
        assert!(!0u32.is_prime());
        assert!(!1u32.is_prime());

        assert!(2u32.is_prime());
        assert!(3u32.is_prime());
        assert!(97u32.is_prime());

        assert!(!100u32.is_prime());
    }

    #[test]
    fn prime_check_small_integer_types() {
        assert!(!0i8.is_prime());
        assert!(!1i8.is_prime());
        assert!(2i8.is_prime());
        assert!(3i8.is_prime());
        assert!(!4i8.is_prime());

        assert!(!0u8.is_prime());
        assert!(!1u8.is_prime());
        assert!(2u8.is_prime());
        assert!(5u8.is_prime());
        assert!(!6u8.is_prime());
    }

    #[test]
    fn prime_check_edge_values() {
        assert!(!(-128i8).is_prime());
        assert!(127i8.is_prime()); // 127 is prime
        assert!(127i32.is_prime());
    }
}
