use core::panic;
use std::collections::{HashMap, VecDeque};

use super::State;

type XY = (usize, usize);

pub struct Iter {
    tiles: HashMap<XY, char>,
    robot: XY,
    instructions: VecDeque<char>,

}

impl Iter {
    pub fn new(state: &State) -> Self {

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

    fn try_move_tile(&mut self, xy: XY, instruction: char) -> Option<XY> {
        let (x, y) = xy;
        let tile = *self.tiles.get(&xy).unwrap();

        if tile == '#' {
            return None;
        }

        let next_xy = match instruction {
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            _ => panic!("Unrecognized instruction: {}", instruction),
        };

        match self.tiles.get(&next_xy).unwrap() {
            '#' => None,
            'O' => {
                self.try_move_tile(next_xy, instruction)?;
                self.tiles.insert(xy, '.');
                self.tiles.insert(next_xy, tile);
                Some(next_xy)
            },
            '.' => {
                self.tiles.insert(xy, '.');
                self.tiles.insert(next_xy, tile);
                Some(next_xy)
            },
            c => panic!("Unrecognized tile: {}", c),
        }
    }
}


impl Iterator for Iter {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.pop_front()?;

        if let Some(xy) = self.try_move_tile(self.robot, instruction) { 
            self.robot = xy 
        }
        
        Some(State {
            tiles: self.tiles.clone(),
            robot: self.robot,
            instructions: self.instructions.iter().cloned().collect(),
        })
    }
}
