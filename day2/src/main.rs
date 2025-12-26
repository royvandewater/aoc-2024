use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    input.trim().lines().map(parse_line).filter(is_safe).count()
}

fn is_safe(line: &Vec<isize>) -> bool {
    let directions = to_directions(line);

    return directions.len() == 1 && !directions.contains(&Direction::Unsafe);
}

fn to_directions(line: &Vec<isize>) -> HashSet<Direction> {
    let mut set: HashSet<Direction> = HashSet::new();

    for window in line.windows(2) {
        let a = window.iter().nth(0).unwrap();
        let b = window.iter().nth(1).unwrap();
        let diff = a - b;

        if -3 <= diff && diff < 0 {
            set.insert(Direction::SafeDown);
            continue;
        }

        if 0 < diff && diff <= 3 {
            set.insert(Direction::SafeUp);
            continue;
        }

        set.insert(Direction::Unsafe);
    }

    return set;
}

#[derive(Hash, Eq, PartialEq)]
enum Direction {
    SafeUp,
    SafeDown,
    Unsafe,
}

fn parse_line(line: &str) -> Vec<isize> {
    line.trim()
        .split(' ')
        .into_iter()
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "
        .to_string();

        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_is_safe_line_1() {
        assert!(is_safe(&vec![7, 6, 4, 2, 1]))
    }
}
