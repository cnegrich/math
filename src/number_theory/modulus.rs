///! Modulus Trait
// Rust can perform the remainder operation, but not a modulus congruence operation
use crate::number_theory::numbers::Integers;

/// Implement the modulo operation
///
/// The modulus operator (%) is the remainder operation but not a congruency modulus
/// See: https://en.wikipedia.org/wiki/Modular_arithmetic for more information
///
/// Two functions are implemented, one to find a possible congruency given the modulus, another to check if a and b are congruent given the modulus
pub trait Modulus<T> {
    fn modulus(self, b: T) -> T;
    // fn is_congruent_modulus(self, b: T) -> bool;
}

impl<T> Modulus<T> for T
where
    T: Integers,
{
    fn modulus(self, b: T) -> T {
        (self % b) + b
    }

    // fn is_congruent_modulus(self, b: T) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 1)]
    #[case(38, 12, 14)]
    #[case(2, 5, 7)]
    fn usize_modulus_test(#[case] a: usize, #[case] b: usize, #[case] expected: usize) {
        assert_eq!(expected, a.modulus(b))
    }

    #[rstest]
    #[case(1, 1, 1)]
    #[case(38, 12, 14)]
    #[case(2, 5, 7)]
    fn isize_modulus_test(#[case] a: isize, #[case] b: isize, #[case] expected: isize) {
        assert_eq!(expected, a.modulus(b))
    }
}
