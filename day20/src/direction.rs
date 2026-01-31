use std::slice::Iter;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(unused)]
pub(crate) enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::{East, North, South, West};

type XY = (usize, usize);

#[allow(unused)]
impl Direction {
    pub(crate) fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [North, South, East, West];
        DIRECTIONS.iter()
    }

    /// returns None if stepping would cause the coordinates to go
    /// negative (which would panic because x & y are unsigned)
    pub(crate) fn step(self, (x, y): XY) -> Option<XY> {
        match self {
            North if y > 0 => Some((x, y - 1)),
            South => Some((x, y + 1)),
            East => Some((x + 1, y)),
            West if x > 0 => Some((x - 1, y)),
            _ => None,
        }
    }

    pub(crate) fn cheat(self, xy: XY) -> Option<XY> {
        self.step(self.step(xy)?)
    }

    pub(crate) fn clockwise(self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub(crate) fn counter_clockwise(self) -> Direction {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    pub(crate) fn invert(self) -> Direction {
        self.clockwise().clockwise()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step_north() {
        assert_eq!(North.step((1, 1)), Some((1, 0)));
    }

    #[test]
    fn test_step_south() {
        assert_eq!(South.step((1, 1)), Some((1, 2)));
    }

    #[test]
    fn test_step_east() {
        assert_eq!(East.step((1, 1)), Some((2, 1)));
    }

    #[test]
    fn test_step_west() {
        assert_eq!(West.step((1, 1)), Some((0, 1)));
    }

    #[test]
    fn test_step_north_when_0() {
        assert_eq!(North.step((0, 0)), None);
    }

    #[test]
    fn test_step_west_when_0() {
        assert_eq!(West.step((0, 0)), None);
    }
}
