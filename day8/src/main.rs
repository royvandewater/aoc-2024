mod parse_input;
mod part_1;
mod tuple_tools;

use part_1::part_1;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    println!("part_1: {}", part_1(&input));
}
