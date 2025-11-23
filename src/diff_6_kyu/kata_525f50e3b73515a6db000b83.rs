//! # Create Phone Number
//! https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/rust

use std::ops::Range;

fn slice_to_str(numbers: &[u8], range: Range<usize>) -> String {
    let mut buffer = String::new();
    for e in &numbers[range] {
        buffer.push((e + b'0') as char);
    }
    buffer
}

pub fn create_phone_number(numbers: &[u8]) -> String {
    let mut buffer = String::new();
    buffer.push('(');
    buffer.push_str(&slice_to_str(numbers, 0..3));
    buffer.push(')');
    buffer.push(' ');
    buffer.push_str(&slice_to_str(numbers, 3..6));
    buffer.push('-');
    buffer.push_str(&slice_to_str(numbers, 6..10));
    buffer
}

#[cfg(test)]
mod tests {
    use super::create_phone_number;

    #[test]
    fn returns_expected() {
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "(123) 456-7890"
        );
        assert_eq!(
            create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            "(111) 111-1111"
        );
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
            "(123) 456-7899"
        );
    }
}
