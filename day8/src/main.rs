mod parse_input;
mod part_1;
mod part_2;
mod tuple_tools;

use part_1::part_1;
use part_2::part_2;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}
