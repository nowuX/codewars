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
    let mut number = number.abs();
    if number == 0 {
        return 0;
    }

    let mut acc = 0;
    while number > 0 {
        acc += number % 10;
        number /= 10;
    }
    acc
}

#[cfg(test)]
mod sample_tests {
    use super::sum_digits;

    #[test]
    fn test_sum_0() {
        assert_eq!(sum_digits(0), 0);
    }

    #[test]
    fn test_sum_digits_10() {
        assert_eq!(sum_digits(10), 1);
    }

    #[test]
    fn test_sum_digits_99() {
        assert_eq!(sum_digits(99), 18);
    }

    #[test]
    fn test_sum_digits_neg_32() {
        assert_eq!(sum_digits(-32), 5);
    }
}
