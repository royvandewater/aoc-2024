use core::panic;
use std::collections::{HashMap, VecDeque};

use super::Part2State;
use crate::part_2_state::tile::Tile;
use crate::part_2_state::instruction::Instruction;


use Instruction::*;
use Tile::*;

type XY = (usize, usize);

pub struct Iter {
    tiles: HashMap<XY, Tile>,
    robot: XY,
    instructions: VecDeque<Instruction>,

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
        matches!(self.tiles.get(xy), Some(c) if *c == Wall)
    }

    // returns true if the tile can be moved, will recursively
    // check boxes that are in the way
    fn can_move_tile(&self, xy: XY, instruction: Instruction) -> bool {
        if *self.tiles.get(&xy).unwrap() == Wall {
            return false;
        }

        let next_xy = apply_instruction(xy, instruction);

        match self.tiles.get(&next_xy).unwrap() {
            Wall => false,
            Robot => true,
            Empty => true,
            LeftBox => {
                match instruction {
                    E | W => self.can_move_tile(next_xy, instruction),
                    N | S => {
                        let neighbor = (next_xy.0 + 1, next_xy.1);
                        self.can_move_tile(next_xy, instruction) && self.can_move_tile(neighbor, instruction)
                    },
                }
            },
            RightBox => {
                match instruction {
                    E | W => self.can_move_tile(next_xy, instruction),
                    N | S => {
                        let neighbor = (next_xy.0 - 1, next_xy.1);
                        self.can_move_tile(next_xy, instruction) && self.can_move_tile(neighbor, instruction)
                    },
                }
            },
        }
    }

    // Will move boxes out of the way. Panics if called when the something
    // is preventing the tile from moving. It's the callers responsibility 
    // to check can_move_tile before calling move_tile. We can't do it inside
    // of move_tile because we'd end up recursively calling it a bunch of extra
    // times.
    fn move_tile(&mut self, xy: XY, instruction: Instruction) {
        let tile = *self.tiles.get(&xy).unwrap();

        if tile == Wall {
            panic!("Don't try to move the wall! {:?}", xy);
        }

        let next_xy = apply_instruction(xy, instruction);

        match self.tiles.get(&next_xy).unwrap() {
            Wall => panic!("Tried to move something into a wall! {:?}", xy),
            Empty | Robot => {
                self.tiles.insert(xy, Empty);
                self.tiles.insert(next_xy, tile);
            },
            LeftBox => {
                self.move_tile(next_xy, instruction);
                self.tiles.insert(xy, Empty);
                self.tiles.insert(next_xy, tile);

                match instruction {
                    N | S => {
                        let neighbor = (next_xy.0 + 1, next_xy.1);
                        self.move_tile(neighbor, instruction);
                    },
                    _ => {}
                };
            },
            RightBox => {
                self.move_tile(next_xy, instruction);
                self.tiles.insert(xy, Empty);
                self.tiles.insert(next_xy, tile);

                match instruction {
                    N | S => {
                        let neighbor = (next_xy.0 - 1, next_xy.1);
                        self.move_tile(neighbor, instruction);
                    },
                    _ => {},
                };
            },
        };
    }
}

fn apply_instruction(xy: XY, instruction: Instruction) -> XY {
    let (x, y) = xy;

    match instruction {
        N => (x, y - 1),
        S => (x, y + 1),
        E => (x + 1, y),
        W => (x - 1, y),
    }
}


impl Iterator for Iter {
    type Item = Part2State;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.pop_front()?;

        if self.can_move_tile(self.robot, instruction) {
            self.move_tile(self.robot, instruction);
            self.robot = apply_instruction(self.robot, instruction);
        }
        
        Some(Part2State {
            tiles: self.tiles.clone(),
            robot: self.robot,
            instructions: self.instructions.iter().cloned().collect(),
        })
    }
}
