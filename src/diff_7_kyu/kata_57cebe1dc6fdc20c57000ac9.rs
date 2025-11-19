//! # Shortest Word
//!
//! Simple, given a string of words, return the length of the shortest word(s).
//! String will never be empty and you do not need to account for different data types.

pub fn find_short(s: &str) -> u32 {
    let mut words = s.split_whitespace();
    let mut min_lenght = words.next().unwrap().len();
    for word in words {
        if word.len() < min_lenght {
            min_lenght = word.len();
        }
    }
    min_lenght as u32
}

pub fn find_short_v2(s: &str) -> u32 {
    s.split_whitespace().map(str::len).min().unwrap_or(0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(
            find_short("bitcoin take over the world maybe who knows perhaps"),
            3
        );
        assert_eq!(
            find_short("turns out random test cases are easier than writing out basic ones"),
            3,
        );
        assert_eq!(
            find_short("lets talk about javascript the best language"),
            3
        );
        assert_eq!(
            find_short("i want to travel the world writing code one day"),
            1
        );
        assert_eq!(find_short("Lets all go on holiday somewhere very cold"), 2);
        assert_eq!(find_short("Let's travel abroad shall we"), 2);
    }
}
