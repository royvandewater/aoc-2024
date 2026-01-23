mod computer;
mod custom_program;
mod custom_program_a;

use std::{fs::read_to_string, time::Instant};
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

    // let prototype = input.parse::<Computer>()?;

    // first output with length 16
    let start: usize = 35_184_372_088_832;
    // last output with length 16
    let end: usize = 281_474_976_710_655;
    //
    // 281474976710655 - 35184372088832 = 246_290_604_621_823
    // it currently takes 5 seconds per billion
    // we need to try 246_290 billion inputs, 5 * 246290 = 1,231,450 seconds, which is 14 weeks

    let mut start_time = Instant::now();

    for i in start..end {
        if i % 1_000_000_000 == 0 {
            let elapsed_ms = start_time.elapsed().as_millis();
            println!("i: {i}. elapsed: {elapsed_ms}ms");
            let output = custom_program::run_for_output(i);
            println!("output: {output:?}");
            start_time = Instant::now();
        }
        if custom_program::run(i) {
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
    fn test_part_2_example_2() {
        let input = read_to_string("./input_example_2.txt").unwrap();
        let result = part_2(&input).unwrap();

        assert_eq!(result, 117440);
    }
}
