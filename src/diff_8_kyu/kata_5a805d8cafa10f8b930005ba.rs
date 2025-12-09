//! # Find Nearest square number
//!
//! Your task is to find the nearest square number of a positive integer n. In mathematics, a square number or perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself.
//!
//! For example, if `n = 111`, then the nearest square number equals 121, since 111 is closer to 121, the square of 11, than 100, the square of 10.
//!
//! If `n` is already a perfect square (e.g. `n = 144`, `n = 81`, etc.), you need to just return n.

pub fn nearest_sq(n: u32) -> u32 {
    let sqrt = (n as f64).sqrt();
    let lower = sqrt.floor().powi(2) as u32;
    let upper = sqrt.ceil().powi(2) as u32;

    if n - lower <= upper - n { lower } else { upper }
}

pub fn nearest_sq_v2(n: u32) -> u32 {
    ((n as f64).sqrt().round() as u32).pow(2)
}

#[cfg(test)]
mod tests {
    use super::{nearest_sq, nearest_sq_v2};

    #[test]
    fn sample_tests() {
        assert_eq!(1, nearest_sq(1));
        assert_eq!(1, nearest_sq(2));
        assert_eq!(9, nearest_sq(10));
        assert_eq!(121, nearest_sq(111));
        assert_eq!(10000, nearest_sq(9999));
    }

    #[test]
    fn sample_tests_2() {
        assert_eq!(1, nearest_sq_v2(1));
        assert_eq!(1, nearest_sq_v2(2));
        assert_eq!(9, nearest_sq_v2(10));
        assert_eq!(121, nearest_sq_v2(111));
        assert_eq!(10000, nearest_sq_v2(9999));
    }
}
