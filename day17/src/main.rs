mod computer;

use std::fs::read_to_string;
use thiserror::Error;

use crate::computer::{Computer, ComputerParseError, RuntimeError};

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
}

fn part_1(input: &str) -> Result<String, ComputerError> {
    let mut computer: Computer = input.parse()?;
    computer.run()?;
    Ok(computer.output_as_string())
}

#[allow(unused)]
fn part_2(input: &str) -> Result<usize, ComputerError> {
    // we already know it's not in the first 20 billion
    // let start = 20_000_000_000;
    // let start = 280_000_000_000_000; it's somewhere around this order or magnitude
    // , but less than 290 trillion
    //
    //

    // let suffixes: [usize; 6] = [
    //     0b10101011010000010101000101010,
    //     0b10101011010000010101000101101,
    //     0b10101011010000010101000101111,
    //     0b11101011010000010101000101010,
    //     0b11101011010000010101000101101,
    //     0b11101011010000010101000101111,
    // ];

    let prototype = input.parse::<Computer>()?;

    let start: usize = 0;
    for i in start.. {
        let mut computer = prototype.clone();
        computer.initialize_a(i);
        computer.run_for_matching_output()?;

        if computer.has_output_itself() {
            return Ok(i);
        }
    }

    panic!("Should be unreachable")
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
    #[ignore]
    fn test_part_2_example_2() {
        let input = read_to_string("./input_example_2.txt").unwrap();
        let result = part_2(&input).unwrap();

        assert_eq!(result, 117440);
    }
}
