mod step;

use std::{fs::read_to_string, num::ParseIntError};
use step::step;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let stones = parse_stones(input).unwrap();
    let stones = (0..25).fold(stones, |acc, _| step(acc));
    stones.iter().map(|(_, n)| n).sum()
}

fn part_2(input: &str) -> usize {
    let stones = parse_stones(input).unwrap();
    let stones = (0..75).fold(stones, |acc, _| step(acc));
    stones.iter().map(|(_, n)| n).sum()
}

fn parse_stones(input: &str) -> Result<Vec<(usize, usize)>, ParseIntError> {
    Ok(input
        .trim()
        .split_whitespace()
        .map(|number| number.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?
        .into_iter()
        .map(|x| (x, 1))
        .collect::<Vec<(usize, usize)>>())
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
