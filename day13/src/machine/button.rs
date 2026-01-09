use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Button {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Into<(isize, isize)> for Button {
    fn into(self) -> (isize, isize) {
        (
            isize::try_from(self.x).unwrap(),
            isize::try_from(self.y).unwrap(),
        )
    }
}

#[derive(Debug, Error)]
pub enum ButtonParseError {
    #[error("Parse called on an invalid button string: `{0}`")]
    NotAButtonString(String),

    #[error("Received an invalid coordinate: `{0}`. Whole line: `{1}`")]
    InvalidCoordinate(String, String),

    #[error("Received an invalid unsigned integer")]
    ParseIntErr(#[from] ParseIntError),
}

use ButtonParseError::*;

impl FromStr for Button {
    type Err = ButtonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_string();
        if !s.starts_with("Button") {
            return Err(NotAButtonString(s.to_string()));
        }

        let (_, coords) = s.split_once(": ").ok_or(NotAButtonString(s.to_string()))?;
        let (x, y) = coords
            .split(", ")
            .map(|coord_str| {
                coord_str
                    .split_once("+")
                    .ok_or(InvalidCoordinate(coord_str.to_string(), s.to_string()))
            })
            .collect::<Result<Vec<(&str, &str)>, _>>()?
            .into_iter()
            .map(|(_, val)| val.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?
            .into_iter()
            .collect_tuple()
            .ok_or(NotAButtonString(s.to_string()))?;

        Ok(Button { x, y })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_string() {
        let sut = "".parse::<Button>();
        assert!(sut.is_err());
    }

    #[test]
    fn test_parse_machine_1() {
        let sut = "Button A: X+94, Y+34".parse::<Button>().unwrap();

        assert_eq!(sut, Button { x: 94, y: 34 },)
    }
}
