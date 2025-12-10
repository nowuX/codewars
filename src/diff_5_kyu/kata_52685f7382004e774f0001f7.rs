//! # Human Readable Time
//! https://www.codewars.com/kata/52685f7382004e774f0001f7

pub fn make_readable(mut seconds: u32) -> String {
    assert!(seconds < 360000, "Out of range");
    let hours = seconds / (60 * 60);
    seconds %= 60 * 60;
    format!("{:02}:{:02}:{:02}", hours, seconds / 60, seconds % 60)
}

#[cfg(test)]
mod tests {
    use super::make_readable;

    #[test]
    fn test() {
        assert_eq!(make_readable(0), "00:00:00");
        assert_eq!(make_readable(59), "00:00:59");
        assert_eq!(make_readable(60), "00:01:00");
        assert_eq!(make_readable(3599), "00:59:59");
        assert_eq!(make_readable(3600), "01:00:00");
        assert_eq!(make_readable(86399), "23:59:59");
        assert_eq!(make_readable(86400), "24:00:00");
        assert_eq!(make_readable(359999), "99:59:59");
    }
}
