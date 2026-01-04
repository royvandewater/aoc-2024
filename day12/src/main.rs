mod region;

use region::Region;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    parse_to_regions(input).iter().map(|r| r.price()).sum()
}

fn parse_to_regions(input: &str) -> Vec<Region> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.trim().chars().enumerate().map(move |(x, c)| (c, (x, y))))
        .fold(vec![], |mut acc, cell| {
            match acc.iter_mut().find(|r| r.contains(&cell)) {
                Some(_) => acc,
                None => {
                    acc.push(Region::from((input, cell)));
                    acc
                }
            }
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input);

        assert_eq!(result, 772);
    }

    #[test]
    fn test_part_1_larger_example() {
        let input = read_to_string("./input_larger_example.txt").unwrap();
        let result = part_1(&input);

        assert_eq!(result, 1930);
    }

    #[test]
    fn test_parse_to_regions_example() {
        let input = "
            AAAA
            BBCD
            BBCC
            EEEC
        ";
        let regions = parse_to_regions(&input);

        assert_eq!(regions.len(), 5);
    }
}
