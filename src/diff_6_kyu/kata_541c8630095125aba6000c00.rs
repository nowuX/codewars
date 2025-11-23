//! # Sum of Digits / Digital Root
//! https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust

fn sum_digits(n: i64) -> i64 {
    if n < 0 {
        return sum_digits(-n);
    }

    if n < 10 {
        return n;
    }
    n % 10 + sum_digits(n / 10)
}

pub fn digital_root(mut n: i64) -> i64 {
    while n >= 10 {
        n = sum_digits(n);
    }
    n
}

#[cfg(test)]
mod tests {
    use super::digital_root;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(195), 6);
        assert_eq!(digital_root(992), 2);
        assert_eq!(digital_root(999999999999), 9);
        assert_eq!(digital_root(167346), 9);
        assert_eq!(digital_root(0), 0);
    }
}
