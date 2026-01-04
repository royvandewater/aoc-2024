use itertools::Itertools;
use std::{collections::HashSet, fmt::Display};

type XY = (usize, usize);
type Cell = (char, XY);
type Wall = (XY, Direction);

#[derive(Debug)]
pub struct Region {
    plant: char,
    plots: HashSet<XY>,
}

impl Region {
    pub fn contains(&self, (plant, xy): &Cell) -> bool {
        self.plant == *plant && self.plots.contains(xy)
    }

    pub fn circumference_based_price(&self) -> usize {
        self.area() * self.circumference()
    }

    pub fn sides_based_price(&self) -> usize {
        self.area() * self.num_sides()
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn circumference(&self) -> usize {
        self.plots
            .iter()
            .map(|xy| {
                let num_neighbors_are_in_region = find_potential_neighbors(xy)
                    .iter()
                    .filter(|neighbor| self.plots.contains(&neighbor))
                    .count();
                4 - num_neighbors_are_in_region
            })
            .sum()
    }

    fn walls(&self) -> HashSet<Wall> {
        self.plots
            .iter()
            .flat_map(|xy| {
                find_potential_walls(xy)
                    .into_iter()
                    .filter(|w| self.valid_wall(w))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn num_sides(&self) -> usize {
        let walls = self.walls();
        let num_south_sides: usize = walls
            .iter()
            .filter(|(_, direction)| direction.is_south())
            .into_group_map_by(|((_x, y), _)| y)
            .values()
            .map(|ws| count_adjacent_groups(&ws.iter().map(|((x, _y), _d)| x).cloned().collect()))
            .sum();

        let num_north_sides: usize = walls
            .iter()
            .filter(|(_, direction)| direction.is_north())
            .into_group_map_by(|((_x, y), _)| y)
            .values()
            .map(|ws| count_adjacent_groups(&ws.iter().map(|((x, _y), _d)| x).cloned().collect()))
            .sum();

        let num_east_sides: usize = walls
            .iter()
            .filter(|(_, direction)| direction.is_east())
            .into_group_map_by(|((x, _y), _)| x)
            .values()
            .map(|ws| count_adjacent_groups(&ws.iter().map(|((_x, y), _d)| y).cloned().collect()))
            .sum();

        let num_west_sides: usize = walls
            .iter()
            .filter(|(_, direction)| direction.is_west())
            .into_group_map_by(|((x, _y), _)| x)
            .values()
            .map(|ws| count_adjacent_groups(&ws.iter().map(|((_x, y), _d)| y).cloned().collect()))
            .sum();

        num_north_sides + num_east_sides + num_south_sides + num_west_sides
    }

    fn valid_wall(&self, wall: &Wall) -> bool {
        match wall {
            ((0, _), East) => true,
            ((_, 0), North) => true,
            (xy, d) => !self.plots.contains(&d.advance(xy)),
        }
    }
}

// takes a sequence and returns the number of groups that are
// no more than 1 digit appart. For example: [1, 2, 4, 5] = 2 ([1,2] & [4,5])
fn count_adjacent_groups(numbers: &Vec<usize>) -> usize {
    let numbers = numbers.clone().into_iter().sorted();
    let num_numbers = numbers.clone().count();
    let num_adjacent = numbers.tuple_windows().filter(|(a, b)| b - a == 1).count();
    num_numbers - num_adjacent
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    // panics if stepping causes the coordinates to become negative
    fn advance(&self, (x, y): &XY) -> (usize, usize) {
        match self {
            North => (*x, y - 1),
            South => (*x, y + 1),
            East => (x - 1, *y),
            West => (x + 1, *y),
        }
    }

    fn is_south(&self) -> bool {
        *self == Direction::South
    }

    fn is_north(&self) -> bool {
        *self == Direction::North
    }

    fn is_east(&self) -> bool {
        *self == Direction::East
    }

    fn is_west(&self) -> bool {
        *self == Direction::West
    }
}

use Direction::*;

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => f.write_str("North"),
            Direction::South => f.write_str("South"),
            Direction::East => f.write_str("East"),
            Direction::West => f.write_str("West"),
        }
    }
}

impl From<(&str, Cell)> for Region {
    fn from((input, (plant, xy)): (&str, Cell)) -> Self {
        let plots = plots_for_region(input, plant, xy, &HashSet::new());
        assert!(
            plots.len() > 0,
            "Should be at least one for plant: {}{:?}",
            plant,
            xy
        );
        Region {
            plant,
            plots: plots_for_region(input, plant, xy, &HashSet::new()),
        }
    }
}

fn plots_for_region(input: &str, plant: char, xy: XY, plots: &HashSet<XY>) -> HashSet<XY> {
    if plots.contains(&xy) {
        return HashSet::new();
    }

    if plant_at_xy(input, xy).is_none() {
        return HashSet::new();
    }

    if !plant_at_xy_is_plant(input, xy, plant) {
        return HashSet::new();
    }

    find_potential_neighbors(&xy)
        .iter()
        .fold(HashSet::from([xy]), |acc, potential_neighbor| {
            acc.union(&plots_for_region(
                input,
                plant,
                *potential_neighbor,
                &plots.union(&acc).cloned().collect(),
            ))
            .cloned()
            .collect()
        })
}

fn find_potential_walls(xy: &XY) -> Vec<Wall> {
    vec![
        (*xy, Direction::North),
        (*xy, Direction::East),
        (*xy, Direction::South),
        (*xy, Direction::West),
    ]
}

fn find_potential_neighbors(xy: &XY) -> Vec<XY> {
    match *xy {
        (0, 0) => vec![(0, 1), (1, 0)],
        (0, y) => vec![(0, y - 1), (0, y + 1), (1, y)],
        (x, 0) => vec![(x - 1, 0), (x + 1, 0), (x, 1)],
        (x, y) => vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)],
    }
}

fn plant_at_xy_is_plant(input: &str, xy: XY, plant: char) -> bool {
    match plant_at_xy(input, xy) {
        None => false,
        Some(p) => p == plant,
    }
}

fn plant_at_xy(input: &str, (x, y): XY) -> Option<char> {
    input.trim().lines().nth(y)?.trim().chars().nth(x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plots_for_region_example() {
        let input = "
            AAAA
            BBCD
            BBCC
            EEEC
        ";
        let plots = plots_for_region(input, 'B', (0, 1), &HashSet::new());

        assert_eq!(plots, HashSet::from([(0, 1), (1, 1), (0, 2), (1, 2)]));
    }

    #[test]
    fn test_num_sides_when_empty() {
        let sut = Region {
            plant: 'A',
            plots: HashSet::new(),
        };
        let result = sut.num_sides();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_num_sides_when_one_plot() {
        let sut = Region {
            plant: 'A',
            plots: HashSet::from([(0, 0)]),
        };
        let result = sut.num_sides();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_num_sides_when_two_plots() {
        let sut = Region {
            plant: 'A',
            plots: HashSet::from([(0, 0), (0, 1)]),
        };
        let result = sut.num_sides();
        assert_eq!(result, 4);
    }

    #[ignore]
    #[test]
    fn test_num_sides_when_three_plots_in_l() {
        let sut = Region {
            plant: 'A',
            plots: HashSet::from([(0, 0), (0, 1), (1, 1)]),
        };
        let result = sut.num_sides();
        assert_eq!(result, 6);
    }
}
