use std::fmt::Display;

use crate::direction::Direction;

use Direction::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct XY(usize, usize);

impl XY {
    pub(crate) fn new(x: usize, y: usize) -> XY {
        XY(x, y)
    }

    pub(crate) fn apply_direction(&self, direction: &Direction) -> XY {
        let (x, y) = self.into();

        match direction {
            North => XY::new(x, y - 1),
            South => XY::new(x, y + 1),
            East => XY::new(x + 1, y),
            West => XY::new(x - 1, y),
        }
    }
}

impl Display for XY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.into();
        f.write_fmt(format_args!("({}, {})", x, y))
    }
}

impl From<(usize, usize)> for XY {
    fn from((x, y): (usize, usize)) -> Self {
        XY(x, y)
    }
}

impl From<&XY> for (usize, usize) {
    fn from(val: &XY) -> Self {
        (val.0, val.1)
    }
}
