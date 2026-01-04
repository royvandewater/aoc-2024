use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    find_trailheads(input)
        .values()
        .map(unique_reachable_peaks_for_paths)
        .map(|reachable_peaks| reachable_peaks.len())
        .sum()
}

fn part_2(input: &str) -> usize {
    find_trailheads(input)
        .values()
        .map(|peaks| peaks.len())
        .sum()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct XY(usize, usize);

impl XY {
    fn neighbors(&self) -> Vec<XY> {
        match self {
            XY(0, 0) => vec![XY(1, 0), XY(0, 1)],
            XY(0, y) => vec![XY(0, y - 1), XY(0, y + 1), XY(1, *y)],
            XY(x, 0) => vec![XY(x - 1, 0), XY(x + 1, 0), XY(*x, 1)],
            XY(x, y) => vec![XY(x - 1, *y), XY(x + 1, *y), XY(*x, y - 1), XY(*x, y + 1)],
        }
    }
}

impl From<(usize, usize)> for XY {
    fn from((x, y): (usize, usize)) -> Self {
        XY(x, y)
    }
}

type Path = Vec<XY>;

fn group_squares_by_value(input: &str) -> HashMap<usize, Vec<XY>> {
    let squares_by_value: HashMap<usize, Vec<XY>> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, c)| {
                Some((c.to_string().parse::<usize>().ok()?, (x, y).into()))
            })
        })
        .into_group_map();
    squares_by_value
}

fn find_trailheads(input: &str) -> HashMap<XY, Vec<Vec<XY>>> {
    let squares_by_value = group_squares_by_value(input);
    let mut previous = initialize_peaks(squares_by_value.get(&9).unwrap());

    for value in (0..=8).rev() {
        let squares = squares_by_value.get(&value).unwrap();
        previous = find_paths(&previous, squares);
    }

    previous
}

fn unique_reachable_peaks_for_paths(paths: &Vec<Path>) -> HashSet<XY> {
    paths
        .iter()
        .map(|path| path.first().unwrap())
        .cloned()
        .collect()
}

fn find_paths(previous: &HashMap<XY, Vec<Path>>, squares: &Vec<XY>) -> HashMap<XY, Vec<Path>> {
    squares
        .iter()
        .map(|square| {
            (
                *square,
                square
                    .neighbors()
                    .iter()
                    .filter_map(|path| previous.get(path))
                    .flatten()
                    .map(|path| {
                        let mut path = path.clone();
                        path.push(square.clone());
                        path
                    })
                    .collect::<Vec<Path>>(),
            )
        })
        .collect()
}

fn initialize_peaks(coords: &Vec<XY>) -> HashMap<XY, Vec<Path>> {
    coords.iter().map(|xy| (*xy, vec![vec![*xy]])).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        ";
        let result = part_1(&input);

        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_2_example() {
        let input = "
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        ";
        let result = part_2(&input);

        assert_eq!(result, 81);
    }

    #[test]
    fn test_part_1_simple() {
        let input = "
            0123
            1234
            8765
            9876
        ";
        let result = part_1(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
        ";
        let result = part_1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_reachable_peaks() {
        let previous = HashMap::from([
            (XY(0, 6), vec![vec![XY(0, 6)]]),
            (XY(6, 6), vec![vec![XY(6, 6)]]),
        ]);

        let squares = vec![XY(0, 5), XY(6, 5)];

        let reachable = find_paths(&previous, &squares);

        assert_eq!(
            reachable,
            HashMap::from([
                (XY(0, 5), vec![vec![XY(0, 6), XY(0, 5)]]),
                (XY(6, 5), vec![vec![XY(6, 6), XY(6, 5)]]),
            ])
        )
    }
}
