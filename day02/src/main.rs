use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parse_line)
        .map(to_directions)
        .filter(is_conservatively_safe)
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parse_line)
        .filter(is_recklessly_safe)
        .count()
}

fn is_conservatively_safe(directions: &Vec<Direction>) -> bool {
    let set: HashSet<&Direction> = directions.iter().collect();

    return set.len() == 1 && !set.contains(&Direction::Unsafe);
}

fn is_conservatively_safe_p(directions: Vec<Direction>) -> bool {
    is_conservatively_safe(&directions)
}

fn is_recklessly_safe(line: &Vec<isize>) -> bool {
    let directions = to_directions(line.to_vec());

    if is_conservatively_safe(&directions) {
        return true;
    }

    let combinations = to_leave_one_out_combinations(line);

    combinations
        .into_iter()
        .map(to_directions)
        .any(is_conservatively_safe_p)
}

fn to_leave_one_out_combinations(line: &Vec<isize>) -> Vec<Vec<isize>> {
    line.iter()
        .enumerate()
        .map(|(i, _)| {
            let mut combination = line.clone();
            combination.remove(i);
            return combination;
        })
        .collect()
}

fn to_directions(line: Vec<isize>) -> Vec<Direction> {
    line.windows(2)
        .map(|window| {
            let a = window.iter().nth(0).unwrap();
            let b = window.iter().nth(1).unwrap();

            match a - b {
                d if -3 <= d && d < 0 => Direction::SafeDown,
                d if 0 < d && d <= 3 => Direction::SafeUp,
                _ => Direction::Unsafe,
            }
        })
        .collect()
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
    fn test_part_1() {
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
    fn test_part_2() {
        let input = "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "
        .to_string();

        assert_eq!(part_2(&input), 4);
    }
}
