//! # Flatten and sort an array
//!
//! Given a two-dimensional array of integers, return the flattened version of the array with all the integers in the sorted (ascending) order.
//!
//! Example:
//!
//! Given [[3, 2, 1], [4, 6, 5], [], [9, 7, 8]], your function should return [1, 2, 3, 4, 5, 6, 7, 8, 9]

pub fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
    let mut result = arr.concat();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(flatten_and_sort(&[]), &[]);
        assert_eq!(flatten_and_sort(&[vec![], vec![]]), &[]);
        assert_eq!(flatten_and_sort(&[vec![], vec![1]]), &[1]);
        assert_eq!(
            flatten_and_sort(&[vec![3, 2, 1], vec![7, 9, 8], vec![6, 4, 5]]),
            &[1, 2, 3, 4, 5, 6, 7, 8, 9],
        );
        assert_eq!(
            flatten_and_sort(&[vec![1, 3, 5], vec![100], vec![2, 4, 6]]),
            &[1, 2, 3, 4, 5, 6, 100],
        );
    }
}
