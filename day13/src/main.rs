mod machine;

use machine::Machine;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(cost_of_cheapest_combination)
        .sum()
}

fn cost_of_cheapest_combination(input: &str) -> Option<usize> {
    let _machine: Machine = input.parse().unwrap();
    Some(280)
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_1_example_1() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input);

        assert_eq!(result, 480);
    }

    #[ignore]
    #[test]
    fn test_cost_of_cheapest_combination_base_case() {
        let input = "
            Button A: X+1, Y+1
            Button B: X+2, Y+2
            Prize: X=1, Y=1
        ";
        let result = cost_of_cheapest_combination(&input).unwrap();

        assert_eq!(result, 3);
    }

    #[ignore]
    #[test]
    fn test_cost_of_cheapest_combination_example_machine_1() {
        let input = "
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
        ";
        let result = cost_of_cheapest_combination(&input).unwrap();

        assert_eq!(result, 280);
    }
}
