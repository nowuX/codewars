//! # Bit Counting
//! https://www.codewars.com/kata/526571aae218b8ee490006f4

/// ⚠️ Most difficult problem in history deah
pub fn count_bits(n: i64) -> u32 {
    n.count_ones()
}

#[cfg(test)]
mod tests {
    use super::count_bits;

    #[test]
    fn sample_tests() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(4), 1);
        assert_eq!(count_bits(7), 3);
        assert_eq!(count_bits(77231418), 14);
        assert_eq!(count_bits(12525589), 11);
        assert_eq!(count_bits(31), 5);
        assert_eq!(count_bits(417862), 7);
        assert_eq!(count_bits(89), 4);
        assert_eq!(count_bits(674259), 10);
        assert_eq!(count_bits(i64::MAX), 63);
    }
}
