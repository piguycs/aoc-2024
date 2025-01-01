#![allow(unused)]

mod dirpad;
pub mod numpad;
mod numpad_new;

use std::{collections::HashMap, sync::LazyLock};

use glam::{ivec2, IVec2};
use itertools::Itertools;
use pathfinding::prelude::*;

use dirpad::*;
use numpad_new::*;

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

        let mem = &self.mem;
        let mt = &Vec::new();

        let vecs = seq
            .iter()
            .tuple_windows()
            .map(|(&start, &end)| mem.get(&(start, end)).unwrap().first().unwrap_or(mt))
            .collect_vec();

        //dbg!(&vecs);

        let lens: usize = vecs.iter().map(|e| e.len()).sum();
        //dbg!(lens + vecs.len());

        vecs.iter()
            .flat_map(|v| v.iter().chain([&Dir::Action]))
            .cloned()
            .collect()
    }
}
