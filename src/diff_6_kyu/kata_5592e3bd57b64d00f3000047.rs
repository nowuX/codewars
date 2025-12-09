//! # Build a pile of Cubes
//!
//! Your task is to construct a building which will be a pile of n cubes. The cube at the bottom will have a volume of n3 n^3 n3, the cube above will have volume of (n−1)3 (n-1)^3 (n−1)3 and so on until the top which will have a volume of 13 1^3 13.
//!
//! You are given the total volume m of the building. Being given m can you find the number n of cubes you will have to build?
//!
//! The parameter of the function findNb `(find_nb, find-nb, findNb, ...)` will be an integer m and you have to return the integer n such as n^3 + (n−1)^3 + (n−2)^3 + ... + 1^3 = m  if such a n exists or -1 if there is no such n.
//!
//! Examples:
//! - findNb(1071225) --> 45
//! - findNb(91716553919377) --> -1

pub fn find_nb(m: u64) -> i32 {
    let mut vol_acc = 0;
    for n in 1_u64.. {
        vol_acc += n.pow(3);
        if vol_acc >= m {
            return if vol_acc == m { n as i32 } else { -1 };
        }
    }
    -1
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn basics_find_nb() {
        let cases = [
            (4, -1),
            (16, -1),
            (4183059834009, 2022),
            (24723578342962, -1),
            (135440716410000, 4824),
            (40539911473216, 3568),
            (26825883955641, 3218),
        ];
        for (n, expected) in cases {
            assert_eq!(find_nb(n), expected);
        }
    }
}
