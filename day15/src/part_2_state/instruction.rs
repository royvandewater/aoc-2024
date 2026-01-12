use std::fmt::{Display, Write};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Instruction {
    N = b'^',
    S = b'v',
    W = b'<',
    E = b'>',
}

use Instruction::*;


impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl From<Instruction> for char {
    fn from(val: Instruction) -> Self  {
        (&val).into()
    }
}

impl From<&Instruction> for char {
    fn from(val: &Instruction) -> Self  {
        match val {
            N => '^',
            S => 'v',
            W => '<',
            E => '>',
        }
    }
}

#[derive(Debug, Error)]
pub enum InvalidInstructionError {
    #[error("Invalid char: '{0}'")]
    InvalidChar(char)
}

impl TryFrom<char> for Instruction {
    type Error = InvalidInstructionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(N),
            'v' => Ok(S),
            '<' => Ok(W),
            '>' => Ok(E),
            _ => Err(InvalidInstructionError::InvalidChar(value)),
        }
    }
}
