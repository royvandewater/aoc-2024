use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Write},
};

#[allow(unused)]
type Maze = HashSet<XY>;
#[allow(unused)]
type Visited = HashSet<XY>;
type XY = (usize, usize);
type Bounds = (XY, XY);

#[allow(unused)]
pub(crate) struct FormattedMaze {
    tiles: HashMap<XY, char>,
    bounds: Bounds,
}

impl Display for FormattedMaze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ((x_min, y_min), (x_max, y_max)) = self.bounds;

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let tile = self.tiles.get(&(x, y)).unwrap_or(&'.');
                f.write_char(*tile)?;
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl From<(&Maze, Bounds)> for FormattedMaze {
    fn from((maze, bounds): (&Maze, Bounds)) -> Self {
        let maze: HashMap<XY, char> = maze.iter().map(|xy| (*xy, '#')).collect();

        let mut tiles = HashMap::new();
        tiles.extend(maze);

        FormattedMaze { tiles, bounds }
    }
}

impl From<(&Maze, &Visited, Bounds)> for FormattedMaze {
    fn from((maze, visited, bounds): (&Maze, &Visited, Bounds)) -> Self {
        let maze: HashMap<XY, char> = maze.iter().map(|xy| (*xy, '#')).collect();
        let visited: HashMap<XY, char> = visited.iter().map(|xy| (*xy, '*')).collect();

        let mut tiles = HashMap::new();
        tiles.extend(maze);
        tiles.extend(visited);

        FormattedMaze { tiles, bounds }
    }
}
