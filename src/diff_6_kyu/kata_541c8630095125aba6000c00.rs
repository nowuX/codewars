//! # Sum of Digits / Digital Root
//! https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust

fn sum_digits(n: i64) -> i64 {
    let mut number = n.abs();
    if number == 0 {
        return 0;
    }

    let mut acc = 0;
    while number > 0 {
        acc += number % 10;
        number /= 10;
    }
    acc
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
