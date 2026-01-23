mod from_str;
mod state;

use itertools::Itertools;
use memoize::memoize;
use state::State;
use std::num::TryFromIntError;
use thiserror::Error;

pub(crate) use from_str::ComputerParseError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Computer {
    state: State,
}

#[derive(Clone, Debug, Error)]
pub(crate) enum RuntimeError {
    #[error("Could not find instruction and argument for pointer")]
    NoMoreInstructions,

    #[error("Unrecognized instruction: {0}")]
    UnrecognizedInstruction(usize),

    #[error("Failed to convert usize to u32")]
    TryFromIntError(#[from] TryFromIntError),

    #[error("Unrecognized output combo: {0} % 8 = {1}")]
    UnrecognizedComboLiteral(usize, usize),
}

use RuntimeError::*;

impl Computer {
    #[allow(dead_code)]
    pub(crate) fn new() -> Computer {
        Computer {
            state: State {
                a: 0,
                b: 0,
                c: 0,
                program: vec![],
                pointer: 0,
                output: vec![],
            },
        }
    }

    pub(crate) fn output_as_string(&self) -> String {
        self.state
            .output
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(",")
    }

    #[allow(unused)]
    pub(crate) fn program_as_string(&self) -> String {
        self.state
            .program
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// initialize register :a to value
    pub(crate) fn initialize_a(&mut self, value: usize) {
        self.state = self.state.with_a(value);
    }

    /// checks to see if the current output equals the program
    pub(crate) fn has_output_itself(&self) -> bool {
        self.state.output == self.state.program
    }

    pub(crate) fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            match next_state(self.state.clone()) {
                Ok(state) => self.state = state,
                Err(NoMoreInstructions) => return Ok(()), // execution is done
                Err(e) => return Err(e),
            }
        }
    }

    /// runs as long as the output is a subset of the program instructions. Will
    /// early return as soon as the program outputs a value that doesn't match
    /// the program.
    pub(crate) fn run_for_matching_output(&mut self) -> Result<(), RuntimeError> {
        loop {
            match next_state(self.state.clone()) {
                Ok(state) => self.state = state,
                Err(NoMoreInstructions) => return Ok(()), // execution is done
                Err(e) => return Err(e),
            }

            if !self.output_is_matching_so_far() {
                return Ok(()); // abort
            }
        }
    }

    fn output_is_matching_so_far(&self) -> bool {
        self.state
            .output
            .iter()
            .enumerate()
            .all(|(i, x)| match self.state.program.get(i) {
                None => false,
                Some(y) => x == y,
            })
    }
}

#[memoize]
pub(crate) fn next_state(state: State) -> Result<State, RuntimeError> {
    let (instruction, arg) = state
        .program
        .iter()
        .skip(state.pointer)
        .take(2)
        .cloned()
        .collect_tuple()
        .ok_or(NoMoreInstructions)?;

    match instruction {
        0 => adv(state, arg),
        1 => bxl(state, arg),
        2 => bst(state, arg),
        3 => jnz(state, arg),
        4 => bxc(state),
        5 => out(state, arg),
        6 => bdv(state, arg),
        7 => cdv(state, arg),
        _ => Err(UnrecognizedInstruction(instruction)),
    }
}

/// combo will be treated as combo_literal % 8
/// if:
///  combo is 0-3, return combo
///  combo is 4, return :a
///  combo is 5, return :b
///  combo is 6, return :c
///  combo is 7, Reserved
///
fn combo(state: &State, combo_literal: usize) -> Result<usize, RuntimeError> {
    match combo_literal % 8 {
        0 => Ok(0),
        1 => Ok(1),
        2 => Ok(2),
        3 => Ok(3),
        4 => Ok(state.a),
        5 => Ok(state.b),
        6 => Ok(state.c),
        _ => Err(UnrecognizedComboLiteral(combo_literal, combo_literal % 8)),
    }
}

/// divide :a by 2^combo, store in :a
fn adv(state: State, combo_literal: usize) -> Result<State, RuntimeError> {
    let combo = combo(&state, combo_literal)?;
    Ok(state.with_a(state.a >> combo).advance_pointer(2))
}

