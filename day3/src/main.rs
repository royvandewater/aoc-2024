use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("part_1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    let muls = extract_muls(input);
    muls.iter().fold(0, |acc, (a, b)| acc + (a * b))
}

fn extract_muls(input: &str) -> Vec<(usize, usize)> {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(input)
        .map(|c| c.extract::<2>())
        .map(|(_, mul)| parse_mul(mul))
        .collect()
}

fn parse_mul([a, b]: [&str; 2]) -> (usize, usize) {
    (
        usize::from_str_radix(a, 10).unwrap(),
        usize::from_str_radix(b, 10).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_1(&input), 161);
    }

    #[test]
    fn test_extract_muls_when_empty() {
        let result = extract_muls("");
        assert_eq!(result, vec![])
    }

    #[test]
    fn test_extract_muls_when_just_mul() {
        let result = extract_muls("mul(1,2)");
        assert_eq!(result, vec![(1, 2)])
    }
}
