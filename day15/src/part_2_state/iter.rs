use core::panic;
use std::collections::{HashMap, VecDeque};

use super::Part2State;

type XY = (usize, usize);

pub struct Iter {
    tiles: HashMap<XY, char>,
    robot: XY,
    instructions: VecDeque<char>,

}

impl Iter {
    pub fn new(state: &Part2State) -> Self {

        Iter{ 
            tiles: state.tiles.clone(), 
            robot: state.robot, 
            instructions: state.instructions.iter().cloned().collect(),
        }
    }

    #[allow(unused)]
    fn is_wall(&self, xy: &XY) -> bool {
        matches!(self.tiles.get(xy), Some(c) if *c == '#')
    }

    // returns true if the tile can be moved, will recursively
    // check boxes that are in the way
    fn can_move_tile(&self, xy: XY, instruction: char) -> bool {
        if *self.tiles.get(&xy).unwrap() == '#' {
            return false;
        }

        let next_xy = apply_instruction(xy, instruction);

        match self.tiles.get(&next_xy).unwrap() {
            '#' => false,
            '@' => true,
            '.' => true,
            '[' => {
                match instruction {
                    '<' | '>' => self.can_move_tile(next_xy, instruction),
                    '^' | 'v' => {
                        let neighbor = (next_xy.0 + 1, next_xy.1);
                        self.can_move_tile(next_xy, instruction) && self.can_move_tile(neighbor, instruction)
                    },
                    _ => panic!("Unrecognized instruction: {}", instruction),
                }
            },
            ']' => {
                match instruction {
                    '<' | '>' => self.can_move_tile(next_xy, instruction),
                    '^' | 'v' => {
                        let neighbor = (next_xy.0 - 1, next_xy.1);
                        self.can_move_tile(next_xy, instruction) && self.can_move_tile(neighbor, instruction)
                    },
                    _ => panic!("Unrecognized instruction: {}", instruction),
                }
            },
            c => panic!("Unrecognized tile: {}", c),
        }
    }

    // Will move boxes out of the way. Panics if called when the something
    // is preventing the tile from moving.
    fn move_tile(&mut self, xy: XY, instruction: char) {
        let tile = *self.tiles.get(&xy).unwrap();

        if tile == '#' {
            panic!("Don't try to move the wall! {:?}", xy);
        }

        let next_xy = apply_instruction(xy, instruction);

        match self.tiles.get(&next_xy).unwrap() {
            '#' => panic!("Tried to move something into a wall! {:?}", xy),
            '[' => {
                self.move_tile(next_xy, instruction);
                self.tiles.insert(xy, '.');
                self.tiles.insert(next_xy, tile);

                match instruction {
                    '<' | '>' => {},
                    '^' | 'v' => {
                        let neighbor = (next_xy.0 + 1, next_xy.1);
                        self.move_tile(neighbor, instruction);
                    },
                    _ => panic!("Unrecognized instruction: {}", instruction),
                };
            },
            ']' => {
                self.move_tile(next_xy, instruction);
                self.tiles.insert(xy, '.');
                self.tiles.insert(next_xy, tile);

                match instruction {
                    '<' | '>' => {},
                    '^' | 'v' => {
                        let neighbor = (next_xy.0 - 1, next_xy.1);
                        self.move_tile(neighbor, instruction);
                    },
                    _ => panic!("Unrecognized instruction: {}", instruction),
                };
            },
            '.' => {
                self.tiles.insert(xy, '.');
                self.tiles.insert(next_xy, tile);
            },
            c => panic!("Unrecognized tile: {}", c),
        };
    }
}

fn apply_instruction(xy: XY, instruction: char) -> XY {
    let (x, y) = xy;

    match instruction {
        '<' => (x - 1, y),
        '>' => (x + 1, y),
        '^' => (x, y - 1),
        'v' => (x, y + 1),
        _ => panic!("Unrecognized instruction: {}", instruction),
    }
}


impl Iterator for Iter {
    type Item = Part2State;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.pop_front()?;

        if self.can_move_tile(self.robot, instruction) {
            self.move_tile(self.robot, instruction);
            self.robot = apply_instruction(self.robot, instruction)
        }
        
        Some(Part2State {
            tiles: self.tiles.clone(),
            robot: self.robot,
            instructions: self.instructions.iter().cloned().collect(),
        })
    }
}
