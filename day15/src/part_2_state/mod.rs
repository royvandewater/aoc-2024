mod iter;
mod tile;

use std::{collections::HashMap, fmt::Display, str::FromStr};
use thiserror::Error;
use iter::Iter;
use tile::Tile;

type XY = (usize, usize);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Part2State {
    tiles: HashMap<XY, Tile>,
    robot: XY,
   instructions: Vec<char>,
}

impl Part2State {
    #[allow(unused)]
    // doesn't validate that the input tiles are valid, doesn't validate that a robot exists.
    // robot defaults to 0,0 if no present in the string. It's also totally fine if instructions
    // are missing, it'll just default them to an empty vector
    fn from_raw(input: &str) -> Part2State {
        let input = input.trim();
        let (tiles_str, instructions_str) = input.split_once("\n\n").unwrap_or((input, ""));

        
        let tiles: HashMap<XY, Tile> = tiles_str.lines().enumerate().flat_map(|(y, line)| {
            line.trim().chars().enumerate().map(move |(x, c)| ((x, y), c.try_into().unwrap()))
        }).collect();

        let (robot,_) = tiles.clone().into_iter().find(|(xy, c)| *c == Tile::Robot).unwrap_or(((0,0), Tile::Robot));

        let instructions: Vec<char> = instructions_str.lines().flat_map(|l| l.trim().chars()).collect();

        Part2State {
            tiles, robot, instructions
        }
    }

    pub(crate) fn score(&self) -> usize {
        self.tiles
            .iter()
            .filter_map(|(xy, tile)| match *tile == Tile::LeftBox {
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

impl Display for Part2State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.tiles.keys().map(|(x,_y)| x).max();
        let max_y = self.tiles.keys().map(|(_x,y)| y).max();

        match (max_x, max_y) {
            (Some(max_x), Some(max_y)) => {
                for y in 0..=*max_y {
                    for x in 0..=*max_x {
                        match self.tiles.get(&(x,y)) {
                            None => panic!("No tile available for coordinates: {:?}", (x,y)),
                            Some(tile) => f.write_fmt(format_args!("{}", tile))?,
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

#[derive(Debug, Error)]
pub enum Part2StateParseError {
    #[error("Malformed State: {0}")]
    MalformedState(String),

    #[error("No Robot found on map: {0}")]
    NoRobotFound(String),

    #[error("Invalid tile: {0}")]
    InvalidTile(char)
}

use Part2StateParseError::*;

impl FromStr for Part2State {
    type Err = Part2StateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tiles, instructions) = s.trim().split_once("\n\n").ok_or(MalformedState(s.to_string()))?;
        let tiles: HashMap<XY, Tile> = tiles
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim().chars().enumerate().map(move |(x, c)| {
                    ((x, y), c)
                })
            }).map(|((x, y), c)| {
                let (x1,y1) = (x * 2, y);
                let (x2,y2) = (x1 + 1, y1);

                match c { 
                    '@' => Ok(vec![((x1, y1), Tile::Robot), ((x2, y2), Tile::Empty)]),
                    '#' => Ok(vec![((x1, y1), Tile::Wall), ((x2, y2), Tile::Wall)]),
                    'O' => Ok(vec![((x1, y1), Tile::LeftBox), ((x2, y2), Tile::RightBox)]),
                    '.' => Ok(vec![((x1, y1), Tile::Empty), ((x2, y2), Tile::Empty)]),
                    _ => Err(Part2StateParseError::InvalidTile(c)),
                }
            }).collect::<Result<Vec<_>, Part2StateParseError>>()?
              .into_iter()
              .flatten()
              .collect();

        let instructions: Vec<char> = instructions.lines().flat_map(|line| line.trim().chars()).collect();

        let robot = tiles.iter().find_map(|(xy, c)| match c {
            Tile::Robot => Some(*xy),
            _ => None,
        }).ok_or(NoRobotFound(s.to_string()))?;



        Ok(Part2State{ tiles, robot, instructions })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Tile::*;

    #[test]
    fn test_score_example() {
        let input = "
            ##########
            ##...[]...
            ##.....@..
        ";
        let sut: Part2State = Part2State::from_raw(input);
        let result = sut.score();
        assert_eq!(result, 105);
    }

    #[test]
    fn test_score_example_small() {
        let input = "
            ####################
            ##[].......[].[][]##
            ##[]...........[].##
            ##[]........[][][]##
            ##[]......[]....[]##
            ##..##......[]....##
            ##..[]............##
            ##..@......[].[][]##
            ##......[][]..[]..##
            ####################
        ";
        let sut: Part2State = Part2State::from_raw(input);
        let result = sut.score();
        assert_eq!(result, 9021);
    }

    #[test]
    fn test_iter_base() {
        let sut = Part2State::from_raw("
            ..@.

            <<
        ");
        let result = sut.iter().next().unwrap();

        assert_eq!(result, Part2State::from_raw("
            .@..

            <
        "));
    }

    #[test]
    fn test_iter_wall() {
        let input = "
            #@

            <<
        ";
        let sut: Part2State = input.parse().unwrap();
        let result = sut.iter().next().unwrap();

        assert_eq!(result, Part2State::from_raw("
            ##@.

            <
        "));
    }

    #[test]
    fn test_iter_box() {
        let sut = Part2State::from_raw("
            ..[]@.

            <<
        ");
        let result = sut.iter().next().unwrap();

        assert_eq!(result, Part2State::from_raw("
            .[]@..

            <
        "), "\n\nstate:\n{}", result);
    }

    #[test]
    fn test_iter_box_up() {
        let sut = Part2State::from_raw("
            ..
            []
            @.

            ^v
        ");
        let result = sut.iter().next().unwrap();

        assert_eq!(result, Part2State::from_raw("
            []
            @.
            ..

            v
        "), "\n\nstate:\n{}", result);
    }

    #[test]
    fn test_iter_box_wall() {
        let sut = Part2State::from_raw("
            ##[]@.

            <<
        ");
        let result = sut.iter().next();

        assert_eq!(result, Some(Part2State::from_raw("
            ##[]@.

            <
        ")), "\n\nstate:\n{}", sut);
    }

    #[test]
    fn test_parse_base() {
        let input = "
            @

            <
        ";
        let result: Part2State = input.parse().unwrap();

        assert_eq!(result, Part2State{
            tiles: HashMap::from([ ((0,0), Robot), ((1, 0), Empty)]),
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
        let result: Part2State = input.parse().unwrap();

        assert_eq!(result, Part2State{
            tiles: HashMap::from([ ((0,0), Wall), ((1,0), Wall), ((2,0), Robot), ((3,0), Empty),
                                   ((0,1), Empty), ((1,1), Empty), ((2,1), LeftBox), ((3,1), RightBox)]),
            robot: (2,0),
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
        let result: Part2State = input.parse().unwrap();

        assert_eq!(result, Part2State{
            tiles: HashMap::from([ ((0,0), Robot), ((1,0), Empty) ]),
            robot: (0,0),
            instructions: vec!['<', '>', '^', 'v'],
        });
    }
}
