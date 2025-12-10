//! # Moving zeros to the end
//! https://www.codewars.com/kata/52597aa56021e91c93000cb0

pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut out = vec![0_u8; arr.len()];
    let mut idx = 0;
    for &e in arr {
        if e != 0_u8 {
            if let Some(value) = out.get_mut(idx) {
                *value = e;
            }
            idx += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::move_zeros;

    #[test]
    fn test() {
        assert_eq!(
            move_zeros(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]),
            &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]
        );
        assert_eq!(
            move_zeros(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9]),
            &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(move_zeros(&[0, 0]), &[0, 0]);
        assert_eq!(move_zeros(&[0]), &[0]);
        assert_eq!(move_zeros(&[]), &[]);
    }
}
