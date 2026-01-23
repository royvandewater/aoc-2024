mod from_str;

use std::num::TryFromIntError;
use thiserror::Error;

pub(crate) use from_str::ComputerParseError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Computer {
    pub(crate) a: usize,
    pub(crate) b: usize,
    pub(crate) c: usize,
    pub(crate) pointer: usize,
    pub(crate) program: Vec<usize>,
    pub(crate) output: Vec<usize>,
}

#[derive(Debug, Error)]
pub(crate) enum RuntimeError {
    #[error("Ran out of instructions or combos")]
    NoMoreInstructions,

    #[error("Unrecognized instruction: {0}")]
    UnrecognizedInstruction(usize),

    #[error("Failed to convert usize to u32")]
    TryFromIntError(#[from] TryFromIntError),

    #[error("Unrecognized output combo: {0} % 8 = {1}")]
    UnrecognizedComboLiteral(usize, usize),

    #[error("We jumped, so don't advance the pointer")]
    JumpNoAdvancePointer,
}

use RuntimeError::*;

impl Computer {
    #[allow(dead_code)]
    pub(crate) fn new() -> Computer {
        Computer {
            a: 0,
            b: 0,
            c: 0,
            program: vec![],
            pointer: 0,
            output: vec![],
        }
    }

    pub(crate) fn output_as_string(&self) -> String {
        self.output
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(",")
    }

    #[allow(unused)]
    pub(crate) fn program_as_string(&self) -> String {
        self.program
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// initialize register :a to value
    pub(crate) fn initialize_a(&mut self, value: usize) {
        self.a = value
    }

    /// checks to see if the current output equals the program
    pub(crate) fn has_output_itself(&self) -> bool {
        self.output == self.program
    }

    pub(crate) fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            match self.step() {
                Ok(_) => self.pointer += 2,
                Err(JumpNoAdvancePointer) => {}
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
            match self.step() {
                Ok(_) => self.pointer += 2,
                Err(JumpNoAdvancePointer) => {}
                Err(NoMoreInstructions) => return Ok(()), // execution is done
                Err(e) => return Err(e),
            }

            if !self.output_is_matching_so_far() {
                return Ok(()); // abort
            }
        }
    }

    fn output_is_matching_so_far(&self) -> bool {
        self.output
            .iter()
            .enumerate()
            .all(|(i, x)| match self.program.get(i) {
                None => false,
                Some(y) => x == y,
            })
    }

    pub(crate) fn output_starts_with(&self, numbers: &Vec<usize>) -> bool {
        numbers
            .iter()
            .enumerate()
            .all(|(i, x)| match self.output.get(i) {
                None => false,
                Some(y) => x == y,
            })
    }

