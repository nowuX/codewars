//! # Crack the PIN
//! https://www.codewars.com/kata/5efae11e2d12df00331f91a6

use rayon::prelude::*;

pub fn crack(hash: &str) -> String {
    for i in 0..=99999 {
        let pin = format!("{:05}", i);
        let digest = md5::compute(pin.as_bytes());
        let hex = format!("{:x}", digest);
        if hex == hash {
            return pin;
        }
    }
    "".to_string()
}

pub fn crack_paralel(hash: &str) -> String {
    format!(
        "{:05}",
        (0..=99999)
            .into_par_iter()
            .find_any(|x| format!("{:x}", md5::compute(format!("{:05}", x))) == hash)
            .unwrap()
    )
}

#[cfg(test)]
mod tests {
    use crate::diff_6_kyu::kata_5efae11e2d12df00331f91a6::crack_paralel;

    use super::crack;

    #[test]
    fn sample_tests() {
        for f in [crack, crack_paralel] {
            assert_eq!(f("827ccb0eea8a706c4c34a16891f84e7b"), "12345");
            assert_eq!(f("86aa400b65433b608a9db30070ec60cd"), "00078");
        }
    }
}
