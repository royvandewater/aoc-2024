use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    let input = trim_lines(input);
    let output = recursively_step(&input);
    output.chars().filter(|c| *c == 'X').count()
}

fn trim_lines(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .join("\n")
}

fn recursively_step(input: &str) -> String {
    match step(input) {
        None => input.to_string(),
        Some(next) => recursively_step(&next),
    }
}

fn step(input: &str) -> Option<String> {
    let pos = find_guard_position(input)?;
    let guard = get_char_at_pos(input, &pos)?;

    let next_pos = get_next_pos(input, guard, &pos);
    if next_pos.is_none() {
        return Some(insert_char_at(input, &pos, 'X'));
    }

    let next_pos = next_pos.unwrap();
    match get_char_at_pos(input, &next_pos).unwrap() {
        '#' => Some(rotate_guard(input, &pos, guard)),
        _ => {
            let guard_replaced = insert_char_at(input, &pos, 'X');
            let output = insert_char_at(&guard_replaced, &next_pos, guard);
            Some(output)
        }
    }
}

fn rotate_guard(input: &str, pos: &(usize, usize), guard: char) -> String {
    let next_guard = match guard {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Unrecognized guard rotation: {}", guard),
    };

    insert_char_at(input, pos, next_guard)
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

fn get_char_at_pos(input: &str, (x, y): &(usize, usize)) -> Option<char> {
    input.lines().nth(*y)?.chars().nth(*x)
}

fn get_next_pos(input: &str, guard: char, (x, y): &(usize, usize)) -> Option<(usize, usize)> {
    let (x0, y0) = (isize::try_from(*x).unwrap(), isize::try_from(*y).unwrap());

    let (x1, y1) = match guard {
        '^' => (x0, y0 - 1),
        '>' => (x0 + 1, y0),
        'v' => (x0, y0 + 1),
        '<' => (x0 - 1, y0),
        _ => panic!("Unrecognized guard character: {}", guard),
    };

    let max_y = isize::try_from(input.lines().count() - 1).unwrap();
    let max_x = isize::try_from(input.lines().nth(0).unwrap_or("").len() - 1).unwrap();

    if x1 < 0 || max_x < x1 || y1 < 0 || max_y < y1 {
        return None;
    }

    let (x1, y1) = (usize::try_from(x1).unwrap(), usize::try_from(y1).unwrap());
    return Some((x1, y1));
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
    fn test_get_next_pos() {
        let input = trim_lines(
            "
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...",
        );

        let result = get_next_pos(&input, '^', &(4, 6)).unwrap();
        assert_eq!(result, (4, 5));
    }
}
