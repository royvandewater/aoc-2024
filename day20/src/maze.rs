use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

type XY = (usize, usize);
type Bounds = (XY, XY);

pub(crate) struct Maze {
    pub(crate) grid: HashMap<XY, Tile>,
    #[allow(unused)]
    pub(crate) bounds: Bounds,
    pub(crate) start: XY,
    #[allow(unused)]
    pub(crate) end: XY,
}

#[derive(Debug, Error)]
pub(crate) enum MazeParseError {
    #[error("Failed to parse Tile")]
    TileParseError(#[from] TileParseError),

    #[error("Start tile could not be found in maze")]
    StartNotFound,

    #[error("End tile could not be found in maze")]
    EndNotFound,
}

impl FromStr for Maze {
    type Err = MazeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: HashMap<XY, Tile> = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| ((x, y), c))
            })
            .map(|(xy, c)| match Tile::try_from(c) {
                Ok(t) => Ok((xy, t)),
                Err(e) => Err(e),
            })
            .try_collect()?;

        let max_x = *grid.keys().map(|(x, _y)| x).max().unwrap_or(&0);
        let max_y = *grid.keys().map(|(_x, y)| y).max().unwrap_or(&0);
        let (start, _) = grid
            .iter()
            .find(|(_xy, tile)| tile.is_start())
            .ok_or(MazeParseError::StartNotFound)?;

        let (end, _) = grid
            .iter()
            .find(|(_xy, tile)| tile.is_end())
            .ok_or(MazeParseError::EndNotFound)?;

        Ok(Maze {
            grid: grid.clone(),
            bounds: ((0, 0), (max_x, max_y)),
            start: *start,
            end: *end,
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Tile {
    Start,
    End,
    Wall,
    Empty,
}

impl Tile {
    fn is_start(&self) -> bool {
        matches!(self, Tile::Start)
    }

    fn is_end(&self) -> bool {
        matches!(self, Tile::End)
    }
}

#[derive(Debug, Error)]
pub(crate) enum TileParseError {
    #[error("Unrecognized char: {0}")]
    UnrecognizedChar(char),
}

impl TryFrom<char> for Tile {
    type Error = TileParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            '.' => Ok(Tile::Empty),
            _ => Err(TileParseError::UnrecognizedChar(value)),
        }
    }
}
