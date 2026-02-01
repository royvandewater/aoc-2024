use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use memoize::memoize;

use crate::direction::Direction;
use crate::maze::Tile;

type XY = (usize, usize);

/// finds the shortes path to End tile from the given start position. Panics if grid does not
/// contain an End tile.
pub(crate) fn find_shortest_path(grid: &HashMap<XY, Tile>, start: XY) -> Option<Vec<XY>> {
    let (&end, _) = grid.iter().find(|(_xy, tile)| **tile == Tile::End).unwrap();
    walk(grid, HashSet::new(), end, start)
}

#[memoize(Ignore: grid, Ignore: visited)]
fn walk(grid: &HashMap<XY, Tile>, visited: HashSet<XY>, end: XY, current: XY) -> Option<Vec<XY>> {
    match grid.get(&current) {
        None | Some(Tile::Wall) => return None,
        Some(Tile::End) => return Some(vec![current]),
        _ if visited.contains(&current) => return None,
        _ => {}
    }

    let visited = visited
        .union(&HashSet::from([current]))
        .cloned()
        .collect::<HashSet<_>>();

    Direction::iter()
        .filter_map(|d| d.step(current))
        .collect::<Vec<_>>()
        .iter()
        .sorted_by(|&&a, &&b| distance_2(a, end).cmp(&distance_2(b, end)))
        .filter_map(|&next| Some([vec![current], walk(grid, visited.clone(), end, next)?].concat()))
        .next() // first 
}

/// finds the distance squared between a & b. Is useful for sorting
/// a collection of points by their distance to some other point.
fn distance_2(a: XY, b: XY) -> usize {
    let (x_a, y_a) = a;
    let (x_b, y_b) = b;
    let dx = x_a.abs_diff(x_b);
    let dy = y_a.abs_diff(y_b);

    dx.pow(2) + dy.pow(2)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::maze::Maze;
    use Direction::*;

    fn m(xy: XY, directions: Vec<Direction>) -> Vec<XY> {
        let (_, path) = directions
            .iter()
            .fold((xy, vec![xy]), |(current, path), direction| {
                let next = direction.step(current).unwrap();
                (next, [path, vec![next]].concat())
            });
        path
    }

    #[test]
    fn test_one_move_east() {
        let maze: Maze = "
            SE
        "
        .parse()
        .unwrap();

        let result = find_shortest_path(&maze.grid, maze.start).unwrap();

        assert_eq!(result, vec![(0, 0), (1, 0)]);
    }

    #[test]
    fn test_one_move_south() {
        let maze: Maze = "
            S
            E
        "
        .parse()
        .unwrap();

        let result = find_shortest_path(&maze.grid, maze.start).unwrap();

        assert_eq!(result, m((0, 0), vec![South]));
    }

    #[test]
    fn test_blocked_by_wall() {
        let maze: Maze = "
            S#E
            ...
        "
        .parse()
        .unwrap();

        let result = find_shortest_path(&maze.grid, maze.start).unwrap();

        assert_eq!(result, m((0, 0), vec![South, East, East, North]));
    }

    #[test]
    fn test_two_paths_one_shorter() {
        let maze: Maze = "
            SE
            ..
        "
        .parse()
        .unwrap();

        let result = find_shortest_path(&maze.grid, maze.start).unwrap();

        assert_eq!(result, m((0, 0), vec![East]));
    }
}
