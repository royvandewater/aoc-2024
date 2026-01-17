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
    println!("part_2: {}", part_2(&input).unwrap());
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

fn part_2(input: &str) -> Result<usize, Day16Error> {
    let input = trim_lines(input);
    let start = find_start(&input).ok_or(NoStart)?;
    let end = find_end(&input).ok_or(NoEnd)?;
    let empty_spaces = find_empty_spaces(&input);
    let cheapest_paths = find_cheapest_paths(&empty_spaces, end);

    let cost = cheapest_paths
        .get(&(start, East))
        .ok_or(NoPathToEnd)
        .cloned()?;

    let route = walk(&cheapest_paths, (start, East), cost);

    Ok(route.len())
}

fn walk(
    cheapest_paths: &HashMap<(XY, Direction), usize>,
    space: (XY, Direction),
    max_cost: usize,
) -> HashSet<XY> {
    let (xy, direction) = space;

    // we now need to decide if we want to move forward, turn left, or turn right. We will go
    // with which every option exists in cheapest path and has the lowest cost
    let forward_direction = direction;
    let forward_space = (xy.apply_direction(&forward_direction), forward_direction);
    let forward_cost = cheapest_paths.get(&forward_space).map(|cost| cost + 1);

    let cw_direction = direction.clockwise();
    let cw_space = (xy, cw_direction);
    let cw_cost = cheapest_paths.get(&cw_space).map(|cost| cost + 1000);

    let ccw_direction = direction.counter_clockwise();
    let ccw_space = (xy, ccw_direction);
    let ccw_cost = cheapest_paths.get(&ccw_space).map(|cost| cost + 1000);

    let spaces_with_costs: Vec<_> = [
        (forward_space, forward_cost),
        (cw_space, cw_cost),
        (ccw_space, ccw_cost),
    ]
    .into_iter()
    .filter_map(|(s, c)| Some((s, c?)))
    .filter(|(_s, c)| *c <= max_cost)
    .collect();

    let min_cost = spaces_with_costs.iter().map(|(_s, c)| c).cloned().min();

    match min_cost {
        None => HashSet::new(),
        Some(0) => spaces_with_costs
            .iter()
            .filter(|(_s, c)| *c == 1)
            .flat_map(|(space, _)| HashSet::from([xy, space.0]))
            .collect(),
        Some(min_cost) => spaces_with_costs
            .iter()
            .filter(|(_s, c)| *c == min_cost)
            .flat_map(|(space, cost)| {
                let space = *space;
                let mut path = walk(cheapest_paths, space, *cost);
                path.insert(xy);
                path.insert(space.0);
                path
            })
            .collect(),
    }
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

    #[test]
    fn test_part_2_simplest() {
        let input = "####
                     #SE#
                     ####";
        let result = part_2(input).unwrap(); // all it has to do is step forward

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2_one_path() {
        // shortest is step forward, ccw, forward. cost: 1002
        //
        let input = "####
                     #.E#
                     #S.#
                     ####";
        let result = part_2(input).unwrap();

        assert_eq!(result, 3); // (2,2), (3,2), (3,1)
    }

    #[test]
    fn test_part_2_two_paths() {
        // going either way around the wall should have equal cost: 3004
        let input = "#####
                     #...#
                     #S#E#
                     #...#
                     #####";
        let result = part_2(input).unwrap();

        // route a: (2,1), (1,1), (2,1), (3,1), (3,2)
        // route b: (2,1), (1,3), (2,3), (3,3), (3,2)
        // first & last of the routes are the same
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2_example() {
        let input = read_to_string("./input_example.txt").unwrap();
        let result = part_2(&input).unwrap();

        assert_eq!(result, 45);
    }

    #[test]
    fn test_part_2_example_2() {
        let input = read_to_string("./input_example_2.txt").unwrap();
        let result = part_2(&input).unwrap();

        assert_eq!(result, 64);
    }
}
