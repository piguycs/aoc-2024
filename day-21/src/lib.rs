mod keypad;

use std::fmt::Display;

use glam::{ivec2, IVec2};
use itertools::Itertools;
use pathfinding::prelude::*;

pub use keypad::*;

pub fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|e| e.replace("A", "").parse::<usize>().ok())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Action,
}

impl From<IVec2> for Key {
    fn from(value: IVec2) -> Self {
        match value {
            IVec2::NEG_X => Self::Left,
            IVec2::X => Self::Right,
            IVec2::NEG_Y => Self::Up,
            IVec2::Y => Self::Down,
            _ => panic!("unit vectors expected"),
        }
    }
}

impl From<Key> for IVec2 {
    fn from(value: Key) -> Self {
        match value {
            Key::Left => IVec2::NEG_X,
            Key::Right => IVec2::X,
            Key::Up => IVec2::NEG_Y,
            Key::Down => IVec2::Y,
            Key::Action => IVec2::ZERO,
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Key::Left => "<",
            Key::Right => ">",
            Key::Up => "^",
            Key::Down => "v",
            Key::Action => "A",
        };

        write!(f, "{txt}")
    }
}

#[derive(Debug, Clone)]
pub struct DirPad {
    keypad: [(Key, IVec2); 5],
    action: IVec2,
}

impl Default for DirPad {
    fn default() -> Self {
        use Key::*;

        #[rustfmt::skip]
        let keypad = [
                                 (Up, ivec2(1, 0)),   (Action, ivec2(2, 0)),
            (Left, ivec2(0, 1)), (Down, ivec2(1, 1)), (Right, ivec2(2, 1)),
        ];

        Self {
            keypad,
            action: ivec2(2, 0),
            //width: 3,
            //height: 2,
        }
    }
}

impl DirPad {
    pub fn at(&self, pos: IVec2) -> Key {
        let val = self
            .keypad
            .iter()
            .find(|(_, num_pos)| *num_pos == pos)
            .map(|&(e, _)| e);

        match val {
            Some(val) => val,
            None => panic!("could not find {pos}"),
        }
    }

    fn positions(&self) -> [IVec2; 5] {
        self.keypad.map(|(_, pos)| pos)
    }

    #[allow(clippy::type_complexity)]
    fn successor(
        &self,
        pos: IVec2,
        mut found: bool,
        key: Key,
        state: Vec<Key>,
    ) -> Vec<((IVec2, bool, Vec<Key>), usize)> {
        if key == Key::Action {
            found = true;
        }

        if key == Key::Action && self.at(pos) == key {
            let mut state = state.clone();
            state.push(Key::Action);
            return vec![((pos, true, state), 0)];
        }

        [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y]
            .iter()
            .filter_map(|&dir| {
                let pos = pos + dir;

                if !self.positions().contains(&pos) {
                    None
                } else if found {
                    let mut state = state.clone();
                    state.push(dir.into());
                    Some(((pos, true, state), 1))
                } else {
                    let mut state = state.clone();
                    state.push(dir.into());

                    if self.at(pos) == key {
                        state.push(Key::Action);
                    }

                    Some(((pos, self.at(pos) == key, state), 1))
                }
            })
            .collect()
    }

    fn pathfind(&self, keys: Vec<Key>) {
        let mut count = 0;

        for key in keys {
            let path = dijkstra(
                &(self.action, false, vec![]),
                |(pos, found, state)| self.successor(*pos, *found, key, state.clone()),
                |&(pos, found, _)| found && pos == self.action,
            )
            .expect("no paths found");

            let pathhhh = path.0.last().unwrap().2.clone();
            count += pathhhh.len();
            for k in pathhhh {
                print!("{}", k);
            }
        }

        println!("{count}");
    }

    pub fn parse_path(&self, path: (Vec<(IVec2, Num)>, usize)) {
        let end_state = path
            .0
            .iter()
            .map(|(_, state)| state.clone())
            .last()
            .expect("path should have some elements");

        for move_vec in &end_state.h_moves {
            match *move_vec {
                IVec2 { x: 1, y: 0 } => print!(">"),
                IVec2 { x: -1, y: 0 } => print!("<"),
                IVec2 { x: 0, y: 1 } => print!("v"),
                IVec2 { x: 0, y: -1 } => print!("^"),
                _ => print!("?"), // Unknown direction
            }
        }
        print!("A");

        for move_vec in &end_state.t_moves {
            match *move_vec {
                IVec2 { x: 1, y: 0 } => print!(">"),
                IVec2 { x: -1, y: 0 } => print!("<"),
                IVec2 { x: 0, y: 1 } => print!("v"),
                IVec2 { x: 0, y: -1 } => print!("^"),
                _ => print!("?"),
            }
        }
        print!("A");

        for move_vec in &end_state.o_moves {
            match *move_vec {
                IVec2 { x: 1, y: 0 } => print!(">"),
                IVec2 { x: -1, y: 0 } => print!("<"),
                IVec2 { x: 0, y: 1 } => print!("v"),
                IVec2 { x: 0, y: -1 } => print!("^"),
                _ => print!("?"),
            }
        }
        print!("A");

        println!();

        let moves = end_state
            .moves()
            .into_iter()
            .flat_map(|moves| {
                moves
                    .into_iter()
                    .map(Key::from)
                    .chain([Key::Action])
                    .collect_vec()
            })
            .collect_vec();

        self.pathfind(moves);
    }
}
