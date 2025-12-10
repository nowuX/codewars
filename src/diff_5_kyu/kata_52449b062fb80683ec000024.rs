//! The Hashtag Generator
//! https://www.codewars.com/kata/52449b062fb80683ec000024/train/rust

pub fn generate_hashtag(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        return None;
    }

    let mut out = String::from("#");

    for word in s.split_whitespace() {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            out.push(first.to_ascii_uppercase());
            out.push_str(&chars.as_str().to_ascii_lowercase());
        }
    }

    if out.len() > 140 { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::generate_hashtag;

    fn dotest(s: &str, expected: Option<String>) {
        let actual = generate_hashtag(s);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right) when testing with s = {s:?}"
        );
    }

    #[test]
    fn test() {
        dotest("", None);
        dotest("Codewars", Some("#Codewars".to_owned()));
        dotest("Codewars      ", Some("#Codewars".to_owned()));
        dotest("Codewars Is Nice", Some("#CodewarsIsNice".to_owned()));
        dotest("codewars is nice", Some("#CodewarsIsNice".to_owned()));
        dotest("CodeWars is nice", Some("#CodewarsIsNice".to_owned()));
        dotest("c i n", Some("#CIN".to_owned()));
        dotest("codewars  is  nice", Some("#CodewarsIsNice".to_owned()));
        dotest(
            "Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong Cat",
            None,
        );
    }
}
