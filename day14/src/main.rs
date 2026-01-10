mod robot;

use itertools::Itertools;
use robot::{Robot, RobotParseError, Robots};
use std::fs::read_to_string;
use thiserror::Error;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input, (101, 103)).unwrap());
    println!("part_2: {}", part_2(&input, (101, 103)).unwrap());
}

#[derive(Debug, Error)]
enum Part1Error {
    #[error("Failed to parse a robot")]
    RobotParseError(#[from] RobotParseError),
}

fn part_1(input: &str, bounds: (isize, isize)) -> Result<usize, Part1Error> {
    Ok(input
        .trim()
        .lines()
        .map(|l| l.parse::<Robot>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|r| r.with_bounds(bounds))
        .map(|r| r.clone().nth(99).unwrap())
        .filter_map(|r| Some((r.quadrant()?, r)))
        .into_group_map()
        .values()
        .map(|robots| robots.len())
        .product())
}

#[derive(Debug, Error)]
enum Part2Error {
    #[error("Failed to parse a robot")]
    RobotParseError(#[from] RobotParseError),

    #[error("Failed to find a christmas tree")]
    NoChristmasTreeFound(),
}

use Part2Error::*;

fn part_2(input: &str, bounds: (isize, isize)) -> Result<usize, Part2Error> {
    let (i, robots) = input
        .trim()
        .lines()
        .map(|l| l.parse::<Robot>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|r| r.with_bounds(bounds))
        .collect::<Robots>()
        .enumerate()
        .filter(|(_i, robots)| robots.might_be_christmas_tree())
        .take(2)
        .last()
        .ok_or(NoChristmasTreeFound())?;

    println!("{}", robots);
    Ok(i + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input, (11, 7)).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_1_single_robot() {
        let input = "p=2,4 v=2,-3";
        let result = part_1(&input, (11, 7)).unwrap();
        assert_eq!(result, 1);
    }
}
