mod machine;

use itertools::Itertools;
use machine::Machine;
use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(cost_of_cheapest_combination_within_100_moves)
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(cost_of_cheapest_combination_with_adjusted_coordinates)
        .sum()
}

fn cost_of_cheapest_combination_within_100_moves(input: &str) -> Option<usize> {
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

fn cost_of_cheapest_combination_with_adjusted_coordinates(input: &str) -> Option<usize> {
    let adjustment = 10000000000000;
    let machine: Machine = input.parse().unwrap();

    let (a_x, a_y) = machine.a.into();
    let (b_x, b_y) = machine.b.into();
    let (p_x, p_y) = (machine.prize + adjustment).into();

    let det = (a_x * b_y) - (a_y * b_x);
    if det == 0 {
        return None;
    }

    let a = b_y * p_x - b_x * p_y;
    let b = a_x * p_y - a_y * p_x;

    match (a % det, b % det) {
        (0, 0) => Some(usize::try_from((3 * (a / det)) + (b / det)).unwrap()),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input);

        assert_eq!(result, 480);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_2(&input);

        assert!(result > 480); // the puzzle doesn't actually give us the expected solution
    }

    #[test]
    fn test_cost_of_cheapest_combination_within_100_moves_base_case() {
        let input = "
            Button A: X+1, Y+1
            Button B: X+2, Y+2
            Prize: X=1, Y=1
        ";
        let result = cost_of_cheapest_combination_within_100_moves(&input).unwrap();

        assert_eq!(result, 3);
    }

    #[test]
    fn test_cost_of_cheapest_combination_within_100_moves_example_machine_1() {
        let input = "
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
        ";
        let result = cost_of_cheapest_combination_within_100_moves(&input).unwrap();

        assert_eq!(result, 280);
    }

    #[test]
    fn test_cost_of_cheapest_combination_with_adjusted_coordinates_machine_4() {
        let input = "
            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        ";
        let result = cost_of_cheapest_combination_with_adjusted_coordinates(&input);
        assert!(result.is_some());
    }
}