/// :b XOR literal, store in :b
fn bxl(state: State, literal: usize) -> Result<State, RuntimeError> {
    Ok(state.with_b(state.b ^ literal).advance_pointer(2))
}

/// combo % 8, store in :b
fn bst(state: State, combo_literal: usize) -> Result<State, RuntimeError> {
    let combo = combo(&state, combo_literal)?;
    Ok(state.with_b(combo % 8).advance_pointer(2))
}

/// update pointer to literal if :a is non-zero.
fn jnz(state: State, literal: usize) -> Result<State, RuntimeError> {
    match state.a {
        0 => Ok(state.advance_pointer(2)),
        _ => Ok(state.with_pointer(literal)),
    }
}

/// :b XOR :c, store in :b
fn bxc(state: State) -> Result<State, RuntimeError> {
    Ok(state.with_b(state.b ^ state.c).advance_pointer(2))
}

/// add combo to output
fn out(state: State, combo_literal: usize) -> Result<State, RuntimeError> {
    let combo = combo(&state, combo_literal)?;
    Ok(state.append_output(combo % 8).advance_pointer(2))
}

/// divide :a by 2^combo, store in :b
fn bdv(state: State, combo_literal: usize) -> Result<State, RuntimeError> {
    let combo = combo(&state, combo_literal)?;
    Ok(state.with_b(state.a >> combo).advance_pointer(2))
}

