//! # Multiples of 3 or 5
//! https://www.codewars.com/kata/514b92a657cdc65150000006

pub fn solution(num: i32) -> i32 {
    (1..num).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(23, solution(10));
        assert_eq!(33, solution(11));
        assert_eq!(225, solution(33));
        assert_eq!(8, solution(6));
        assert_eq!(3420, solution(123));
        assert_eq!(543, solution(50));
        assert_eq!(0, solution(0));
        assert_eq!(0, solution(-203));
        assert_eq!(25719750, solution(10500));
    }
}
