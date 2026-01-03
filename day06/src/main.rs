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

// 1756 is too low
// 1797 is too high
fn part_2(input: &str) -> usize {
    find_obstacles(&trim_lines(input)).len()
}

fn is_loop(input: &str, trail: &HashSet<Guard>, guard: &Guard) -> bool {
    match step(input, &guard) {
        // we have stepped out of bounds, so no loop
        None => false,
        // we are in bounds, let's see if we've been here before
        Some(next_guard) => match trail.contains(&next_guard) {
            // We are repeating a previous step, so we must be in a loop
            true => true,
            // we are somewhere unique, advance until we loop or step out of bounds
            false => {
                let mut next_trail = trail.clone();
                next_trail.insert(next_guard.clone());
                is_loop(&input, &next_trail, &next_guard)
            }
        },
    }
}

#[allow(dead_code)]
fn loop_to_string(input: &str, trail: &HashSet<Guard>, guard: &Guard) -> String {
    let original_guard = find_guard(input).unwrap();

    let output = insert_char_at(input, &original_guard.pos(), '.');
    let output = trail.iter().fold(output.to_string(), |acc, g| {
        let existing = get_char_at_pos(&acc, &g.pos()).unwrap();

        let c = match (existing, g) {
            ('-', Guard::North(_) | Guard::South(_)) => '+',
            (_, Guard::North(_) | Guard::South(_)) => '|',
            ('|', Guard::East(_) | Guard::West(_)) => '+',
            (_, Guard::East(_) | Guard::West(_)) => '-',
        };
        insert_char_at(&acc, &g.pos(), c)
    });

    let c = match guard {
        Guard::North(_) => 'N',
        Guard::East(_) => 'E',
        Guard::South(_) => 'S',
        Guard::West(_) => 'W',
    };
    insert_char_at(&output, &guard.pos(), c)
}

// Returns every coordinate that:
//   * Is not directly in front of the guard at the start
//   * Is directly in front of the guard at some point on their route
//   * Is contained within the map
//   * Is not already an obstacle
//   * Causes the guard to deviate such that it repeats an exact step
//     that it performed in the past. This is defined as their current
//     direction and position is already present in the trail
fn find_obstacles(input: &str) -> HashSet<(usize, usize)> {
    let og_guard = find_guard(&input).unwrap();
    let trail = recursively_step(&input, &HashSet::new(), &og_guard);
    let in_front_of_guard = og_guard.advance().unwrap();

    let mut trail = trail.clone();
    trail.remove(&in_front_of_guard);
    trail
        .par_iter()
        .filter_map(|guard| {
            let next = guard.advance()?;
            let pos = next.pos();
            let tile = get_char_at_pos(input, &pos)?;

            if tile == '#' {
                return None;
            }

            match is_loop(&insert_obstacle(&input, &pos), &HashSet::new(), &og_guard) {
                false => None,
                true => Some(pos),
            }
        })
        .collect()
}

fn insert_obstacle(input: &str, pos: &(usize, usize)) -> String {
    insert_char_at(input, pos, '#')
}

fn insert_char_at(input: &str, pos: &(usize, usize), n: char) -> String {
    let (x, y) = pos;
    let row_length = input.lines().nth(0).unwrap().len() + 1; // plus 1 for the newline
    let mut output = input.to_string();
    let i = (y * row_length) + x;
    let range = i..i + 1;
    output.replace_range(range, &n.to_string());

    assert_ne!(input, output, "Input and output should not be the same");
    return output;
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
        '#' => Some(guard.rotate()),
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
        '>' => Some(Guard::East(pos)),
        'v' => Some(Guard::South(pos)),
        '<' => Some(Guard::West(pos)),
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

    #[test]
    fn test_insert_char_at_when_one_char() {
        let result = insert_char_at(".", &(0, 0), 'X');
        assert_eq!(result, "X");
    }

    #[test]
    fn test_insert_char_at_when_two_lines_char() {
        let input = trim_lines(
            "
            .
            .
        ",
        );
        let result = insert_char_at(&input, &(0, 0), 'X');
        assert_eq!(result, "X\n.");
        let result = insert_char_at(&input, &(0, 1), 'X');
        assert_eq!(result, ".\nX");
    }

    #[test]
    fn test_insert_char_at_when_two_chars() {
        let result = insert_char_at("..", &(0, 0), 'X');
        assert_eq!(result, "X.");
        let result = insert_char_at("..", &(1, 0), 'X');
        assert_eq!(result, ".X");
    }
}