/// divide :a by 2^combo, store in :c
fn cdv(state: State, combo_literal: usize) -> Result<State, RuntimeError> {
    let combo = combo(&state, combo_literal)?;
    Ok(state.with_c(state.a >> combo).advance_pointer(2))
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use ntest::timeout;

    use super::*;

    #[test]
    #[timeout(1000)]
    fn test_empty_instructions() {
        let mut computer = Computer::new();
        computer.run().unwrap();

        assert_eq!(computer.state.output, vec![])
    }

    #[test]
    #[timeout(1000)]
    fn test_adv() {
        let mut computer = Computer::new();

        computer.state.a = 4; // set :a to 4
        computer.state.program = vec![0, 1]; // divide :a by 2^1, store in :a
        computer.run().unwrap(); // 4 / 2 = 2

        assert_eq!(computer.state.a, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_bxl() {
        let mut computer = Computer::new();

        computer.state.b = 4; // set :b to 4 (0b100)
        computer.state.program = vec![1, 2]; // bitwise XOR :b by 2 (0x010), store in :b
        computer.run().unwrap(); // 0b100 XOR 0b010 = 0b110

        assert_eq!(computer.state.b, 6)
    }

    #[test]
    #[timeout(1000)]
    fn test_bst() {
        let mut computer = Computer::new();

        computer.state.program = vec![2, 10]; // 10 % 8, store in :b
        computer.run().unwrap(); // 10 % 8 = 2

        assert_eq!(computer.state.b, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_jnz_when_0() {
        let mut computer = Computer::new();

        computer.state.a = 0;
        computer.state.program = vec![3, 3]; // does nothing because :a is 0
        computer.run().unwrap();

        assert_eq!(computer.state.a, 0)
    }

    #[test]
    #[timeout(1000)]
    fn test_jnz_when_1() {
        let mut computer = Computer::new();

        // because :a is non-zero, it skips to the index
        // at :a (4), causing it to jump over the first
        // output
        computer.state.a = 4;
        computer.state.program = vec![3, 4, 5, 4, 5, 4];
        computer.run().unwrap();

        assert_eq!(computer.state.output, vec![4])
    }

    #[test]
    #[timeout(1000)]
    fn test_bxc() {
        let mut computer = Computer::new();

        computer.state.b = 1; // 0b001
        computer.state.c = 3; // 0b011
        computer.state.program = vec![4, 0]; // :b XOR :c
        computer.run().unwrap(); // 0b001 XOR 0b011 = 0b010

        assert_eq!(computer.state.b, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_output_literal_0_through_3() {
        let mut computer = Computer::new();
        computer.state.program = vec![5, 0, 5, 1, 5, 2, 5, 3]; // print 0,1,2,3
        computer.run().unwrap();

        assert_eq!(computer.state.output, vec![0, 1, 2, 3])
    }

    #[test]
    #[timeout(1000)]
    fn test_output_register_a() {
        let mut computer = Computer::new();
        computer.state.a = 4;
        computer.state.program = vec![5, 4]; // print :a
        computer.run().unwrap(); // output 4.

        assert_eq!(computer.state.output, vec![4])
    }

    #[test]
    #[timeout(1000)]
    fn test_output_register_b() {
        let mut computer = Computer::new();
        computer.state.b = 4;
        computer.state.program = vec![5, 5]; // print :b
        computer.run().unwrap(); // output 4.

        assert_eq!(computer.state.output, vec![4])
    }

    #[test]
    #[timeout(1000)]
    fn test_output_register_c() {
        let mut computer = Computer::new();
        computer.state.c = 4;
        computer.state.program = vec![5, 6]; // print :b
        computer.run().unwrap(); // output 4.

        assert_eq!(computer.state.output, vec![4])
    }

    #[test]
    #[timeout(1000)]
    fn test_output_mods_8() {
        let mut computer = Computer::new();
        computer.state.a = 10;
        computer.state.program = vec![5, 4]; // print :a % 8
        computer.run().unwrap();

        assert_eq!(computer.state.output, vec![2])
    }

    #[test]
    #[timeout(1000)]
    fn test_bdv() {
        let mut computer = Computer::new();

        computer.state.a = 4; // set :a to 4
        computer.state.program = vec![6, 1]; // divide :a by 2^1, store in :b
        computer.run().unwrap(); // 4 / 2 = 2

        assert_eq!(computer.state.b, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_cdv() {
        let mut computer = Computer::new();

        computer.state.a = 4; // set :a to 4
        computer.state.program = vec![7, 1]; // divide :a by 2^1, store in :c
        computer.run().unwrap(); // 4 / 2 = 2

        assert_eq!(computer.state.c, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_adv_then_output() {
        let mut computer = Computer::new();
        computer.state.a = 4; // set :a to 4
        computer.state.program = vec![0, 1, 5, 4]; // divide :a by 2^1, then print :a
        computer.run().unwrap(); // 4 / 2 = 2, output 2.

        assert_eq!(computer.state.output, vec![2])
    }

    #[test]
    #[timeout(1000)]
    fn test_program_1() {
        let mut computer = Computer::new();
        computer.state.c = 9;
        computer.state.program = vec![2, 6];
        computer.run().unwrap();

        assert_eq!(computer.state.b, 1);
    }

    #[test]
    #[timeout(1000)]
    fn test_program_2() {
        let mut computer = Computer::new();
        computer.state.a = 10;
        computer.state.program = vec![5, 0, 5, 1, 5, 4];
        computer.run().unwrap();

        assert_eq!(computer.state.output, vec![0, 1, 2]);
    }

    #[test]
    #[timeout(1000)]
    fn test_program_3() {
        let mut computer = Computer::new();
        computer.state.a = 2024;
        computer.state.program = vec![0, 1, 5, 4, 3, 0];
        computer.run().unwrap();

        assert_eq!(computer.state.a, 0);
        assert_eq!(computer.state.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    #[timeout(1000)]
    fn test_program_4() {
        let mut computer = Computer::new();
        computer.state.b = 29;
        computer.state.program = vec![1, 7];
        computer.run().unwrap();

        assert_eq!(computer.state.b, 26);
    }

    #[test]
    #[timeout(1000)]
    fn test_program_5() {
        let mut computer = Computer::new();
        computer.state.b = 2024;
        computer.state.c = 43690;
        computer.state.program = vec![4, 0];
        computer.run().unwrap();

        assert_eq!(computer.state.b, 44354);
    }

    #[test]
    #[timeout(1000)]
    fn test_example_equals_itself() {
        let input = read_to_string("./input_example_2.txt").unwrap();
        let mut computer: Computer = input.parse().unwrap();

        computer.state.a = 117440;
        computer.run().unwrap();

        assert_eq!(computer.state.output, computer.state.program);
    }
}
