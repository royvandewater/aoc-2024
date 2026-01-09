mod machine;

use itertools::Itertools;
use machine::Machine;
use std::{fs::read_to_string, ops::RangeInclusive};

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
    let machine: Machine = input.parse().unwrap();

    let range: RangeInclusive<usize> = 0..=100;

    let button_a_movements = range.clone().map(|i| (machine.a.x * i, machine.a.y * i));
    let button_b_movements = range.clone().map(|i| (machine.b.x * i, machine.b.y * i));
    let prize = machine.prize;

    button_a_movements
        .enumerate()
        .cartesian_product(button_b_movements.enumerate())
        .find_map(|((na, (xa, ya)), (nb, (xb, yb)))| {
            match (xa + xb, ya + yb) == (prize.x, prize.y) {
                true => Some((na * 3) + nb),
                false => None,
            }
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input);

        assert_eq!(result, 480);
    }

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
