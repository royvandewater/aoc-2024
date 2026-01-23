use std::str::FromStr;

use thiserror::Error;

use crate::computer::{Computer, ComputerParseError};

pub(crate) struct ReverseComputer {
    #[allow(unused)]
    program: Vec<usize>,
    #[allow(unused)]
    pointer: usize,
    #[allow(unused)]
    expected_output: Vec<usize>,
    #[allow(unused)]
    expected_b: usize,
    #[allow(unused)]
    expected_c: usize,
    #[allow(unused)]
    expected_pointer: usize,
}

#[allow(unused)]
struct PotentialState {
    a: Option<usize>,
    b: Option<usize>,
    c: Option<usize>,
    pointer: Option<usize>,
}

#[derive(Debug, Error)]
pub(crate) enum ReverseError {}

impl ReverseComputer {
    pub(crate) fn run(&self) -> Result<usize, ReverseError> {
        Ok(0)
    }
}

impl From<Computer> for ReverseComputer {
    fn from(computer: Computer) -> Self {
        let program = computer.program.clone();
        let expected_output = computer.program.clone();
        let pointer = computer.program.len() - 2;
        let expected_b = computer.b;
        let expected_c = computer.c;
        let expected_pointer = computer.pointer;

        ReverseComputer {
            program,
            expected_output,
            expected_b,
            expected_c,
            expected_pointer,
            pointer,
        }
    }
}

impl FromStr for ReverseComputer {
    type Err = ComputerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let computer: Computer = s.parse()?;
        Ok(computer.into())
    }
}

#[cfg(test)]
mod test {
    use ntest::timeout;

    use super::*;

    #[test]
    #[timeout(200)]
    fn test_hello_world() {
        let sut: ReverseComputer = "
          Register A: 0
          Register B: 0
          Register C: 0

          Program: 5,0
        "
        .parse()
        .unwrap();

        let a = sut.run().unwrap();
        assert_eq!(a, 0);
    }
}
