use std::collections::HashSet;

use crate::parse_input::{dimensions, parse_input};
use crate::tuple_tools::{i_to_u, u_to_i};
use itertools::Itertools;

pub fn part_2(input: &str) -> usize {
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

    let up: HashSet<(isize, isize)> = ResonantHarmonics {
        dimensions: (max_x, max_y),
        origin: (x1 - dx, y1 - dy),
        step: (dx, dy),
    }
    .collect();

    let down: HashSet<(isize, isize)> = ResonantHarmonics {
        dimensions: (max_x, max_y),
        origin: (x1 + dx, y1 + dy),
        step: (-dx, -dy),
    }
    .collect();

    up.union(&down).map(|c| i_to_u(*c)).collect()
}

struct ResonantHarmonics {
    dimensions: (isize, isize),
    origin: (isize, isize),
    step: (isize, isize),
}

impl Iterator for ResonantHarmonics {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let (max_x, max_y) = self.dimensions;
        let (x0, y0) = self.origin;
        let (dx, dy) = self.step;

        match (x0 + dx, y0 + dy) {
            (x1, y1) if 0 <= x1 && x1 <= max_x && 0 <= y1 && y1 <= max_y => {
                self.origin = (x1, y1);
                Some((x1, y1))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    #[ignore]
    #[test]
    fn test_part_2_example() {
        let input = read_to_string("./input_example.txt").unwrap();

        assert_eq!(part_2(&input), 34)
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
        assert_eq!(result, HashSet::from([(0, 0), (1, 1), (2, 2), (3, 3)]))
    }

    #[test]
    fn test_find_antinodes_for_pair_one_using_1_2_step() {
        // A.......
        // ..A.....
        // ....#...
        // ......#.
        let result = find_antinodes_for_pair((7, 3), (0, 0), (2, 1));

        assert_eq!(result, HashSet::from([(0, 0), (2, 1), (4, 2), (6, 3)]))
    }

    #[ignore]
    #[test]
    fn test_find_antinodes_for_pair_in_a_row() {
        let result = find_antinodes_for_pair((3, 3), (1, 1), (2, 1));

        assert_eq!(result, HashSet::from([(0, 1), (3, 1)]))
    }

    #[ignore]
    #[test]
    fn test_find_antinodes_for_pair_in_a_row_opposite_order() {
        let result = find_antinodes_for_pair((3, 3), (2, 1), (1, 1));

        assert_eq!(result, HashSet::from([(0, 1), (3, 1)]))
    }

    #[ignore]
    #[test]
    fn test_find_antinodes_for_pair_in_a_column() {
        let result = find_antinodes_for_pair((3, 3), (1, 1), (1, 2));

        assert_eq!(result, HashSet::from([(1, 0), (1, 3)]))
    }

    #[ignore]
    #[test]
    fn test_find_antinodes_for_pair_different_arrangement() {
        let result = find_antinodes_for_pair((3, 3), (1, 2), (2, 1));

        assert_eq!(result, HashSet::from([(3, 0), (0, 3)]))
    }

    #[ignore]
    #[test]
    fn test_find_antinodes_for_pair_different_arrangement_opposite_order() {
        let result = find_antinodes_for_pair((3, 3), (2, 1), (1, 2));

        assert_eq!(result, HashSet::from([(3, 0), (0, 3)]))
    }

    #[ignore]
    #[test]
    fn test_find_antinodes_for_pair_one_off_map() {
        let result = find_antinodes_for_pair((2, 2), (1, 1), (2, 2));

        assert_eq!(result, HashSet::from([(0, 0)]))
    }

    #[ignore]
    #[test]
    fn test_find_antinodes_for_pair_goes_negative() {
        let result = find_antinodes_for_pair((2, 2), (0, 0), (1, 1));

        assert_eq!(result, HashSet::from([(2, 2)]))
    }
}
