mod fmt_maze;
mod walk;

use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};
use thiserror::Error;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input, 1024, (70, 70)).unwrap());
}

type XY = (usize, usize);

#[derive(Debug, Error)]
enum Day18Error {
    #[error("Invalid Coordinate: {0}")]
    InvalidCoordinate(String),

    #[error("Unable to walk")]
    WalkError(#[from] walk::Error),
}

use Day18Error::*;

fn part_1(input: &str, first: usize, end: XY) -> Result<usize, Day18Error> {
    let maze: HashSet<XY> = input
        .trim()
        .lines()
        .take(first)
        .map(parse_line)
        .try_collect()?;

    let bounds = ((0, 0), end);

    Ok(walk::shortest_path_length(&maze, bounds, (0, 0), end)?)
}

fn parse_line(line: &str) -> Result<XY, Day18Error> {
    line.trim()
        .split(",")
        .flat_map(|c| c.parse::<usize>())
        .collect_tuple()
        .ok_or(InvalidCoordinate(line.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input, 12, (6, 6)).unwrap();

        assert_eq!(result, 22);
    }
}
