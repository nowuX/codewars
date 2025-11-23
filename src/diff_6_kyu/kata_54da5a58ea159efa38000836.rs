//! # Find the odd int
//! https://www.codewars.com/kata/54da5a58ea159efa38000836/train/rust

use std::collections::HashMap;

pub fn find_odd(arr: &[i32]) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::new();
    for &e in arr {
        let count = map.entry(e).or_insert(0);
        *count += 1;
    }
    *map.iter().find(|(_, v)| *v % 2 != 0).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::find_odd;

    #[test]
    fn basic_tests() {
        assert_eq!(
            find_odd(&vec![
                20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5
            ]),
            5
        );
        assert_eq!(find_odd(&vec![1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), -1);
        assert_eq!(find_odd(&vec![20, 1, 1, 2, 2, 3, 3, 5, 5, 4, 20, 4, 5]), 5);
        assert_eq!(find_odd(&vec![10]), 10);
        assert_eq!(find_odd(&vec![1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), 10);
        assert_eq!(find_odd(&vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), 1);
    }
}
