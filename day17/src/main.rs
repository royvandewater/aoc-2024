mod computer;
mod custom_program;
mod custom_program_a;

use std::fs::read_to_string;
use thiserror::Error;

use crate::computer::{Computer, ComputerParseError, RuntimeError, find_target_input};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input).unwrap());
    println!("part_2: {}", part_2(&input).unwrap());
}

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
enum ComputerError {
    #[error("Failed to parse computer")]
    ComputerParseError(#[from] ComputerParseError),

    #[error("Runtime Error")]
    RuntimeError(#[from] RuntimeError),

    #[error("No Input Found for output")]
    NoInputFound,
}

fn part_1(input: &str) -> Result<String, ComputerError> {
    let mut computer: Computer = input.parse()?;
    computer.run()?;
    Ok(computer.output_as_string())
}

fn part_2(input: &str) -> Result<usize, ComputerError> {
    let computer: Computer = input.parse()?;

    let result = (0b000..=0b111)
        .find_map(|input| find_target_input(&computer, input))
        .ok_or(ComputerError::NoInputFound)?;

    Ok(result)
}

#[cfg(test)]
mod test {
    use ntest::timeout;

    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input).unwrap();

        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    #[timeout(60_000)]
    fn test_part_2_example_2() {
        let input = read_to_string("./input_example_2.txt").unwrap();
        let result = part_2(&input).unwrap();

        assert_eq!(result, 117440);
    }
}
