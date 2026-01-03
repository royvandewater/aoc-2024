use std::collections::HashSet;

use crate::parse_input::{dimensions, parse_input};
use crate::tuple_tools::{i_to_u, u_to_i};
use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let (width, height) = dimensions(&input);
    assert!(width > 0, "Width must be greater than 0");
    assert!(height > 0, "height must be greater than 0");

    let (max_x, max_y) = (width - 1, height - 1);

    parse_input(&input)
        .values()
        .flat_map(|antennas| find_antinodes_for_all_antennas((max_x, max_y), antennas))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

fn find_antinodes_for_all_antennas(
    (max_x, max_y): (usize, usize),
    antennas: &Vec<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    antennas
        .iter()
        .combinations(2)
        .flat_map(|pair| {
            let (a, b) = pair.iter().collect_tuple().unwrap();
            find_antinodes_for_pair((max_x, max_y), **a, **b)
        })
        .collect()
}

fn find_antinodes_for_pair(
    (max_x, max_y): (usize, usize),
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
) -> HashSet<(usize, usize)> {
    let (max_x, max_y) = u_to_i((max_x, max_y));
    let (x1, y1) = u_to_i((x1, y1));
    let (x2, y2) = u_to_i((x2, y2));

    let dx = x2 - x1;
    let dy = y2 - y1;

    HashSet::from([(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)])
        .into_iter()
        .filter(|(x, y)| 0 <= *x && 0 <= *y)
        .filter(|(x, y)| *x <= max_x && *y <= max_y)
        .map(|c| i_to_u(c))
        .collect()
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();

        assert_eq!(part_1(&input), 14)
    }

    #[test]
    fn test_find_antinodes_for_all_antennas_when_empty() {
        let result = find_antinodes_for_all_antennas((0, 0), &vec![]);
        assert_eq!(result, HashSet::new())
    }

    #[test]
    fn test_find_antinodes_for_all_antennas_when_one() {
        let result = find_antinodes_for_all_antennas((0, 0), &vec![(0, 0)]);
        assert_eq!(result, HashSet::new())
    }

    #[test]
    fn test_find_antinodes_for_all_antennas_when_two() {
        let result = find_antinodes_for_all_antennas((3, 3), &vec![(1, 1), (2, 2)]);
        assert_eq!(result, HashSet::from([(0, 0), (3, 3)]))
    }

    #[test]
    fn test_find_antinodes_for_all_antennas_when_three() {
        // .#.#
        // #aa#
        // .a..
        // ##..

        let result = find_antinodes_for_all_antennas((3, 3), &vec![(1, 1), (1, 2), (2, 1)]);
        let mut result: Vec<(usize, usize)> = result.iter().cloned().collect();
        result.sort();

        assert_eq!(result, vec![(0, 1), (0, 3), (1, 0), (1, 3), (3, 0), (3, 1)],)
    }

    #[test]
    fn test_find_antinodes_for_pair_one_apart() {
        let result = find_antinodes_for_pair((3, 3), (1, 1), (2, 2));

        assert_eq!(result, HashSet::from([(0, 0), (3, 3)]))
    }

    #[test]
    fn test_find_antinodes_for_pair_in_a_row() {
        let result = find_antinodes_for_pair((3, 3), (1, 1), (2, 1));

        assert_eq!(result, HashSet::from([(0, 1), (3, 1)]))
    }

    #[test]
    fn test_find_antinodes_for_pair_in_a_row_opposite_order() {
        let result = find_antinodes_for_pair((3, 3), (2, 1), (1, 1));

        assert_eq!(result, HashSet::from([(0, 1), (3, 1)]))
    }

    #[test]
    fn test_find_antinodes_for_pair_in_a_column() {
        let result = find_antinodes_for_pair((3, 3), (1, 1), (1, 2));

        assert_eq!(result, HashSet::from([(1, 0), (1, 3)]))
    }

    #[test]
    fn test_find_antinodes_for_pair_different_arrangement() {
        let result = find_antinodes_for_pair((3, 3), (1, 2), (2, 1));

        assert_eq!(result, HashSet::from([(3, 0), (0, 3)]))
    }

    #[test]
    fn test_find_antinodes_for_pair_different_arrangement_opposite_order() {
        let result = find_antinodes_for_pair((3, 3), (2, 1), (1, 2));

        assert_eq!(result, HashSet::from([(3, 0), (0, 3)]))
    }

    #[test]
    fn test_find_antinodes_for_pair_one_off_map() {
        let result = find_antinodes_for_pair((2, 2), (1, 1), (2, 2));

        assert_eq!(result, HashSet::from([(0, 0)]))
    }

    #[test]
    fn test_find_antinodes_for_pair_goes_negative() {
        let result = find_antinodes_for_pair((2, 2), (0, 0), (1, 1));

        assert_eq!(result, HashSet::from([(2, 2)]))
    }
}
