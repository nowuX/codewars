//! # Break camelCase
//! https://www.codewars.com/kata/5208f99aee097e6552000148/train/rust

pub fn solution(s: &str) -> String {
    let mut buffer = String::new();
    for c in s.chars() {
        if c.is_uppercase() {
            buffer.push(' ');
        }
        buffer.push(c);
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
