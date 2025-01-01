#![allow(unused)]

mod dirpad;
mod numpad;

use std::{collections::HashMap, sync::LazyLock};

use glam::{ivec2, IVec2};
use itertools::Itertools;
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

    pub fn traverse_seq(&self, mut seq: Vec<Dir>) -> Vec<Dir> {
        seq.insert(0, Dir::Action); // initial position

        let mem = &self.mem; // HashMap: (start, end), Vec of direction
        let mt = &Vec::new();

        let vecs = seq
            .iter()
            .tuple_windows()
            .map(|(&start, &end)| mem.get(&(start, end)).unwrap().first().unwrap_or(mt))
            .collect_vec();

        vecs.iter()
            .flat_map(|v| v.iter().chain([&Dir::Action]))
            .cloned()
            .collect()
    }

    pub fn traverse_seq_all(&self, mut seq: Vec<Dir>) -> Vec<Vec<Dir>> {
        seq.insert(0, Dir::Action); // initial position

        let mem = &self.mem;

        let vecs = seq
            .iter()
            .tuple_windows()
            .map(|(&start, &end)| mem.get(&(start, end)).cloned().unwrap_or(vec![]))
            .collect_vec();

        let a = numpad::combine_directions(&vecs, 0, vec![]);
        dbg!(&a);

        a
    }
}
