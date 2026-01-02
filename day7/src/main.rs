mod is_solveable;

use is_solveable::is_solveable;
use std::{fs::read_to_string, num::ParseIntError, str::FromStr};
use thiserror::Error;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input).unwrap());
}

#[derive(Error, Debug)]
enum Part1Err {
    #[error("Failed to parse input")]
    InputLineParseErr(#[from] InputLineParseErr),
}

fn part_1(input: &str) -> Result<usize, Part1Err> {
    let result = input
        .trim()
        .lines()
        .map(|l| l.parse::<InputLine>())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .filter(|line| is_solveable(line.target, &line.values))
        .map(|line| line.target)
        .sum();

    Ok(result)
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
}
