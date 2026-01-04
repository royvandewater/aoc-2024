use itertools::Itertools;

// (stone_digit, number_of_stones_it_appears_on)
type Stone = (usize, usize);

pub fn step(stones: Vec<Stone>) -> Vec<Stone> {
    stones
        .iter()
        .flat_map(|stone| step_stone(*stone))
        .into_group_map()
        .iter()
        .map(|(x, ns)| (*x, ns.iter().sum()))
        .collect()
}

fn step_stone(stone: Stone) -> Vec<Stone> {
    match stone {
        (0, n) => vec![(1, n)],
        (x, n) if has_even_digits(x) => split_stone(x, n),
        (x, n) => vec![(x * 2024, n)],
    }
}

fn has_even_digits(x: usize) -> bool {
    format!("{}", x).chars().count().is_multiple_of(2)
}

fn split_stone(x: usize, n: usize) -> Vec<Stone> {
    let stone_str = format!("{}", x);
    let mid = stone_str.chars().count() / 2;
    let (head, tail) = stone_str.split_at(mid);

    vec![(head.parse().unwrap(), n), (tail.parse().unwrap(), n)]
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_step_empty() {
        assert_eq!(step(vec![]), vec![]);
    }

    #[test]
    fn test_step_0() {
        assert_eq!(step(vec![(0, 1)]), vec![(1, 1)]);
    }

    #[test]
    fn test_step_0_with_n_2() {
        assert_eq!(step(vec![(0, 2)]), vec![(1, 2)]);
    }

    #[test]
    fn test_step_1() {
        assert_eq!(step(vec![(1, 1)]), vec![(2024, 1)]);
    }

    #[test]
    fn test_step_1_n_2() {
        assert_eq!(step(vec![(1, 2)]), vec![(2024, 2)]);
    }

    #[test]
    fn test_step_10() {
        let result: HashSet<Stone> = step(vec![(10, 1)]).iter().cloned().collect();
        assert_eq!(result, HashSet::from([(0, 1), (1, 1)]));
    }

    #[test]
    fn test_step_2_stones_with_0() {
        assert_eq!(step(vec![(0, 1), (0, 1)]), vec![(1, 2)]);
    }

    #[test]
    fn test_step_2_stones_with_1() {
        assert_eq!(step(vec![(1, 1), (1, 1)]), vec![(2024, 2)]);
    }

    #[test]
    fn test_step_2_stones_with_10() {
        let result: HashSet<Stone> = step(vec![(10, 1), (10, 1)]).iter().cloned().collect();
        assert_eq!(result, HashSet::from([(0, 2), (1, 2)]));
    }
}
