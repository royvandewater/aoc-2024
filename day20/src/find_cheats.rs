use std::collections::{HashMap, HashSet};
use std::slice::Iter;

use crate::maze::{Maze, Tile};

type XY = (usize, usize);
type Move = (XY, XY);

pub(crate) fn find_cheats(maze: &Maze, max_length: usize) -> Vec<Vec<Move>> {
    walk(&maze.grid, &HashSet::new(), maze.start, max_length, false).unwrap_or(vec![])
}

fn walk(
    grid: &HashMap<XY, Tile>,
    visited: &HashSet<XY>,
    current: XY,
    steps_remaining: usize,
    cheat_used: bool,
) -> Option<Vec<Vec<Move>>> {
    match grid.get(&current) {
        Some(Tile::End) => return Some(vec![vec![]]),
        None | Some(Tile::Wall) => return None,
        _ if steps_remaining == 0 => return None,
        _ => {}
    }

    let visited: HashSet<XY> = HashSet::from([current]).union(visited).cloned().collect();

    let paths: Vec<Vec<Move>> = Movement::iter()
        .filter_map(|movement| {
            let next = movement.step(current)?;
            if visited.contains(&next) {
                return None;
            }

            if cheat_used && movement.is_cheat() {
                return None;
            }

            let sub_paths = walk(
                grid,
                &visited,
                next,
                steps_remaining - 1,
                cheat_used || movement.is_cheat(),
            )?;

            Some(
                sub_paths
                    .iter()
                    .map(|p| [vec![(current, next)], p.clone()].concat())
                    .collect::<Vec<Vec<Move>>>(),
            )
        })
        .flatten()
        .collect();

    Some(paths)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Movement {
    North,
    South,
    East,
    West,
    CheatNorth,
    CheatSouth,
    CheatEast,
    CheatWest,
}

use Movement::*;

impl Movement {
    pub(crate) fn iter() -> Iter<'static, Movement> {
        static MOVEMENT: [Movement; 8] = [
            North, South, East, West, CheatNorth, CheatSouth, CheatEast, CheatWest,
        ];
        MOVEMENT.iter()
    }

    fn is_cheat(self) -> bool {
        matches!(self, CheatNorth | CheatSouth | CheatEast | CheatWest)
    }

    fn step(self, (x, y): XY) -> Option<XY> {
        match self {
            North if y > 0 => Some((x, y - 1)),
            South => Some((x, y + 1)),
            East => Some((x + 1, y)),
            West if x > 0 => Some((x - 1, y)),
            CheatNorth if y > 1 => Some((x, y - 2)),
            CheatSouth => Some((x, y + 2)),
            CheatEast => Some((x + 2, y)),
            CheatWest if x > 1 => Some((x - 2, y)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(unused)]
    fn m(xy: XY, directions: Vec<Movement>) -> Vec<Move> {
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
    fn test_no_cheats_possible() {
        let maze: Maze = "
            SE
        "
        .parse()
        .unwrap();

        let result: Vec<Vec<Move>> = find_cheats(&maze, 0);
        let expected: Vec<Vec<Move>> = vec![];

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

        let result: Vec<Vec<Move>> = find_cheats(&maze, 1);
        let expected: Vec<Vec<Move>> = vec![m((0, 0), vec![CheatEast])];

        assert_eq!(result, expected);
    }
}
