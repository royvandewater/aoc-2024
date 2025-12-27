use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect();

    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, char)| match char {
                    'X' => count_xmas(&lines, x.try_into().unwrap(), y.try_into().unwrap()),
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect();

    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| count_x_mas(&lines, x.try_into().unwrap(), y.try_into().unwrap()))
                .sum::<usize>()
        })
        .sum()
}

fn count_xmas(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    xmas_n(lines, x, y)
        + xmas_ne(lines, x, y)
        + xmas_e(lines, x, y)
        + xmas_se(lines, x, y)
        + xmas_s(lines, x, y)
        + xmas_sw(lines, x, y)
        + xmas_w(lines, x, y)
        + xmas_nw(lines, x, y)
}

fn count_x_mas(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    let c = character_at(lines, x, y);
    let nw = character_at(lines, x - 1, y - 1);
    let ne = character_at(lines, x + 1, y - 1);
    let sw = character_at(lines, x - 1, y + 1);
    let se = character_at(lines, x + 1, y + 1);

    match ((nw, c, se), (sw, c, ne)) {
        (('M', 'A', 'S'), ('M', 'A', 'S')) => 1,
        (('M', 'A', 'S'), ('S', 'A', 'M')) => 1,
        (('S', 'A', 'M'), ('M', 'A', 'S')) => 1,
        (('S', 'A', 'M'), ('S', 'A', 'M')) => 1,
        _ => 0,
    }
}

fn xmas_n(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    match_xmas(lines, [(x, y), (x, y - 1), (x, y - 2), (x, y - 3)])
}

fn xmas_ne(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    match_xmas(
        lines,
        [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)],
    )
}

fn xmas_e(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    match_xmas(lines, [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)])
}

fn xmas_se(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    match_xmas(
        lines,
        [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
    )
}

fn xmas_s(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    match_xmas(lines, [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)])
}

fn xmas_sw(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    match_xmas(
        lines,
        [(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)],
    )
}

fn xmas_w(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    match_xmas(lines, [(x, y), (x - 1, y), (x - 2, y), (x - 3, y)])
}

fn xmas_nw(lines: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    match_xmas(
        lines,
        [(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)],
    )
}

fn match_xmas(lines: &Vec<Vec<char>>, coords: [(isize, isize); 4]) -> usize {
    let first = character_at(lines, coords[0].0, coords[0].1);
    let second = character_at(lines, coords[1].0, coords[1].1);
    let third = character_at(lines, coords[2].0, coords[2].1);
    let fourth = character_at(lines, coords[3].0, coords[3].1);

    let combined = format!("{}{}{}{}", first, second, third, fourth);

    match combined.as_str() {
        "XMAS" => 1,
        _ => 0,
    }
}

fn character_at(lines: &Vec<Vec<char>>, x: isize, y: isize) -> char {
    if x < 0 || y < 0 {
        return '.';
    }

    let x: usize = x.try_into().unwrap();
    let y: usize = y.try_into().unwrap();

    *lines
        .iter()
        .nth(y)
        .unwrap_or(&vec![])
        .iter()
        .nth(x)
        .unwrap_or(&'.')
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "
            ....XXMAS.
            .SAMXMS...
            ...S..A...
            ..A.A.MS.X
            XMASAMX.MM
            X.....XA.A
            S.S.S.S.SS
            .A.A.A.A.A
            ..M.M.M.MM
            .X.X.XMASX";
        assert_eq!(part_1(&input), 18);
    }

    #[test]
    fn test_part_2() {
        let input = "
            .M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........";
        assert_eq!(part_2(&input), 9);
    }

    #[test]
    fn test_part_1_when_empty() {
        let input = "";
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn test_part_1_when_the_word() {
        let input = "XMAS";
        assert_eq!(part_1(&input), 1);
    }

    #[test]
    fn test_part_1_when_not_the_word() {
        let input = "XMA.";
        assert_eq!(part_1(&input), 0);
    }
}
