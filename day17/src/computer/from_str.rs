use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

use crate::computer::state::State;

use super::Computer;

#[derive(Debug, Error)]
pub(crate) enum ComputerParseError {
    #[error("No double newline found")]
    NoDoubleNewline,

    #[error("Invalid register: {0}")]
    InvalidRegister(String),

    #[error("Parse Int Error")]
    ParseIntError(#[from] ParseIntError),

    #[error("Wrong Number of Registers")]
    WrongNumberOfRegisters,
}

use ComputerParseError::*;

impl FromStr for Computer {
    type Err = ComputerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = trim_lines(s);
        let (registers, program) = s.split_once("\n\n").ok_or(NoDoubleNewline)?;

        let (a, b, c) = registers
            .lines()
            .flat_map(|l| l.split_once(": ").ok_or(InvalidRegister(l.to_string())))
            .flat_map(|(_, r)| r.parse::<usize>())
            .collect_tuple()
            .ok_or(WrongNumberOfRegisters)?;

        let program: Vec<usize> = program
            .replace("Program:", "")
            .trim()
            .split(",")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<usize>())
            .try_collect()?;

        Ok(Computer {
            state: State {
                a,
                b,
                c,
                program,
                pointer: 0,
                output: vec![],
            },
        })
    }
}

fn trim_lines(s: &str) -> String {
    s.trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_instructions() {
        let input = "
            Register A: 0
            Register B: 0
            Register C: 0

            Program:
        ";

        assert_eq!(
            input.parse::<Computer>().unwrap(),
            Computer {
                state: State {
                    a: 0,
                    b: 0,
                    c: 0,
                    program: vec![],
                    pointer: 0,
                    output: vec![],
                }
            }
        );
    }

    #[test]
    fn test_parse_register_values() {
        let input = "
            Register A: 1
            Register B: 2
            Register C: 3

            Program:
        ";

        assert_eq!(
            input.parse::<Computer>().unwrap(),
            Computer {
                state: State {
                    a: 1,
                    b: 2,
                    c: 3,
                    program: vec![],
                    pointer: 0,
                    output: vec![],
                }
            }
        );
    }

    #[test]
    fn test_parse_with_instructions() {
        let input = "
            Register A: 0
            Register B: 0
            Register C: 0

            Program: 1,2,3,4,5
        ";

        assert_eq!(
            input.parse::<Computer>().unwrap(),
            Computer {
                state: State {
                    a: 0,
                    b: 0,
                    c: 0,
                    program: vec![1, 2, 3, 4, 5],
                    pointer: 0,
                    output: vec![],
                }
            }
        );
    }
}
