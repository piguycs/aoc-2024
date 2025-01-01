#![allow(unused)]

mod dirpad;
mod numpad;

use std::sync::LazyLock;

use glam::{ivec2, IVec2};
use pathfinding::prelude::*;

use dirpad::*;
use numpad::*;

pub static NUMPAD: LazyLock<NumPad> = LazyLock::new(NumPad::default);
pub static DIRPAD: LazyLock<DirPad> = LazyLock::new(DirPad::default);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
    Action,
}

impl NumPad {
    pub fn at(&self, pos: IVec2) -> Option<char> {
        self.keypad
            .iter()
            .find(|&&(_, coord)| coord == pos)
            .map(|&(e, _)| e)
    }

    pub fn get_moves(&self, seq: &str) -> Vec<Dir> {
        seq.chars()
            .scan(self.action, |state, c| {
                let (moves, last_pos) = self.get_moves_c(c, *state);
                *state = last_pos;
                Some(moves)
            })
            .flatten()
            .collect()
    }

    pub fn get_moves_c(&self, c: char, start: IVec2) -> (Vec<Dir>, IVec2) {
        let mut moves: Vec<Dir> = vec![];

        let path = dijkstra(
            &start,
            |pos| {
                [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y]
                    .into_iter()
                    .filter_map(|dir| {
                        let pos = dir + pos;

                        if self.at(pos).is_some() {
                            Some((pos, 1))
                        } else {
                            None // position is outside the grid
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |&pos| self.at(pos) == Some(c),
        )
        .expect("no valid path found");

        let positions = &path.0;
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

        dbg!(c);
        dbg!(&moves);

        moves.push(Dir::Action);

        // infailable because there NEEDS to be a path to reach this point
        // which also implies that there NEEDS to be at least one element
        let last_pos = path.0.last().expect("infailable");

        (moves, *last_pos)
    }
}
