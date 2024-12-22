#![allow(unused)]

use glam::{IVec2, ivec2};

mod keypad;

pub use keypad::*;
use pathfinding::prelude::dijkstra;

pub struct DirPad {
    keys: [(IVec2, IVec2); 5],
    action: IVec2,
}

impl Default for DirPad {
    fn default() -> Self {
        #[rustfmt::skip]
        let keys = [
                                         (IVec2::NEG_Y, ivec2(1, 0)), (IVec2::MAX, ivec2(2, 0)),
            (IVec2::NEG_X, ivec2(0, 1)), (IVec2::Y, ivec2(1, 1)),     (IVec2::X, ivec2(2, 1)),
        ];

        let action = ivec2(2, 0);

        Self { keys, action }
    }
}

impl DirPad {
    fn at(&self, pos: IVec2) -> IVec2 {
        self.keys
            .iter()
            .find(|(_, num_pos)| *num_pos == pos)
            .map(|&(e, _)| e)
            .expect("invalid position")
    }

    fn positions(&self) -> [IVec2; 5] {
        self.keys.map(|(_, pos)| pos)
    }

    pub fn find(&self, path: Vec<(IVec2, keypad::Num)>) {
        // Parse the final sequence of moves
        let parsed_moves = path
            .iter()
            .map(|(_, num)| num.to_owned())
            .last()
            .expect("path should have some elements");

        for moves in parsed_moves.moves() {
            print_moves(moves);
        }
        println!();
    }
}

pub fn print_moves(moves: Vec<IVec2>) {
    for m in moves {
        match m {
            IVec2::X => print!(">"),
            IVec2::NEG_X => print!("<"),
            IVec2::Y => print!("v"),
            IVec2::NEG_Y => print!("^"),
            _ => panic!("unit vectors expected"),
        }
    }
    print!("A");
}

pub fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|e| e.replace("A", "").parse::<usize>().ok())
        .collect()
}
