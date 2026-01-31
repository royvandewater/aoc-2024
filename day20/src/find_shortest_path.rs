use std::collections::{HashMap, HashSet};

use crate::direction::Direction;
use crate::maze::Tile;

type XY = (usize, usize);

pub(crate) fn find_shortest_path(grid: &HashMap<XY, Tile>, start: XY) -> Option<Vec<XY>> {
    walk(grid, &HashSet::new(), start)
}

fn walk(grid: &HashMap<XY, Tile>, visited: &HashSet<XY>, current: XY) -> Option<Vec<XY>> {
    match grid.get(&current) {
        None | Some(Tile::Wall) => return None,
        Some(Tile::End) => return Some(vec![current]),
        _ => {}
    }

    let visited: HashSet<XY> = HashSet::from([current]).union(visited).cloned().collect();

    Direction::iter()
        .filter_map(|d| {
            let next = d.step(current)?;
            match visited.contains(&next) {
                true => None,
                false => Some(next),
            }
        })
        .filter_map(|next| Some([vec![current], walk(grid, &visited, next)?].concat()))
        .min_by(|a, b| a.len().cmp(&b.len()))
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
