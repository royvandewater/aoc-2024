pub fn step(stones: &Vec<usize>) -> Vec<usize> {
    stones.iter().flat_map(step_stone).collect()
}

fn step_stone(stone: &usize) -> Vec<usize> {
    match stone {
        0 => vec![1],
        x if has_even_digits(stone) => split_stone(stone),
        x => vec![x * 2024],
    }
}

fn has_even_digits(stone: &usize) -> bool {
    format!("{}", stone).chars().count().is_multiple_of(2)
}

fn split_stone(stone: &usize) -> Vec<usize> {
    let stone_str = format!("{}", stone);
    let mid = stone_str.chars().count() / 2;
    let (head, tail) = stone_str.split_at(mid);

    vec![head.parse().unwrap(), tail.parse().unwrap()]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step_empty() {
        assert_eq!(step(&vec![]), vec![]);
    }

    #[test]
    fn test_step_0() {
        assert_eq!(step(&vec![0]), vec![1]);
    }
}
