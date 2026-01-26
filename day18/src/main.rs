mod fmt_maze;
mod walk;

use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};
use thiserror::Error;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input, 1024, (70, 70)).unwrap());
    println!("part_2: {:?}", part_2(&input, (70, 70)).unwrap());
}

type XY = (usize, usize);

#[derive(Debug, Error)]
enum Day18Error {
    #[error("Invalid Coordinate: {0}")]
    InvalidCoordinate(String),

    #[error("Unable to walk")]
    WalkError(#[from] walk::Error),

    #[error("Was always able to walk from start to finish")]
    NeverBlocked,
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

fn part_2(input: &str, end: XY) -> Result<XY, Day18Error> {
    let coordinates: Vec<XY> = input.trim().lines().map(parse_line).try_collect()?;
    let start = (0, 0);
    let bounds = (start, end);

    for (i, xy) in coordinates.iter().enumerate().progress() {
        let maze: HashSet<XY> = coordinates.iter().take(i + 1).cloned().collect();

        match walk::shortest_path_length(&maze, bounds, start, end) {
            Ok(_) => {
                continue;
            }
            Err(e) => match e {
                walk::Error::NoPathFound => return Ok(*xy),
            },
        }
    }

    Err(NeverBlocked)
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

    #[test]
    fn test_part_2_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_2(&input, (6, 6)).unwrap();

        assert_eq!(result, (6, 1));
    }
}
