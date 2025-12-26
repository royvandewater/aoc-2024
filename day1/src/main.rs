use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("part_1: {}", compute_part_1(&input));
    println!("part_2: {}", compute_part_2(&input));
}

fn compute_part_1(input: &str) -> i32 {
    let (mut column_a, mut column_b) = to_columns(input);

    column_a.sort();
    column_b.sort();

    return column_a
        .into_iter()
        .zip(column_b)
        .map(|(a, b)| (a - b).abs())
        .sum();
}

fn compute_part_2(input: &str) -> i32 {
    let (column_a, column_b) = to_columns(input);

    column_a
        .into_iter()
        .map(|a| a * i32::try_from(column_b.iter().filter(|&&b| a == b).count()).unwrap())
        .sum()
}

fn to_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .trim()
        .lines()
        .map(|s| s.trim().split_once(' ').unwrap())
        .map(parse_pair)
        .unzip()
}

fn parse_pair((a, b): (&str, &str)) -> (i32, i32) {
    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_part_1() {
        let input = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "
        .to_string();

        let result = compute_part_1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_example_1_part_2() {
        let input = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "
        .to_string();

        let result = compute_part_2(&input);
        assert_eq!(result, 31);
    }
}
