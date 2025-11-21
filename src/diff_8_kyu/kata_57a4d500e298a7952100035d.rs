//! # Hex to Decimal
//!
//! Complete the function which converts hex number (given as a string) to a decimal number.

pub fn hex_to_dec(hex_string: &str) -> u32 {
    u32::from_str_radix(hex_string, 16).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(hex_to_dec("1"), 1);
        assert_eq!(hex_to_dec("a"), 10);
        assert_eq!(hex_to_dec("10"), 16);
    }
}
