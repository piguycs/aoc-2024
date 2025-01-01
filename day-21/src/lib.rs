#![allow(unused)]

mod dirpad;
mod numpad;

use std::{collections::HashMap, sync::LazyLock};

use glam::{ivec2, IVec2};
use pathfinding::prelude::*;

use dirpad::*;
use numpad::*;

pub static NUMPAD: LazyLock<NumPad> = LazyLock::new(NumPad::default);
pub static DIRPAD: LazyLock<DirPad> = LazyLock::new(DirPad::default);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
    Action,
}

impl Dir {
    pub const ALL: [Self; 5] = [Self::Left, Self::Right, Self::Up, Self::Down, Self::Action];
}

impl DirPad {
    pub fn at(&self, pos: IVec2) -> Option<Dir> {
        self.keypad
            .iter()
            .find(|&&(_, coord)| coord == pos)
            .map(|&(e, _)| e)
    }

    pub fn ta(&self, dir: Dir) -> IVec2 {
        self.keypad
            .iter()
            .find(|&&(curr, _)| curr == dir)
            .map(|&(_, e)| e)
            .unwrap()
    }

    pub fn build_memory(&self) -> HashMap<(Dir, Dir), Vec<Vec<Dir>>> {
        let mut mem = HashMap::new();

        let combo = itertools::iproduct!(Dir::ALL, Dir::ALL).collect::<Vec<_>>();

        for (start, end) in combo {
            let start_pos = self.ta(start);
            let end_pos = self.ta(end);

            let path = astar_bag_collect(
                &start_pos,
                |&pos| {
                    [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y]
                        .iter()
                        .filter_map(move |&d| {
                            let pos = pos + d;

                            self.at(pos).map(|_| (pos, 1))
                        })
                },
                |_| 0,
                |&dir| dir == end_pos,
            )
            .unwrap();

            mem.insert(
                (start, end),
                path.0.iter().map(|e| path_to_dir(e)).collect(),
            );
        }

        mem
    }

    pub fn traverse_seq(&self, seq: Vec<Dir>) -> Vec<Dir> {
        todo!()
    }
}

pub fn path_to_dir(positions: &[IVec2]) -> Vec<Dir> {
    let mut moves = vec![];

    for i in 0..positions.len() - 1 {
        let diff = positions[i + 1] - positions[i];
        let dir = match diff {
            d if d == IVec2::X => Dir::Right,
            d if d == IVec2::NEG_X => Dir::Left,
            d if d == IVec2::Y => Dir::Down,
            d if d == IVec2::NEG_Y => Dir::Up,
            _ => panic!("Invalid direction"),
        };
        moves.push(dir);
    }

    moves
}
