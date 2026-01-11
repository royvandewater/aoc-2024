mod state;

use std::fs::read_to_string;
use state::State;
use thiserror::Error;

use crate::state::StateParseError;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input).unwrap());
}

#[derive(Debug, Error)]
enum Part1Error {
    #[error("Failed to parse state")]
    ParseError(#[from] StateParseError),

    #[error("Iterator malfunctioned")]
    IteratorMalfunction,
}

fn part_1(input: &str) -> Result<usize, Part1Error> {
    let state: State = input.parse()?;
    let final_state = state.iter().last().ok_or(Part1Error::IteratorMalfunction)?;
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
}
