mod computer;

use std::fs::read_to_string;
use thiserror::Error;

use crate::computer::{Computer, ComputerParseError, RuntimeError};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input).unwrap());
}

#[derive(Debug, Error)]
enum ComputerError {
    #[error("Failed to parse computer")]
    ComputerParseError(#[from] ComputerParseError),

    #[error("Runtime Error")]
    RuntimeError(#[from] RuntimeError),
}

fn part_1(input: &str) -> Result<String, ComputerError> {
    let mut computer: Computer = input.parse()?;
    computer.run()?;
    Ok(computer.output_as_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input).unwrap();

        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }
}
