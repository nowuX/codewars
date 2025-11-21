//! # Consecutive strings
//!
//! You are given an array(list) `strarr` of strings and an integer `k`. Your task is to return the first longest string consisting of k consecutive strings taken in the array.
//!
//! n being the length of the string array, if n = 0 or k > n or k <= 0 return "" (return Nothing in Elm, "nothing" in Erlang).

pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let n = strarr.len();
    if n == 0 || n < k || k == 0 {
        return String::new();
    }

    let mut max = String::new();
    for win in strarr.windows(k) {
        // let mut a = String::new();
        // for i in 0..k {
        //     a.push_str(win[i]);
        // }
        let a = win.join("");
        if a.len() > max.len() {
            max = a;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_longest_consec() {
        assert_eq!(
            longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2),
            "abigailtheta"
        );
        assert_eq!(
            longest_consec(
                vec![
                    "ejjjjmmtthh",
                    "zxxuueeg",
                    "aanlljrrrxx",
                    "dqqqaaabbb",
                    "oocccffuucccjjjkkkjyyyeehh"
                ],
                1
            ),
            "oocccffuucccjjjkkkjyyyeehh"
        );
        assert_eq!(longest_consec(vec![], 3), "");
        assert_eq!(
            longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3),
            "ixoyx3452zzzzzzzzzzzz"
        );
        assert_eq!(
            longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15),
            ""
        );
        assert_eq!(
            longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0),
            ""
        );
    }
}
