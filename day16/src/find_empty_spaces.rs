use std::collections::HashSet;

use crate::xy::XY;

pub(crate) fn find_empty_spaces(input: &str) -> HashSet<XY> {
    input
        .trim()
        .lines()
        .enumerate()
        .filter(|(_, l)| *l != "#")
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter(|(_x, c)| *c != '#')
                .map(move |(x, _c)| XY::new(x, y))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_string() {
        let result = find_empty_spaces("");
        assert!(result.is_empty(), "Expected an empty set: {:?}", result)
    }

    #[test]
    fn test_one_empty_space() {
        let result = find_empty_spaces(".");
        assert_eq!(result, HashSet::from([XY::new(0, 0)]));
    }

    #[test]
    fn test_one_wall() {
        let result = find_empty_spaces("#");
        assert!(result.is_empty(), "Expected an empty set: {:?}", result)
    }

    #[test]
    fn test_the_end() {
        let result = find_empty_spaces("E");
        assert_eq!(result, HashSet::from([XY::new(0, 0)]));
    }

    #[test]
    fn test_ignores_leading_whitespace() {
        let result = find_empty_spaces(" E");
        assert_eq!(result, HashSet::from([XY::new(0, 0)]));
    }

    #[test]
    fn test_two_characters() {
        let result = find_empty_spaces(".E");
        assert_eq!(result, HashSet::from([XY::new(0, 0), XY::new(1, 0)]));
    }

    #[test]
    fn test_two_lines_with_leading_whitespace() {
        let result = find_empty_spaces(" .\n .");
        assert_eq!(result, HashSet::from([XY::new(0, 0), XY::new(0, 1)]));
    }
}
