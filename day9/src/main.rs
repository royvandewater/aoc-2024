mod compact_disk_map;

use compact_disk_map::CompactDiskMap;
use std::{collections::VecDeque, fs::read_to_string, vec};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(parse_chunk)
        .collect::<CompactDiskMap>()
        .enumerate()
        .map(|(i, x)| i * x)
        .sum()
}

fn parse_chunk((id, c): (usize, &[usize])) -> Vec<Option<usize>> {
    let mut c: VecDeque<usize> = c.to_vec().into();
    let a = c.pop_front().unwrap();
    let used = vec![Some(id); a];

    match c.pop_front() {
        None => used,
        Some(b) => [used, vec![None; b]].concat(),
    }
}

#[cfg(test)]
mod test {
    use crate::compact_disk_map::disk_to_string;

    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "2333133121414131402";
        let result = part_1(&input);

        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_1_simple() {
        let input = "12345";
        let result = part_1(&input);
        // expanded: "0..111....22222"
        // compacted: "022111222"
        // checksum = 0*0 + 1*2 + 2*2 + 3*1 + 4*1 + 5*1 + 6*2 + 7*2 + 8*2

        assert_eq!(result, 60);
    }

    #[test]
    fn test_parse_chunk_12() {
        let result = parse_chunk((0, &[1, 2]));

        assert_eq!(result, vec![Some(0), None, None]);
    }

    #[test]
    fn test_parse_example() {
        let input = "2333133121414131402";
        let disk: Vec<Option<usize>> = input
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .chunks(2)
            .into_iter()
            .enumerate()
            .flat_map(parse_chunk)
            .collect();

        let disk_str = disk_to_string(&disk);
        assert_eq!(disk_str, "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn test_compact_disk_map_example() {
        let input = "2333133121414131402";
        let compacted: Vec<usize> = input
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .chunks(2)
            .into_iter()
            .enumerate()
            .flat_map(parse_chunk)
            .collect::<CompactDiskMap>()
            .collect();

        assert_eq!(
            compacted,
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6
            ]
        );
    }
}
