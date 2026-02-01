use std::collections::HashMap;

use indicatif::ProgressIterator;

use crate::direction::Direction;
use crate::find_shortest_path;
use crate::maze::Tile;

type XY = (usize, usize);

pub(crate) fn find_cheats(grid: &HashMap<XY, Tile>, start: XY, max_length: usize) -> Vec<Vec<XY>> {
    let shortest_path = find_shortest_path(grid, grid.len(), start).unwrap();

    let path_prefixes: Vec<Vec<XY>> = shortest_path
        .iter()
        .enumerate()
        .map(|(i, _)| {
            shortest_path
                .iter()
                .take(i + 1)
                .cloned()
                .collect::<Vec<XY>>()
        })
        .collect();

    path_prefixes
        .iter()
        .progress()
        .flat_map(|prefix| {
            if max_length < prefix.len() {
                return vec![];
            }

            Direction::iter()
                .filter_map(|d| {
                    let prefix = prefix.clone();
                    let steps_left = max_length - prefix.len();
                    let d = *d;
                    let current = *prefix.last().unwrap();
                    let next = d.cheat(current)?;
                    Some([prefix.clone(), find_shortest_path(grid, steps_left, next)?].concat())
                })
                .collect()
        })
        .filter(|path| path.len() <= max_length)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::maze::Maze;

    #[test]
    fn test_no_cheats_possible() {
        let maze: Maze = "
            SE
        "
        .parse()
        .unwrap();

        let result: Vec<Vec<XY>> = find_cheats(&maze.grid, maze.start, 0);
        let expected: Vec<Vec<XY>> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cheat_base_case() {
        let maze: Maze = "
            S#E
            ...
        "
        .parse()
        .unwrap();

        let result: Vec<Vec<XY>> = find_cheats(&maze.grid, maze.start, 2);
        let expected: Vec<Vec<XY>> = vec![vec![(0, 0), (2, 0)]];

        assert_eq!(result, expected);
    }
}
