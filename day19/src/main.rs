mod is_possible;

use is_possible::is_possible;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input).unwrap());
}

#[derive(Debug)]
struct InvalidInputFormat;

fn part_1(input: &str) -> Result<usize, InvalidInputFormat> {
    let (towels_str, patterns_str) = input.trim().split_once("\n\n").ok_or(InvalidInputFormat)?;
    let towels: Vec<_> = towels_str.split(", ").collect();
    let patterns: Vec<_> = patterns_str.lines().collect();

    Ok(patterns.iter().filter(|p| is_possible(&towels, p)).count())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input).unwrap();

        assert_eq!(result, 6);
    }
}
