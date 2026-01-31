mod direction;
mod find_cheats;
mod find_shortest_path;
mod maze;

use find_cheats::find_cheats;
use find_shortest_path::find_shortest_path;
use maze::Maze;
use std::fs::read_to_string;
use thiserror::Error;

use crate::maze::MazeParseError;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input, 100).unwrap());
}

#[derive(Debug, Error)]
enum Day20Error {
    #[error("Error parsing maze")]
    MazeParseError(#[from] MazeParseError),

    #[error("No path found in maze")]
    NoPathFound,
}

fn part_1(input: &str, threshold: usize) -> Result<usize, Day20Error> {
    let maze: Maze = input.parse()?;

    let shortest_path_len = find_shortest_path(&maze)
        .ok_or(Day20Error::NoPathFound)?
        .len();

    println!("shortest_path_len: {shortest_path_len}");

    let max_cheat_path_len = shortest_path_len - threshold;

    Ok(find_cheats(&maze, max_cheat_path_len).len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input, 20).unwrap();
        assert_eq!(result, 5);
    }
}
