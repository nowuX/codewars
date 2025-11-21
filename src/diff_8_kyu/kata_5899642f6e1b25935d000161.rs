//! # Merge two sorted arrays into one
//! https://www.codewars.com/kata/5899642f6e1b25935d000161/train/rust

pub fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut vec = vec![];
    for e in arr1 {
        if !vec.contains(e) {
            vec.push(*e);
        }
    }
    for e in arr2 {
        if !vec.contains(e) {
            vec.push(*e);
        }
    }
    vec.sort();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(
            merge_arrays(&[1, 2, 3, 4], &[5, 6, 7, 8]),
            &[1, 2, 3, 4, 5, 6, 7, 8]
        );
        assert_eq!(
            merge_arrays(&[1, 3, 5, 7, 9], &[10, 8, 6, 4, 2]),
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
        assert_eq!(
            merge_arrays(&[1, 3, 5, 7, 9, 11, 12], &[1, 2, 3, 4, 5, 10, 12]),
            &[1, 2, 3, 4, 5, 7, 9, 10, 11, 12]
        );
    }
}
