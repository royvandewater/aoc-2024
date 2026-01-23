mod computer;
mod reverse_computer;

use std::fs::read_to_string;
use thiserror::Error;

use crate::{
    computer::{Computer, ComputerParseError, RuntimeError},
    reverse_computer::{ReverseComputer, ReverseError},
};

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

    #[error("Reverse Error")]
    ReverseError(#[from] ReverseError),
}

fn part_1(input: &str) -> Result<String, ComputerError> {
    let mut computer: Computer = input.parse()?;
    computer.run()?;
    Ok(computer.output_as_string())
}

#[allow(unused)]
fn part_2(input: &str) -> Result<usize, ComputerError> {
    let prototype: Computer = input.parse()?;
    let mut reverse_computer: ReverseComputer = prototype.into();
    let a = reverse_computer.run()?;
    Ok(a)

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

    // let start: usize = 0;
    // for i in start.. {
    //     for suffix in suffixes.iter() {
    //         let x = (i << 29) | suffix;

    //         // println!("i:      {i}");
    //         // println!("suffix: {i:b}{suffix:b}");
    //         // println!("x:      {x:b}");
    //         let mut computer = prototype.clone();
    //         computer.initialize_a(x);
    //         computer.run_for_matching_output()?;

    //         // Program: 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0
    //         let prefix: Vec<usize> = vec![2, 4, 1, 1, 7, 5, 4, 4, 1, 4];
    //         if computer.output_starts_with(&prefix) {
    //             // println!("i: {i}");
    //             println!("x:b: {x:b}\t{x}");
    //             // println!("program: {}", computer.program_as_string());
    //             // println!("output:  {}", computer.output_as_string());
    //         }

    //         if computer.has_output_itself() {
    //             return Ok(i);
    //         }
    //     }
    // }

    // panic!("Should be unreachable")
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
