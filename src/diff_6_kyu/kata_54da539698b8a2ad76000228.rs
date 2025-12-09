//! # Take a Ten Minutes Walk
//!
//! You live in the city of Cartesia where all roads are laid out in a perfect grid. You arrived ten minutes too early to an appointment, so you decided to take the opportunity to go for a short walk. The city provides its citizens with a Walk Generating App on their phones -- everytime you press the button it sends you an array of one-letter strings representing directions to walk (eg. ['n', 's', 'w', 'e']). You always walk only a single block for each letter (direction) and you know it takes you one minute to traverse one city block, so create a function that will return true if the walk the app gives you will take you exactly ten minutes (you don't want to be early or late!) and will, of course, return you to your starting point. Return false otherwise.
//!
//! > Note: you will always receive a valid array containing a random assortment of direction letters ('n', 's', 'e', or 'w' only). It will never give you an empty array (that's not a walk, that's standing still!).

pub fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }

    let mut x = 0;
    let mut y = 0;
    for step in walk {
        match step {
            'n' => y += 1,
            's' => y -= 1,
            'w' => x -= 1,
            'e' => x += 1,
            _ => unreachable!(),
        }
    }
    x == 0 && y == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_cases() {
        assert_eq!(
            is_valid_walk(&['n', 's', 's', 'e', 'w', 'n', 's', 's', 'e', 'w']),
            false
        );
        assert_eq!(
            is_valid_walk(&['n', 'n', 'w', 'w', 's', 's', 'e', 'e', 'n', 's']),
            true
        );
        assert_eq!(
            is_valid_walk(&['n', 'n', 'w', 'w', 's', 's', 'w', 'w', 'n', 's']),
            false
        );
    }

    #[test]
    fn sample_tests() {
        assert!(is_valid_walk(&[
            'n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e'
        ]));
        assert!(!is_valid_walk(&['w']));
        assert!(!is_valid_walk(&[
            'n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's'
        ]))
    }
}
