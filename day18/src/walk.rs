use std::collections::{HashMap, HashSet, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type XY = (usize, usize);

#[derive(EnumIter)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Could not find path to end from")]
    NoPathFound,
}
use Error::*;

pub(crate) fn shortest_path_length(
    maze: &HashSet<XY>,
    bounds: (XY, XY),
    start: XY,
    end: XY,
) -> Result<usize, Error> {
    let path = shortest_path(maze, bounds, start, end)?;
    // we subtract 1 to avoid the fencepost error since the path includes both
    // the start and end nodes.
    Ok(path.len() - 1)
}

fn shortest_path(
    maze: &HashSet<XY>,
    bounds: (XY, XY),
    start: XY,
    end: XY,
) -> Result<Vec<XY>, Error> {
    let mut path_for_xy: HashMap<XY, Vec<XY>> = HashMap::from([(start, vec![start])]);
    let mut queue: VecDeque<XY> = VecDeque::from([start]);

    while let Some(current) = queue.pop_front() {
        let path = path_for_xy.get(&current).unwrap().clone();

        for xy in potential_next_steps(maze, bounds, current) {
            let new_path = [path.clone(), vec![xy]].concat();
            match path_for_xy.get(&xy) {
                Some(old_path) if old_path.len() <= new_path.len() => {}
                _ => {
                    path_for_xy.insert(xy, new_path);
                    queue.push_back(xy);
                }
            };
        }
    }

    let result = path_for_xy.get(&end).ok_or(NoPathFound)?;

    Ok(result.clone())
}

fn potential_next_steps(maze: &HashSet<XY>, bounds: (XY, XY), current: XY) -> Vec<XY> {
    let ((x_min, y_min), (x_max, y_max)) = bounds;
    let (x, y) = current;

    Directions::iter()
        .filter_map(|direction| match direction {
            Directions::Up if y > y_min => Some((x, y - 1)),
            Directions::Down if y < y_max => Some((x, y + 1)),
            Directions::Left if x > x_min => Some((x - 1, y)),
            Directions::Right if x < x_max => Some((x + 1, y)),
            _ => None,
        })
        .filter(|xy| !maze.contains(xy))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_move_to_the_right() {
        let maze = HashSet::new();
        let bounds = ((0, 0), (1, 0));
        let result = shortest_path_length(&maze, bounds, (0, 0), (1, 0)).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_two_moves_unobstructed() {
        let maze = HashSet::new();
        let bounds = ((0, 0), (1, 1));
        let result = shortest_path_length(&maze, bounds, (0, 0), (1, 1)).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_one_move_from_non_origin() {
        let maze = HashSet::new();
        let bounds = ((0, 0), (1, 1));
        let result = shortest_path_length(&maze, bounds, (1, 1), (1, 0)).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_path_one_move_from_non_origin() {
        let maze = HashSet::new();
        let bounds = ((0, 0), (1, 1));
        let result = shortest_path(&maze, bounds, (1, 1), (1, 0)).unwrap();
        assert_eq!(result, vec![(1, 1), (1, 0)]);
    }

    #[test]
    fn test_destination_obstructed() {
        let maze = HashSet::from([(1, 0)]);
        let bounds = ((0, 0), (2, 1));

        // S#E
        // ...
        let result = shortest_path_length(&maze, bounds, (0, 0), (2, 0)).unwrap();
        assert_eq!(result, 4); // down, right, right, up
    }

    #[test]
    fn test_destination_obstructed_path() {
        let maze = HashSet::from([(1, 0)]);
        let bounds = ((0, 0), (2, 1));

        // S#E
        // ...
        let result = shortest_path(&maze, bounds, (0, 0), (2, 0)).unwrap();
        assert_eq!(
            result,
            vec![
                (0, 0), // start
                (0, 1), // down
                (1, 1), // right
                (2, 1), // right
                (2, 0), // up (end)
            ]
        );
    }

    #[test]
    fn test_includes_dead_end() {
        let maze = HashSet::from([(1, 1), (2, 1)]);
        let bounds = ((0, 0), (2, 2));

        // S..
        // .##
        // ..E
        let result = shortest_path_length(&maze, bounds, (0, 0), (2, 2)).unwrap();
        assert_eq!(result, 4); // down, down, right, right
    }

    #[test]
    fn test_includes_dead_end_alternate() {
        let maze = HashSet::from([(1, 1), (1, 2)]);
        let bounds = ((0, 0), (2, 2));

        // S..
        // .#.
        // .#E
        let result = shortest_path_length(&maze, bounds, (0, 0), (2, 2)).unwrap();
        assert_eq!(result, 4); // right, right, down, down
    }
}
