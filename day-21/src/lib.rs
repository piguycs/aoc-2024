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

impl DirPad {
    //
}
