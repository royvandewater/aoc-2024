use rayon::prelude::*;
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let input = trim_lines(input);
    let guard = find_guard(&input).unwrap();
    let trail = recursively_step(&input, &HashSet::new(), &guard);

    let positions: HashSet<_> = trail.iter().map(|g| g.pos()).collect();
    positions.len()
}

fn part_2(input: &str) -> usize {
    let input = trim_lines(input);
    let guard = find_guard(&input).unwrap();
    let trail = recursively_step(&input, &HashSet::new(), &guard);

    let total = find_potential_obstacles(&input, &trail).len();
    println!("num_potential_obstacles: {}", total);

    find_potential_obstacles(&input, &trail)
        .par_iter()
        .filter(|pos| is_loop(&insert_obstacle(&input, pos), &HashSet::new(), &guard))
        .count()
}

fn is_loop(input: &str, trail: &HashSet<Guard>, guard: &Guard) -> bool {
    match step(input, &guard) {
        None => false,
        Some(next_guard) => match trail.contains(&next_guard) {
            true => true,
            false => {
                let mut next_trail = trail.clone();
                next_trail.insert(next_guard.clone());
                is_loop(&input, &next_trail, &next_guard)
            }
        },
    }
}

fn find_potential_obstacles(input: &str, trail: &HashSet<Guard>) -> HashSet<(usize, usize)> {
    trail
        .into_iter()
        .skip(1) // we have to skip the first position because the guard would see us place the obstacle
        .filter_map(|guard| {
            let pos = guard.advance()?.pos();

            match get_char_at_pos(input, &pos)? {
                '#' => None,
                _ => Some(pos),
            }
        })
        .collect()
}

fn insert_obstacle(input: &str, pos: &(usize, usize)) -> String {
    insert_char_at(input, pos, '#')
}

fn insert_char_at(input: &str, pos: &(usize, usize), n: char) -> String {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| match y == pos.1 {
            false => line.to_string(),
            true => line
                .chars()
                .enumerate()
                .map(|(x, c)| match x {
                    x if x == pos.0 => n,
                    _ => c,
                })
                .collect(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn trim_lines(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .join("\n")
}

fn recursively_step(input: &str, trail: &HashSet<Guard>, guard: &Guard) -> HashSet<Guard> {
    match step(input, &guard) {
        None => trail.clone(),
        Some(next_guard) => {
            let mut next_trail = trail.clone();
            next_trail.insert(next_guard.clone());
            recursively_step(&input, &next_trail, &next_guard)
        }
    }
}

// returns None when the guard has left the map
fn step(input: &str, guard: &Guard) -> Option<Guard> {
    let next_guard = guard.advance()?;
    let next_tile = get_char_at_pos(input, &next_guard.pos())?;

    match next_tile {
        '#' => step(input, &guard.rotate()),
        _ => Some(next_guard),
    }
}

fn get_char_at_pos(input: &str, (x, y): &(usize, usize)) -> Option<char> {
    input.lines().nth(*y)?.chars().nth(*x)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Guard {
    North((usize, usize)),
    East((usize, usize)),
    South((usize, usize)),
    West((usize, usize)),
}

impl Guard {
    fn rotate(&self) -> Guard {
        match self {
            Guard::North(p) => Guard::East(*p),
            Guard::East(p) => Guard::South(*p),
            Guard::South(p) => Guard::West(*p),
            Guard::West(p) => Guard::North(*p),
        }
    }

    fn pos(&self) -> (usize, usize) {
        match self {
            Guard::North(p) => *p,
            Guard::East(p) => *p,
            Guard::South(p) => *p,
            Guard::West(p) => *p,
        }
    }

    fn advance(&self) -> Option<Guard> {
        match self.clone() {
            Guard::North((_, y)) if y == 0 => return None,
            Guard::West((x, _)) if x == 0 => return None,
            _ => {}
        }

        Some(match self.clone() {
            Guard::North((x, y)) => Guard::North((x, y - 1)),
            Guard::East((x, y)) => Guard::East((x + 1, y)),
            Guard::South((x, y)) => Guard::South((x, y + 1)),
            Guard::West((x, y)) => Guard::West((x - 1, y)),
        })
    }
}

fn find_guard(input: &str) -> Option<Guard> {
    let pos = find_guard_position(input)?;
    let c = get_char_at_pos(input, &pos).unwrap();

    match c {
        '^' => Some(Guard::North(pos)),
        '>' => Some(Guard::West(pos)),
        'v' => Some(Guard::South(pos)),
        '<' => Some(Guard::East(pos)),
        _ => panic!("Unrecognized guard position: {} ({}, {})", c, pos.0, pos.1),
    }
}

fn find_guard_position(input: &str) -> Option<(usize, usize)> {
    input.lines().enumerate().find_map(|(y, line)| {
        line.chars().enumerate().find_map(|(x, tile)| match tile {
            '^' | '>' | 'v' | '<' => Some((x, y)),
            _ => None,
        })
    })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...";

        assert_eq!(part_1(&input), 41);
    }

    // #[ignore]
    #[test]
    fn test_part_2_example() {
        let input = "
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...";

        assert_eq!(part_2(&input), 6);
    }
}
