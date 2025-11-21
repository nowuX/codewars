//! # Love vs friendship
//!
//! If　`a = 1, b = 2, c = 3 ... z = 26`
//!
//! Then `l + o + v + e = 54`
//!
//! and `f + r + i + e + n + d + s + h + i + p = 108`
//!
//! So `friendship` is twice as strong as `love` :-)
//!
//! Your task is to write a function which calculates the value of a word based off the sum of the alphabet positions of its characters.
//!
//! The input will always be made of only lowercase letters and will never be empty.

pub fn words_to_marks(s: &str) -> u32 {
    s.chars().map(|c| c as u32 - 'a' as u32 + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(words_to_marks("attitude"), 100);
        assert_eq!(words_to_marks("friends"), 75);
        assert_eq!(words_to_marks("family"), 66);
        assert_eq!(words_to_marks("selfness"), 99);
        assert_eq!(words_to_marks("knowledge"), 96);
    }
}
