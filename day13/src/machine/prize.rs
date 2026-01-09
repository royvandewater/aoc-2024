use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Prize {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[derive(Debug, Error)]
pub enum PrizeParseError {
    #[error("Parse called on an invalid prize string: `{0}`")]
    NotAPrizeString(String),

    #[error("Received an invalid coordinate: `{0}`. Whole line: `{1}`")]
    InvalidCoordinate(String, String),

    #[error("Received an invalid unsigned integer")]
    ParseIntErr(#[from] ParseIntError),
}

use PrizeParseError::*;

impl FromStr for Prize {
    type Err = PrizeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_string();
        if !s.starts_with("Prize") {
            return Err(NotAPrizeString(s.to_string()));
        }

        let (_, coords) = s.split_once(": ").ok_or(NotAPrizeString(s.to_string()))?;
        let (x, y) = coords
            .split(", ")
            .map(|coord_str| {
                coord_str
                    .split_once("=")
                    .ok_or(InvalidCoordinate(coord_str.to_string(), s.to_string()))
            })
            .collect::<Result<Vec<(&str, &str)>, _>>()?
            .into_iter()
            .map(|(_, val)| val.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?
            .into_iter()
            .collect_tuple()
            .ok_or(NotAPrizeString(s.to_string()))?;

        Ok(Prize { x, y })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_string() {
        let sut = "".parse::<Prize>();
        assert!(sut.is_err());
    }

    #[test]
    fn test_parse_machine_1() {
        let sut = "Prize: X=8400, Y=5400".parse::<Prize>().unwrap();

        assert_eq!(sut, Prize { x: 8400, y: 5400 },)
    }
}
