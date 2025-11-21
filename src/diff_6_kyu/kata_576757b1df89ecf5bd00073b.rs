//! # Build Tower
//! https://www.codewars.com/kata/576757b1df89ecf5bd00073b

pub fn tower_builder(n_floors: usize) -> Vec<String> {
    let mut towers = Vec::with_capacity(n_floors);

    for i in 1..=n_floors {
        let tower = format!(
            "{}{}{}",
            " ".repeat(n_floors - i),
            "*".repeat((2 * i) - 1),
            " ".repeat(n_floors - i),
        );
        towers.push(tower);
    }

    towers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(tower_builder(1), vec!["*"]);
        assert_eq!(tower_builder(2), vec![" * ", "***"]);
        assert_eq!(tower_builder(3), vec!["  *  ", " *** ", "*****"]);
    }
}
