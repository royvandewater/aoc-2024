// 0,3,5,4,3,0

// adv 3
// out 4
// jnz 0

const EXPECTED_OUTPUT: [usize; 6] = [0, 3, 5, 4, 3, 0];

#[allow(unused)]
pub(crate) fn run(a: usize) -> bool {
    let mut a = a;

    let mut output_index = 0;

    loop {
        // adv 3
        a >>= 3;

        // out 4
        if EXPECTED_OUTPUT[output_index] != a % 8 {
            return false;
        }
        output_index += 1;

        // jnz 0
        if a == 0 {
            return output_index == EXPECTED_OUTPUT.len();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_correct() {
        assert!(run(117440));
    }

    #[test]
    fn test_run_incorrect() {
        assert!(!run(0));
    }
}
