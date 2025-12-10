//! # Directions Reduction
//! https://www.codewars.com/kata/550f22f4d758534c1100025a

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    West,
    South,
}

fn cancels(a: Direction, b: Direction) -> bool {
    matches!(
        (a, b),
        (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::East, Direction::West)
            | (Direction::West, Direction::East)
    )
}

pub fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut path = Vec::new();
    for &dir in arr {
        if let Some(&last) = path.last()
            && cancels(last, dir)
        {
            path.pop();
            continue;
        }
        path.push(dir);
    }
    path
}

#[cfg(test)]
mod tests {
    use super::{Direction::*, dir_reduc};

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
}
