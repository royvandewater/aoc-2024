use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let muls = extract_muls(input);
    muls.iter().fold(0, |acc, (a, b)| acc + (a * b))
}

fn part_2(input: &str) -> usize {
    extract_tokens(input)
        .iter()
        .fold((true, 0), |(enabled, total), token| match token {
            Token::Mul((a, b)) => (enabled, total + compute_mul(enabled, a, b)),
            Token::Do => (true, total),
            Token::Dont => (false, total),
        })
        .1
}

fn compute_mul(enabled: bool, a: &usize, b: &usize) -> usize {
    match enabled {
        true => a * b,
        false => 0,
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Mul((usize, usize)),
    Do,
    Dont,
}

const MUL_REGEX: &'static str = r"mul\(\d{1,3},\d{1,3}\)";
const DO_REGEX: &'static str = r"do\(\)";
const DONT_REGEX: &'static str = r"don't\(\)";

fn extract_tokens(input: &str) -> Vec<Token> {
    Regex::new(format!("{}|{}|{}", MUL_REGEX, DO_REGEX, DONT_REGEX).as_str())
        .unwrap()
        .find_iter(input)
        .map(|m| match m.as_str() {
            x if is_mul(x) => Token::Mul(parse_mul(x)),
            x if is_do(x) => Token::Do,
            x if is_dont(x) => Token::Dont,
            _ => panic!("Unrecognized token: {}", m.as_str()),
        })
        .collect()
}

fn is_mul(input: &str) -> bool {
    Regex::new(MUL_REGEX).unwrap().is_match(input)
}

fn is_do(input: &str) -> bool {
    Regex::new(DO_REGEX).unwrap().is_match(input)
}

fn is_dont(input: &str) -> bool {
    Regex::new(DONT_REGEX).unwrap().is_match(input)
}

fn extract_muls(input: &str) -> Vec<(usize, usize)> {
    Regex::new(MUL_REGEX)
        .unwrap()
        .find_iter(input)
        .map(|m| parse_mul(m.as_str()))
        .collect()
}

fn parse_mul(mul: &str) -> (usize, usize) {
    let (_, [a, b]) = Regex::new(r"(\d{0,3}),(\d{0,3})")
        .unwrap()
        .captures(mul)
        .unwrap()
        .extract();

    (a.parse().unwrap(), b.parse().unwrap())
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
    fn test_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(&input), 48);
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

    #[test]
    fn test_extract_tokens_when_empty() {
        let result = extract_tokens("");
        assert_eq!(result, vec![])
    }

    #[test]
    fn test_extract_tokens_when_just_mul() {
        let result = extract_tokens("mul(1,2)");
        assert_eq!(result, vec![Token::Mul((1, 2))])
    }

    #[test]
    fn test_extract_tokens_when_just_do() {
        let result = extract_tokens("do()");
        assert_eq!(result, vec![Token::Do])
    }

    #[test]
    fn test_extract_tokens_when_just_dont() {
        let result = extract_tokens("don't()");
        assert_eq!(result, vec![Token::Dont])
    }

    #[test]
    fn test_extract_tokens_when_mul_then_do() {
        let result = extract_tokens("mul(1,2)do()");
        assert_eq!(result, vec![Token::Mul((1, 2)), Token::Do])
    }
}
