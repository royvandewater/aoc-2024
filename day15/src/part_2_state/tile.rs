
use std::fmt::{Display, Write};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Tile {
    Empty = b'.',
    Robot = b'@', 
    LeftBox = b'[',
    RightBox = b']',
    Wall = b'#',
}

#[derive(Debug)]
pub struct InvalidTileError();

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl From<Tile> for char {
    fn from(val: Tile) -> Self  {
        (&val).into()
    }
}

impl From<&Tile> for char {
    fn from(val: &Tile) -> Self  {
        match val {
            Tile::Empty => '.',
            Tile::Robot => '@',
            Tile::LeftBox => '[',
            Tile::RightBox => ']',
            Tile::Wall => '#',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = InvalidTileError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '@' => Ok(Tile::Robot),
            '[' => Ok(Tile::LeftBox),
            ']' => Ok(Tile::RightBox),
            '#' => Ok(Tile::Wall),
            _ => Err(InvalidTileError()),
        }
    }
}
