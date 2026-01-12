mod part_1_state;
mod part_2_state;

use std::fs::read_to_string;
use part_1_state::{Part1State, Part1StateParseError};
use part_2_state::{Part2State, Part2StateParseError};
use thiserror::Error;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input).unwrap());
    println!("part_2: {}", part_2(&input).unwrap());
}

#[derive(Debug, Error)]
enum Part1Error {
    #[error("Failed to parse state")]
    ParseError(#[from] Part1StateParseError),

    #[error("Iterator malfunctioned")]
    IteratorMalfunction,
}

fn part_1(input: &str) -> Result<usize, Part1Error> {
    let state: Part1State = input.parse()?;
    let final_state = state.iter().last().ok_or(Part1Error::IteratorMalfunction)?;
    Ok(final_state.score())
}

#[derive(Debug, Error)]
enum Part2Error {
    #[error("Failed to parse state")]
    ParseError(#[from] Part2StateParseError),

    #[error("Iterator malfunctioned")]
    IteratorMalfunction,
}

fn part_2(input: &str) -> Result<usize, Part2Error> {
    let state: Part2State = input.parse()?;
    let final_state = state.iter().last().ok_or(Part2Error::IteratorMalfunction)?;
    Ok(final_state.score())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input).unwrap();

        assert_eq!(result, 10092);
    }

    #[test]
    fn test_part_1_example_small() {
        let input = read_to_string("./input_example_small.txt").unwrap();
        let result = part_1(&input).unwrap();

        assert_eq!(result, 2028);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_2(&input).unwrap();

        assert_eq!(result, 9021);
    }
}
