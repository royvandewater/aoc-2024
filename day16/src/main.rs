mod direction;
mod find_empty_spaces;
mod find_end;
mod find_start;
mod xy;

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
use thiserror::Error;

use crate::direction::Direction;
use crate::find_empty_spaces::find_empty_spaces;
use crate::find_end::find_end;
use crate::find_start::find_start;
use crate::xy::XY;

use Direction::*;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input).unwrap());
}

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
enum Day16Error {
    #[error("Could not find E in input")]
    NoEnd,

    #[error("Could not find S in input")]
    NoStart,

    #[error("Could not find a path from S to E")]
    NoPathToEnd,
}

use Day16Error::*;

fn part_1(input: &str) -> Result<usize, Day16Error> {
    let input = trim_lines(input);
    let start = find_start(&input).ok_or(NoStart)?;
    let end = find_end(&input).ok_or(NoEnd)?;
    let empty_spaces = find_empty_spaces(&input);
    let cheapest_paths = find_cheapest_paths(&empty_spaces, end);

    cheapest_paths
        .get(&(start, East))
        .ok_or(NoPathToEnd)
        .cloned()
}

fn trim_lines(s: &str) -> String {
    s.trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_cheapest_paths(empty_spaces: &HashSet<XY>, end: XY) -> HashMap<(XY, Direction), usize> {
    let mut cheapest_paths = HashMap::new();
    let mut unprocessed_spaces = BTreeSet::new();
    let neighbors = find_neighbors(empty_spaces, end);

    for direction in Direction::iter().cloned() {
        cheapest_paths.insert((end, direction), 0);
    }

    for neighbor in neighbors {
        let cost = compute_cost(&cheapest_paths, neighbor);
        unprocessed_spaces.insert((cost, neighbor));
    }

    while let Some((cost, space)) = unprocessed_spaces.pop_first() {
        if let Some(known_cost) = cheapest_paths.get(&space)
            && *known_cost <= cost
        {
            continue;
        }

        let (xy, d) = space;
        cheapest_paths.insert(space, cost);
        cheapest_paths
            .entry((xy, d.clockwise()))
            .or_insert(cost + 1000);
        cheapest_paths
            .entry((xy, d.counter_clockwise()))
            .or_insert(cost + 1000);
        cheapest_paths
            .entry((xy, d.clockwise().clockwise()))
            .or_insert(cost + 2000);

        for neighbor in find_neighbors(empty_spaces, space.0) {
            if cheapest_paths.contains_key(&neighbor) {
                continue;
            }

            let cost = compute_cost(&cheapest_paths, neighbor);
            unprocessed_spaces.insert((cost, neighbor));
        }
    }

    cheapest_paths
}

fn find_neighbors(empty_spaces: &HashSet<XY>, xy: XY) -> Vec<(XY, Direction)> {
    Direction::iter()
        .filter_map(|d| {
            let next = xy.apply_direction(d);
            let space = empty_spaces.get(&next)?;
            Some((*space, d.invert()))
        })
        .collect()
}

fn compute_cost(
    cheapest_paths: &HashMap<(XY, Direction), usize>,
    (xy, direction): (XY, Direction),
) -> usize {
    let mut costs = vec![];

    if let Some(cost) = cheapest_paths.get(&(xy, direction)) {
        costs.push(*cost);
    }

    let forward = xy.apply_direction(&direction);
    if let Some(cost) = cheapest_paths.get(&(forward, direction)) {
        costs.push(1 + cost);
    }

    let clockwise = xy.apply_direction(&direction.clockwise());
    if let Some(cost) = cheapest_paths.get(&(clockwise, direction.clockwise())) {
        costs.push(1001 + cost);
    }

    let counter_clockwise = xy.apply_direction(&direction.counter_clockwise());
    if let Some(cost) = cheapest_paths.get(&(counter_clockwise, direction.counter_clockwise())) {
        costs.push(1001 + cost);
    }

    let double_clockwise = xy.apply_direction(&direction.clockwise().clockwise());
    if let Some(cost) = cheapest_paths.get(&(double_clockwise, direction.clockwise().clockwise())) {
        costs.push(2001 + cost);
    }

    *costs.iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_simplest() {
        let input = "####
                     #SE#
                     ####";
        let result = part_1(input).unwrap(); // all it has to do is step forward

        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_1_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_1(&input).unwrap();

        assert_eq!(result, 7036);
    }

    #[test]
    fn test_part_1_example_2() {
        let input = read_to_string("./input_example_2.txt").unwrap();
        let result = part_1(&input).unwrap();

        assert_eq!(result, 11048);
    }
}
