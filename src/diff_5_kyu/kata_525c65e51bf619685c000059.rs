//! # Pete, the baker
//! https://www.codewars.com/kata/525c65e51bf619685c000059/train/rust

use std::collections::HashMap;

pub fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
    let mut min_cakes = u32::MAX;
    for (ingredient, needed) in recipe {
        match available.get(ingredient) {
            Some(have) => min_cakes = min_cakes.min(have / needed),
            None => return 0,
        };
    }

    min_cakes
}

#[cfg(test)]
mod tests {
    use super::cakes;
    use std::collections::HashMap;

    macro_rules! map {
        () => { HashMap::new() };
        ($($ingredient:ident : $amount:expr),+) => {{
            let mut map = HashMap::new();
            $( map.insert(stringify!($ingredient), $amount); )*
            map
        }};
    }

    fn test(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>, expected: u32) {
        let actual = cakes(recipe, available);
        assert!(
            actual == expected,
            "Expected to bake {expected} cakes, but got {actual} cakes instead.\nAvailable ingredients:\n  {available:?}\nRecipe:\n  {recipe:?}\n\n"
        );
    }

    #[test]
    fn test_example() {
        let recipe = map!(flour: 500, sugar: 200, eggs: 1);
        let available = map!(flour: 1200, sugar: 1200, eggs: 5, milk: 200);
        test(&recipe, &available, 2);

        let recipe = map!(apples: 3, flour: 300, sugar: 150, milk: 100, oil: 100);
        let available = map!(sugar: 500, flour: 2000, milk: 2000);
        test(&recipe, &available, 0);
    }
}
