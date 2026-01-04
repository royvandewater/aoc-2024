use std::collections::HashSet;

type XY = (usize, usize);
type Cell = (char, XY);

#[derive(Debug)]
pub struct Region {
    plant: char,
    plots: HashSet<XY>,
}

impl Region {
    pub fn contains(&self, (plant, xy): &Cell) -> bool {
        self.plant == *plant && self.plots.contains(xy)
    }

    pub fn price(&self) -> usize {
        self.circumference() * self.area()
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn circumference(&self) -> usize {
        self.plots
            .iter()
            .map(|xy| {
                let num_neighbors_are_in_region = find_potential_neighbors(*xy)
                    .iter()
                    .filter(|neighbor| self.plots.contains(&neighbor))
                    .count();
                4 - num_neighbors_are_in_region
            })
            .sum()
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

    find_potential_neighbors(xy)
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

fn find_potential_neighbors(xy: XY) -> Vec<XY> {
    match xy {
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
}
