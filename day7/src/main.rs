mod part_1;
mod part_2;

use part_1::is_solveable as part_1_is_solveable;
use part_2::is_solveable as part_2_is_solveable;
use std::{fs::read_to_string, num::ParseIntError, str::FromStr};
use thiserror::Error;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input).unwrap());
    println!("part_2: {}", part_2(&input).unwrap());
}

fn part_1(input: &str) -> Result<usize, InputLineParseErr> {
    Ok(input
        .trim()
        .lines()
        .map(|l| l.parse::<InputLine>())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter(|line| part_1_is_solveable(line.target, &line.values))
        .map(|line| line.target)
        .sum())
}

fn part_2(input: &str) -> Result<usize, InputLineParseErr> {
    Ok(input
        .trim()
        .lines()
        .map(|l| l.parse::<InputLine>())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter(|line| part_2_is_solveable(line.target, &line.values))
        .map(|line| line.target)
        .sum())
}

struct InputLine {
    target: usize,
    values: Vec<usize>,
}

#[derive(Error, Debug)]
enum InputLineParseErr {
    #[error("Couldn't find ': ' in input string")]
    NoColonSpace,

    #[error("Tried to parse an invalid number")]
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for InputLine {
    type Err = InputLineParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target_str, values_str) = s
            .trim()
            .split_once(": ")
            .ok_or(InputLineParseErr::NoColonSpace)?;

        let target: usize = target_str.parse()?;

        let values = values_str
            .split(" ")
            .map(|v| v.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(InputLine { target, values })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input).unwrap();

        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_2(&input).unwrap();

        assert_eq!(result, 11387);
    }
}
