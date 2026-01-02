use itertools::Itertools;
use std::collections::HashMap;

pub fn parse_input(input: &str) -> HashMap<char, Vec<(usize, usize)>> {
    trim_whitespace(input)
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, (x, y))))
        .filter(|(c, _)| *c != '.')
        .into_group_map()
}

// Returns the width & height of the map
pub fn dimensions(input: &str) -> (usize, usize) {
    let input = trim_whitespace(input);

    match input.lines().count() {
        0 => (0, 0),
        height => (input.lines().nth(0).unwrap().len(), height),
    }
}

// trims whitespace from the whole map and from each line
fn trim_whitespace(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join("\n")
}
