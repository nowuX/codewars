//! # Summing a number's digits
//!
//! Write a function which takes a number as input and returns the sum of the absolute value of each of the number's decimal digits.
//!
//! For example: (**Input** --> **Output**)
//!
//! 10 --> 1
//! 9 --> 18
//! -32 --> 5
//!
//! Let's assume that all numbers in the input will be integer values.

pub fn sum_digits(number: i32) -> i32 {
    if number < 0 {
        return sum_digits(-number);
    }

    if number < 10 {
        return number;
    }

    number % 10 + sum_digits(number / 10)
}

#[cfg(test)]
mod sample_tests {
    use super::sum_digits;

    #[test]
    fn test_sum_digits_10() {
        let n = 10;
        let expected = 1;
        let actual = sum_digits(n);
        assert_eq!(
            actual, expected,
            "\nsum_digits({}) should be {}, got {}",
            n, expected, actual
        );
    }

    #[test]
    fn test_sum_digits_99() {
        let n = 99;
        let expected = 18;
        let actual = sum_digits(n);
        assert_eq!(
            actual, expected,
            "\nsum_digits({}) should be {}, got {}",
            n, expected, actual
        );
    }

    #[test]
    fn test_sum_digits_neg_32() {
        let n = -32;
        let expected = 5;
        let actual = sum_digits(n);
        assert_eq!(
            actual, expected,
            "\nsum_digits({}) should be {}, got {}",
            n, expected, actual
        );
    }
}
