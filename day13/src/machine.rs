mod button;
mod prize;

use button::{Button, ButtonParseError};
use itertools::Itertools;
use prize::Prize;
use std::str::FromStr;
use thiserror::Error;

use crate::machine::prize::PrizeParseError;

#[derive(Debug, Eq, PartialEq)]
pub struct Machine {
    pub(crate) a: Button,
    pub(crate) b: Button,
    pub(crate) prize: Prize,
}

#[derive(Debug, Error)]
pub enum MachineParseErr {
    #[error("Expected exactly 3 lines (not including whitespace), got {got:?}")]
    NotEnoughLines { got: usize },

    #[error("Failed to parse button")]
    ButtonParseErr(#[from] ButtonParseError),

    #[error("Failed to parse prize")]
    PrizeParseError(#[from] PrizeParseError),
}

impl FromStr for Machine {
    type Err = MachineParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a_str, b_str, p_str) = s
            .trim()
            .lines()
            .collect_tuple::<(&str, &str, &str)>()
            .ok_or(MachineParseErr::NotEnoughLines {
                got: s.trim().lines().count(),
            })?;

        Ok(Machine {
            a: a_str.parse()?,
            b: b_str.parse()?,
            prize: p_str.parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_string() {
        let sut = "".parse::<Machine>();
        assert!(sut.is_err());
    }

    #[test]
    fn test_parse_machine_1() {
        let sut = "
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
        "
        .parse::<Machine>()
        .unwrap();

        assert_eq!(
            sut,
            Machine {
                a: Button { x: 94, y: 34 },
                b: Button { x: 22, y: 67 },
                prize: Prize { x: 8400, y: 5400 },
            }
        )
    }
}
