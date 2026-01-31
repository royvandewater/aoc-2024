use std::collections::{HashMap, HashSet};

use crate::direction::Direction;
use crate::maze::{Maze, Tile};

type XY = (usize, usize);
type Move = (XY, XY);

pub(crate) fn find_shortest_path(maze: &Maze) -> Option<Vec<Move>> {
    walk(&maze.grid, &HashSet::new(), maze.start)
}

fn walk(grid: &HashMap<XY, Tile>, visited: &HashSet<XY>, current: XY) -> Option<Vec<Move>> {
    match grid.get(&current) {
        None | Some(Tile::Wall) => return None,
        Some(Tile::End) => return Some(vec![]),
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
        .filter_map(|next| Some([vec![(current, next)], walk(grid, &visited, next)?].concat()))
        .min_by(|a, b| a.len().cmp(&b.len()))
}

#[cfg(test)]
mod test {
    use super::*;
    use Direction::*;

    fn m(xy: XY, directions: Vec<Direction>) -> Vec<Move> {
        let (_, path) = directions
            .iter()
            .fold((xy, vec![]), |(current, path), direction| {
                let next = direction.step(current).unwrap();
                let current_move: Move = (current, next);
                (next, [path, vec![current_move]].concat())
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

        let result = find_shortest_path(&maze).unwrap();

        assert_eq!(result, vec![((0, 0), (1, 0))]);
    }

    #[test]
    fn test_one_move_south() {
        let maze: Maze = "
            S
            E
        "
        .parse()
        .unwrap();

        let result = find_shortest_path(&maze).unwrap();

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

        let result = find_shortest_path(&maze).unwrap();

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

        let result = find_shortest_path(&maze).unwrap();

        assert_eq!(result, m((0, 0), vec![East]));
    }
}