    fn step(&mut self) -> Result<(), RuntimeError> {
        let p_i = self.pointer;
        let p_c = self.pointer + 1;

        let instruction = *self.program.get(p_i).ok_or(NoMoreInstructions)?;
        let combo = *self.program.get(p_c).ok_or(NoMoreInstructions)?;

        match instruction {
            0 => self.adv(combo),
            1 => self.bxl(combo),
            2 => self.bst(combo),
            3 => self.jnz(combo),
            4 => self.bxc(),
            5 => self.out(combo),
            6 => self.bdv(combo),
            7 => self.cdv(combo),
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
    fn combo(&self, combo_literal: usize) -> Result<usize, RuntimeError> {
        match combo_literal % 8 {
            0 => Ok(0),
            1 => Ok(1),
            2 => Ok(2),
            3 => Ok(3),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err(UnrecognizedComboLiteral(combo_literal, combo_literal % 8)),
        }
    }

    /// divide :a by 2^combo, store in :a
    fn adv(&mut self, combo_literal: usize) -> Result<(), RuntimeError> {
        let combo = self.combo(combo_literal)?;
        self.a >>= combo;
        Ok(())
    }

    /// :b XOR literal, store in :b
    fn bxl(&mut self, literal: usize) -> Result<(), RuntimeError> {
        self.b ^= literal;
        Ok(())
    }

    /// combo % 8, store in :b
    fn bst(&mut self, combo_literal: usize) -> Result<(), RuntimeError> {
        let combo = self.combo(combo_literal)?;
        self.b = combo % 8;
        Ok(())
    }

    /// update pointer to literal if :a is non-zero.
    fn jnz(&mut self, literal: usize) -> Result<(), RuntimeError> {
        match self.a {
            0 => Ok(()),
            _ => {
                self.pointer = literal;
                Err(JumpNoAdvancePointer)
            }
        }
    }

    /// :b XOR :c, store in :b
    fn bxc(&mut self) -> Result<(), RuntimeError> {
        self.b ^= self.c; // bitwise XOR
        Ok(())
    }

    /// add combo to output
    fn out(&mut self, combo_literal: usize) -> Result<(), RuntimeError> {
        let combo = self.combo(combo_literal)? % 8;
        self.output.push(combo);
        Ok(())
    }

    /// divide :a by 2^combo, store in :b
    fn bdv(&mut self, combo_literal: usize) -> Result<(), RuntimeError> {
        let combo = self.combo(combo_literal)?;
        self.b = self.a >> combo;
        Ok(())
    }

    /// divide :a by 2^combo, store in :c
    fn cdv(&mut self, combo_literal: usize) -> Result<(), RuntimeError> {
        let combo = self.combo(combo_literal)?;
        self.c = self.a >> combo;
        Ok(())
    }
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

        assert_eq!(computer.output, vec![])
    }

    #[test]
    #[timeout(1000)]
    fn test_adv() {
        let mut computer = Computer::new();

        computer.a = 4; // set :a to 4
        computer.program = vec![0, 1]; // divide :a by 2^1, store in :a
        computer.run().unwrap(); // 4 / 2 = 2

        assert_eq!(computer.a, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_bxl() {
        let mut computer = Computer::new();

        computer.b = 4; // set :b to 4 (0b100)
        computer.program = vec![1, 2]; // bitwise XOR :b by 2 (0x010), store in :b
        computer.run().unwrap(); // 0b100 XOR 0b010 = 0b110

        assert_eq!(computer.b, 6)
    }

    #[test]
    #[timeout(1000)]
    fn test_bst() {
        let mut computer = Computer::new();

        computer.program = vec![2, 10]; // 10 % 8, store in :b
        computer.run().unwrap(); // 10 % 8 = 2

        assert_eq!(computer.b, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_jnz_when_0() {
        let mut computer = Computer::new();

        computer.a = 0;
        computer.program = vec![3, 3]; // does nothing because :a is 0
        computer.run().unwrap();

        assert_eq!(computer.a, 0)
    }

    #[test]
    #[timeout(1000)]
    fn test_jnz_when_1() {
        let mut computer = Computer::new();

        // because :a is non-zero, it skips to the index
        // at :a (4), causing it to jump over the first
        // output
        computer.a = 4;
        computer.program = vec![3, 4, 5, 4, 5, 4];
        computer.run().unwrap();

        assert_eq!(computer.output, vec![4])
    }

    #[test]
    #[timeout(1000)]
    fn test_bxc() {
        let mut computer = Computer::new();

        computer.b = 1; // 0b001
        computer.c = 3; // 0b011
        computer.program = vec![4, 0]; // :b XOR :c
        computer.run().unwrap(); // 0b001 XOR 0b011 = 0b010

        assert_eq!(computer.b, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_output_literal_0_through_3() {
        let mut computer = Computer::new();
        computer.program = vec![5, 0, 5, 1, 5, 2, 5, 3]; // print 0,1,2,3
        computer.run().unwrap();

        assert_eq!(computer.output, vec![0, 1, 2, 3])
    }

    #[test]
    #[timeout(1000)]
    fn test_output_register_a() {
        let mut computer = Computer::new();
        computer.a = 4;
        computer.program = vec![5, 4]; // print :a
        computer.run().unwrap(); // output 4.

        assert_eq!(computer.output, vec![4])
    }

    #[test]
    #[timeout(1000)]
    fn test_output_register_b() {
        let mut computer = Computer::new();
        computer.b = 4;
        computer.program = vec![5, 5]; // print :b
        computer.run().unwrap(); // output 4.

        assert_eq!(computer.output, vec![4])
    }

    #[test]
    #[timeout(1000)]
    fn test_output_register_c() {
        let mut computer = Computer::new();
        computer.c = 4;
        computer.program = vec![5, 6]; // print :b
        computer.run().unwrap(); // output 4.

        assert_eq!(computer.output, vec![4])
    }

    #[test]
    #[timeout(1000)]
    fn test_output_mods_8() {
        let mut computer = Computer::new();
        computer.a = 10;
        computer.program = vec![5, 4]; // print :a % 8
        computer.run().unwrap();

        assert_eq!(computer.output, vec![2])
    }

    #[test]
    #[timeout(1000)]
    fn test_bdv() {
        let mut computer = Computer::new();

        computer.a = 4; // set :a to 4
        computer.program = vec![6, 1]; // divide :a by 2^1, store in :b
        computer.run().unwrap(); // 4 / 2 = 2

        assert_eq!(computer.b, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_cdv() {
        let mut computer = Computer::new();

        computer.a = 4; // set :a to 4
        computer.program = vec![7, 1]; // divide :a by 2^1, store in :c
        computer.run().unwrap(); // 4 / 2 = 2

        assert_eq!(computer.c, 2)
    }

    #[test]
    #[timeout(1000)]
    fn test_adv_then_output() {
        let mut computer = Computer::new();
        computer.a = 4; // set :a to 4
        computer.program = vec![0, 1, 5, 4]; // divide :a by 2^1, then print :a
        computer.run().unwrap(); // 4 / 2 = 2, output 2.

        assert_eq!(computer.output, vec![2])
    }

    #[test]
    #[timeout(1000)]
    fn test_program_1() {
        let mut computer = Computer::new();
        computer.c = 9;
        computer.program = vec![2, 6];
        computer.run().unwrap();

        assert_eq!(computer.b, 1);
    }

    #[test]
    #[timeout(1000)]
    fn test_program_2() {
        let mut computer = Computer::new();
        computer.a = 10;
        computer.program = vec![5, 0, 5, 1, 5, 4];
        computer.run().unwrap();

        assert_eq!(computer.output, vec![0, 1, 2]);
    }

    #[test]
    #[timeout(1000)]
    fn test_program_3() {
        let mut computer = Computer::new();
        computer.a = 2024;
        computer.program = vec![0, 1, 5, 4, 3, 0];
        computer.run().unwrap();

        assert_eq!(computer.a, 0);
        assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    #[timeout(1000)]
    fn test_program_4() {
        let mut computer = Computer::new();
        computer.b = 29;
        computer.program = vec![1, 7];
        computer.run().unwrap();

        assert_eq!(computer.b, 26);
    }

    #[test]
    #[timeout(1000)]
    fn test_program_5() {
        let mut computer = Computer::new();
        computer.b = 2024;
        computer.c = 43690;
        computer.program = vec![4, 0];
        computer.run().unwrap();

        assert_eq!(computer.b, 44354);
    }

    #[test]
    #[timeout(1000)]
    fn test_example_equals_itself() {
        let input = read_to_string("./input_example_2.txt").unwrap();
        let mut computer: Computer = input.parse().unwrap();

        computer.a = 117440;
        computer.run().unwrap();

        assert_eq!(computer.output, computer.program);
    }
}
