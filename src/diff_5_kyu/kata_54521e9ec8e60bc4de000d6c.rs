//! # Maximum subarray sum
//! https://www.codewars.com/kata/54521e9ec8e60bc4de000d6c

pub fn max_sequence(seq: &[i32]) -> i32 {
    let mut sum = 0;
    let mut max_sum = 0;

    for &x in seq {
        sum = (sum + x).max(0);
        max_sum = max_sum.max(sum);
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::max_sequence;

    #[test]
    fn sample_tests() {
        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
    }
}
