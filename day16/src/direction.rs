use std::slice::Iter;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(unused)]
pub(crate) enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;

impl Direction {
    pub(crate) fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [North, South, East, West];
        DIRECTIONS.iter()
    }

    pub(crate) fn clockwise(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub(crate) fn counter_clockwise(&self) -> Direction {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    pub(crate) fn invert(&self) -> Direction {
        self.clockwise().clockwise()
    }
}
