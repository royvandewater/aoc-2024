pub fn is_solveable(target: usize, values: &Vec<usize>) -> bool {
    walk(target, 0, values)
}

fn walk(target: usize, acc: usize, values: &Vec<usize>) -> bool {
    if target < acc {
        return false;
    };

    match values.split_first() {
        None => target == acc,
        Some((value, rest)) => {
            let rest: Vec<_> = rest.into_iter().cloned().collect();
            let value = *value;

            false
                || walk(target, acc + value, &rest)
                || walk(target, acc * value, &rest)
                || walk(target, concat(acc, value), &rest)
        }
    }
}

fn concat(a: usize, b: usize) -> usize {
    let ab = format!("{}{}", a, b);
    ab.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solveable_empty() {
        let solveable = is_solveable(1, &vec![]);
        assert!(!solveable);
    }

    #[test]
    fn test_is_solveable_one_value() {
        let solveable = is_solveable(1, &vec![1]);
        assert!(solveable);
    }

    #[test]
    fn test_is_solveable_one_value_not_solveable() {
        let solveable = is_solveable(2, &vec![1]);
        assert!(!solveable);
    }

    #[test]
    fn test_is_solveable_two_values_need_multiplication() {
        let solveable = is_solveable(6, &vec![2, 3]);
        assert!(solveable);
    }

    #[test]
    fn test_is_solveable_two_values_need_concatination() {
        let solveable = is_solveable(23, &vec![2, 3]);
        assert!(solveable);
    }
}
