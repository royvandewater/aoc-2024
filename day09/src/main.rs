mod compact_disk_map;
mod defrag_disk_map;

use compact_disk_map::CompactDiskMap;
use defrag_disk_map::DiskMap;
use std::{collections::VecDeque, fs::read_to_string, vec};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
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
        .map(parse_chunk)
        .flat_map(expand_chunk)
        .collect::<CompactDiskMap>()
        .enumerate()
        .map(|(i, x)| i * x)
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .parse::<DiskMap>()
        .unwrap()
        .to_defragged()
        .enumerate()
        .map(|(i, x)| match x {
            Some(x) => i * x,
            None => 0,
        })
        .sum()
}

fn parse_chunk((id, c): (usize, &[usize])) -> (usize, usize, Option<usize>) {
    let mut c: VecDeque<usize> = c.to_vec().into();
    let used = c.pop_front().unwrap();
    let free = c.pop_front();

    (id, used, free)
}

fn expand_chunk((id, used, free): (usize, usize, Option<usize>)) -> Vec<Option<usize>> {
    match (id, used, free) {
        (id, used, None) => vec![Some(id); used],
        (id, used, Some(free)) => [vec![Some(id); used], vec![None; free]].concat(),
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
    fn test_part_2_example() {
        let input = "2333133121414131402";
        let result = part_2(&input);

        assert_eq!(result, 2858);
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

        assert_eq!(result, (0, 1, Some(2)));
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
            .map(parse_chunk)
            .flat_map(expand_chunk)
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
            .map(parse_chunk)
            .flat_map(expand_chunk)
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
