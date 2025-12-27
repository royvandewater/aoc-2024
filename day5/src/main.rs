use std::error::Error;
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("part_1: {}", part_1(&input));
    // println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let (rules, updates) = parse_updates_and_rules(input).unwrap();

    updates
        .iter()
        .filter(|update| valid_update(&rules, update))
        .map(|update| middle_page(&update))
        .sum()
}

fn middle_page(update: &Vec<usize>) -> usize {
    let length = update.len();
    assert_eq!(length % 2, 1); // length should always be odd
    let i = length / 2;

    *update.iter().nth(i).unwrap()
}

fn parse_updates_and_rules(
    input: &str,
) -> Result<(Vec<(usize, usize)>, Vec<Vec<usize>>), Box<dyn Error>> {
    let (rules, updates) = input.split_once("\n\n").ok_or("Missing double newline")?;
    let rules = parse_rules(rules)?;
    let updates = parse_updates(updates)?;

    Ok((rules, updates))
}

fn parse_rules(rules: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    rules.trim().lines().map(parse_rule).collect()
}

fn parse_rule(rule: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let (a, b) = rule.trim().split_once("|").ok_or("Missing Pipe")?;

    Ok((a.parse()?, b.parse()?))
}

fn parse_updates(updates: &str) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    updates.trim().lines().map(parse_update).collect()
}

fn parse_update(update: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    update.trim().split(',').map(parse_int).collect()
}

fn parse_int(s: &str) -> Result<usize, Box<dyn Error>> {
    let i = s.parse::<usize>()?;
    Ok(i)
}

fn valid_update(rules: &Vec<(usize, usize)>, update: &Vec<usize>) -> bool {
    update.iter().enumerate().all(|(i, page)| {
        let preceding_pages: HashSet<usize> = update.iter().take(i).map(|p| p.clone()).collect();

        let invalid_pages: HashSet<usize> = rules
            .iter()
            .map(|rule| rule.clone())
            .filter_map(|(x, y)| match x == *page {
                true => Some(y),
                false => None,
            })
            .collect();

        invalid_pages.intersection(&preceding_pages).count() == 0
    })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_to_string("./example_input.txt").unwrap();
        assert_eq!(part_1(&input), 143);
    }

    #[test]
    fn test_valid_update_when_empty() {
        let result = valid_update(&vec![], &vec![]);
        assert!(result)
    }

    #[test]
    fn test_valid_update_when_one_rule_one_update() {
        let result = valid_update(&vec![(1, 2)], &vec![1, 2]);
        assert!(result)
    }

    #[test]
    fn test_valid_update_when_one_rule_one_invalid_update() {
        let result = valid_update(&vec![(1, 2)], &vec![2, 1]);
        assert!(!result)
    }

    #[test]
    fn test_valid_update_example_update_1() {
        let input = read_to_string("./example_input.txt").unwrap();
        let (rules, updates) = parse_updates_and_rules(&input).unwrap();
        let update = updates.iter().nth(0).unwrap();

        let result = valid_update(&rules, &update);
        assert!(result);
    }

    #[test]
    fn test_valid_update_example_update_4() {
        let input = read_to_string("./example_input.txt").unwrap();
        let (rules, updates) = parse_updates_and_rules(&input).unwrap();
        let update = updates.iter().nth(3).unwrap();

        let result = valid_update(&rules, &update);
        assert!(!result);
    }

    #[test]
    fn test_valid_update_example_update_5() {
        let input = read_to_string("./example_input.txt").unwrap();
        let (rules, updates) = parse_updates_and_rules(&input).unwrap();
        let update = updates.iter().nth(3).unwrap();

        let result = valid_update(&rules, &update);
        assert!(!result);
    }

    #[test]
    fn test_middle_page() {
        let result = middle_page(&vec![1, 2, 3]);
        assert_eq!(result, 2)
    }
}
