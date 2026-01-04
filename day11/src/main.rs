mod step;

use std::fs::read_to_string;
use step::step;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    let stones: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    let stones = (0..25).fold(stones, |acc, _| step(&acc));
    stones.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        let input = "125 17";
        let result = part_1(&input);

        assert_eq!(result, 55312);
    }
}
