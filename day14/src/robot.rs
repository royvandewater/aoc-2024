use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};
use thiserror::Error;

type XY = (isize, isize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Robot {
    pub(crate) position: XY,
    velocity: XY,
    bounds: XY,
}

impl Robot {
    #[allow(unused)]
    pub(crate) fn new(position: XY, velocity: XY, bounds: XY) -> Robot {
        Robot {
            position,
            velocity,
            bounds,
        }
    }

    pub(crate) fn with_bounds(&self, bounds: (isize, isize)) -> Robot {
        let mut robot = self.clone();
        robot.bounds = bounds;
        robot
    }

    pub(crate) fn quadrant(&self) -> Option<u8> {
        let half_x = self.bounds.0 / 2;
        let half_y = self.bounds.1 / 2;

        match self.position {
            (x, y) if x == half_x || y == half_y => None,
            (x, y) if x < half_x && y < half_y => Some(1),
            (x, y) if half_x < x && y < half_y => Some(2),
            (x, y) if x < half_x && half_y < y => Some(3),
            (x, y) if half_x < x && half_y < y => Some(4),
            _ => panic!(
                "Position is out of bounds! position: {:?}, bounds: {:?}",
                self.position, self.bounds
            ),
        }
    }
}

#[derive(Debug, Error)]
pub enum RobotParseError {
    #[error("Malformed Robot String {0}")]
    MalformedString(String),

    #[error("Malformed integer in robot string")]
    ParseIntError(#[from] std::num::ParseIntError),
}

use RobotParseError::*;

impl FromStr for Robot {
    type Err = RobotParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (px, py, vx, vy) = s
            .replace("p=", "")
            .replace("v=", "")
            .split([',', ' '])
            .map(|v| v.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect_tuple()
            .ok_or(MalformedString(s.to_string()))?;

        Ok(Robot {
            position: (px, py),
            velocity: (vx, vy),
            bounds: (isize::MAX, isize::MAX),
        })
    }
}

impl Iterator for Robot {
    type Item = Robot;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut x, mut y) = self.position;
        let (vx, vy) = self.velocity;
        let (bx, by) = self.bounds;

        x += vx;
        while x < 0 {
            x += bx;
        }
        x %= bx;

        y += vy;
        while y < 0 {
            y += by;
        }
        y %= by;

        self.position = (x, y);
        Some(*self)
    }
}

impl Into<(XY, XY)> for Robot {
    fn into(self) -> (XY, XY) {
        (self.position, self.velocity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Robots {
    robots: Vec<Robot>,
    bounds: XY,
}

impl Robots {
    pub(crate) fn might_be_christmas_tree(&self) -> bool {
        let positions: HashSet<XY> = self.robots.iter().map(|r| r.position).collect();

        positions.len() == self.robots.len()
    }
}

impl Display for Robots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_per_position = self
            .robots
            .iter()
            .map(|r| (r.position, r))
            .into_group_map()
            .iter()
            .map(|(key, values)| (*key, values.len()))
            .collect::<HashMap<(isize, isize), usize>>();

        let (bx, by) = self.bounds;

        for y in 0..by {
            for x in 0..bx {
                match num_per_position.get(&(x, y)) {
                    Some(num) => f.write_fmt(format_args!("{}", num))?,
                    None => f.write_str(".")?,
                };
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl FromIterator<Robot> for Robots {
    fn from_iter<T: IntoIterator<Item = Robot>>(iter: T) -> Self {
        let robots: Vec<Robot> = iter.into_iter().collect();

        Robots {
            robots: robots.clone(),
            bounds: robots
                .first()
                .and_then(|r| Some(r.bounds))
                .unwrap_or((isize::MAX, isize::MAX)),
        }
    }
}

impl Iterator for Robots {
    type Item = Robots;

    fn next(&mut self) -> Option<Self::Item> {
        for robot in self.robots.iter_mut() {
            robot.next().unwrap();
        }

        Some(self.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_string() {
        let result = "".parse::<Robot>();
        assert!(result.is_err());
    }

    #[test]
    fn test_example_robot_1() {
        let result: Robot = "p=0,4 v=3,-3".parse().unwrap();
        assert_eq!(
            result,
            Robot {
                position: (0, 4),
                velocity: (3, -3),
                bounds: (isize::MAX, isize::MAX),
            }
        );
    }

    #[test]
    fn test_iterator_base() {
        let robot = Robot::new((0, 0), (1, 1), (11, 7)).next().unwrap();

        assert_eq!(robot, Robot::new((1, 1), (1, 1), (11, 7)))
    }

    #[test]
    fn test_iterator_wrap_x() {
        let robot = Robot::new((10, 0), (2, 2), (11, 7)).next().unwrap();

        assert_eq!(robot, Robot::new((1, 2), (2, 2), (11, 7)))
    }

    #[test]
    fn test_iterator_wrap_y() {
        let robot = Robot::new((0, 6), (2, 2), (11, 7)).next().unwrap();

        assert_eq!(robot, Robot::new((2, 1), (2, 2), (11, 7)))
    }

    #[test]
    fn test_iterator_wrap_y_negative() {
        let robot = Robot::new((2, 4), (2, -3), (11, 7)).nth(1).unwrap();

        assert_eq!(robot, Robot::new((6, 5), (2, -3), (11, 7)))
    }

    #[test]
    fn test_quadrant_1() {
        let robot = Robot {
            position: (0, 0),
            velocity: (0, 0), // irrelevant
            bounds: (11, 7),
        };

        assert_eq!(robot.quadrant(), Some(1));
    }

    #[test]
    fn test_quadrant_2() {
        let robot = Robot {
            position: (7, 0),
            velocity: (0, 0), // irrelevant
            bounds: (11, 7),
        };

        assert_eq!(robot.quadrant(), Some(2));
    }

    #[test]
    fn test_quadrant_3() {
        let robot = Robot {
            position: (0, 6),
            velocity: (0, 0), // irrelevant
            bounds: (11, 7),
        };

        assert_eq!(robot.quadrant(), Some(3));
    }

    #[test]
    fn test_quadrant_4() {
        let robot = Robot {
            position: (10, 6),
            velocity: (0, 0), // irrelevant
            bounds: (11, 7),
        };

        assert_eq!(robot.quadrant(), Some(4));
    }

    #[test]
    fn test_quadrant_exactly_between_1_and_2() {
        let robot = Robot {
            position: (5, 0),
            velocity: (0, 0), // irrelevant
            bounds: (11, 7),
        };

        assert_eq!(robot.quadrant(), None);
    }

    #[test]
    fn test_quadrant_exactly_between_1_and_3() {
        let robot = Robot {
            position: (0, 3),
            velocity: (0, 0), // irrelevant
            bounds: (11, 7),
        };

        assert_eq!(robot.quadrant(), None);
    }

    #[test]
    fn test_is_christmas_tree_when_the_unit_might_be_tree() {
        let robots: Robots = vec![Robot::new((0, 0), (0, 0), (1, 1))]
            .into_iter()
            .collect();

        assert!(robots.might_be_christmas_tree());
    }
}
