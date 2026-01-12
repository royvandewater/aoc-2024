mod iter;

use std::{collections::HashMap, fmt::Display, str::FromStr};
use thiserror::Error;
use iter::Iter;

type XY = (usize, usize);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Part1State {
    tiles: HashMap<XY, char>,
    robot: XY,
   instructions: Vec<char>,
}

impl Part1State {
    pub(crate) fn score(&self) -> usize {
        self.tiles
            .iter()
            .filter_map(|(xy, tile)| match *tile == 'O' {
                true => Some(*xy),
                false => None,
            })
            .map(|(x, y)| x + (100 * y))
            .sum()
    }

    pub(crate) fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

#[derive(Debug, Error)]
pub enum Part1StateParseError {
    #[error("Malformed State: {0}")]
    MalformedState(String),

    #[error("No Robot found on map: {0}")]
    NoRobotFound(String),
}

impl Display for Part1State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.tiles.keys().map(|(x,_y)| x).max();
        let max_y = self.tiles.keys().map(|(_x,y)| y).max();

        match (max_x, max_y) {
            (Some(max_x), Some(max_y)) => {
                for y in 0..=*max_y {
                    for x in 0..=*max_x {
                        match self.tiles.get(&(x,y)) {
                            None => panic!("No tile available for coordinates: {:?}", (x,y)),
                            Some(tile) => f.write_fmt(format_args!("{}",tile))?,
                        }
                    }

                    f.write_fmt(format_args!("\n"))?;
                }
            }
            _ => panic!("Received state with no tiles"),
        };

        f.write_fmt(format_args!("\n"))?;

        for instruction in self.instructions.iter() {
            f.write_fmt(format_args!("{}", *instruction))?;
        }

        f.write_fmt(format_args!("\n"))
    }
}

use Part1StateParseError::*;

impl FromStr for Part1State {
    type Err = Part1StateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tiles, instructions) = s.trim().split_once("\n\n").ok_or(MalformedState(s.to_string()))?;
        let tiles: HashMap<XY, char> = tiles.lines().enumerate().flat_map(|(y, line)| {
            line.trim().chars().enumerate().map(move |(x, c)| ((x,y), c))
        }).collect();
        let instructions: Vec<char> = instructions.lines().flat_map(|line| line.trim().chars()).collect();

        let robot = tiles.iter().find_map(|(xy, c)| match c {
            '@' => Some(*xy),
            _ => None,
        }).ok_or(NoRobotFound(s.to_string()))?;



        Ok(Part1State{ tiles, robot, instructions })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score_example() {
        let input = "
            #######
            #...O..
            #.....@

            <
        ";
        let sut: Part1State = input.parse().unwrap();
        let result = sut.score();
        assert_eq!(result, 104);
    }

    #[test]
    fn test_score_example_small() {
        let input = "
            ########
            #....OO#
            ##.....#
            #.....O#
            #.#O@..#
            #...O..#
            #...O..#
            ########

            <
        ";
        let sut: Part1State = input.parse().unwrap();
        let result = sut.score();
        assert_eq!(result, 2028);
    }

    #[test]
    fn test_iter_base() {
        let input = "
            .@

            <<
        ";
        let sut: Part1State = input.parse().unwrap();
        let result = sut.iter().next();

        assert_eq!(result, Some("
            @.

            <
        ".parse().unwrap()));
    }

    #[test]
    fn test_iter_wall() {
        let input = "
            #@

            <<
        ";
        let sut: Part1State = input.parse().unwrap();
        let result = sut.iter().next();

        assert_eq!(result, Some("
            #@

            <
        ".parse().unwrap()));
    }

    #[test]
    fn test_iter_box() {
        let input = "
            .O@

            <<
        ";
        let sut: Part1State = input.parse().unwrap();
        let result = sut.iter().next();

        assert_eq!(result, Some("
            O@.

            <
        ".parse().unwrap()));
    }

    #[test]
    fn test_iter_box_wall() {
        let input = "
            #O@

            <<
        ";
        let sut: Part1State = input.parse().unwrap();
        let result = sut.iter().next();

        assert_eq!(result, Some("
            #O@

            <
        ".parse().unwrap()));
    }

    #[test]
    fn test_parse_base() {
        let input = "
            @

            <
        ";
        let result: Part1State = input.parse().unwrap();

        assert_eq!(result, Part1State{
            tiles: HashMap::from([ ((0,0), '@'), ]),
            robot: (0,0),
            instructions: vec!['<'],
        });
    }

    #[test]
    fn test_parse_multiline_map() {
        let input = "
            #@
            .O

            <
        ";
        let result: Part1State = input.parse().unwrap();

        assert_eq!(result, Part1State{
            tiles: HashMap::from([ ((0,0), '#'), ((1,0), '@'), ((0,1), '.'), ((1,1), 'O')]),
            robot: (1,0),
            instructions: vec!['<'],
        });
    }

    #[test]
    fn test_parse_multiline_instructions() {
        let input = "
            @

            <>
            ^v
        ";
        let result: Part1State = input.parse().unwrap();

        assert_eq!(result, Part1State{
            tiles: HashMap::from([ ((0,0), '@') ]),
            robot: (0,0),
            instructions: vec!['<', '>', '^', 'v'],
        });
    }
}
