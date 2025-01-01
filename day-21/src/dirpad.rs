use glam::{ivec2, IVec2};
use pathfinding::prelude::*;

use std::collections::HashMap;

use super::Dir;

pub struct DirPad {
    pub keypad: [(Dir, IVec2); 5],
    pub mem: HashMap<(Dir, Dir), Vec<Vec<Dir>>>,
}

impl Default for DirPad {
    fn default() -> Self {
        use super::Dir::*;

        #[rustfmt::skip]
        let keypad = [
                                 (Up,   ivec2(1, 0)), (Action, ivec2(2, 0)),
            (Left, ivec2(0, 1)), (Down, ivec2(1, 1)), (Right,  ivec2(2, 1)),
        ];

        let mut pad = Self {
            keypad,
            mem: HashMap::new(),
        };

        pad.mem = build_memory(&pad);

        pad
    }
}

pub fn build_memory(pad: &DirPad) -> HashMap<(Dir, Dir), Vec<Vec<Dir>>> {
    let mut mem = HashMap::new();

    let combo = itertools::iproduct!(Dir::ALL, Dir::ALL).collect::<Vec<_>>();

    for (start, end) in combo {
        let start_pos = pad.ta(start);
        let end_pos = pad.ta(end);

        let path = astar_bag_collect(
            &start_pos,
            |&pos| {
                [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y]
                    .iter()
                    .filter_map(move |&d| {
                        let pos = pos + d;

                        pad.at(pos).map(|_| (pos, 1))
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

pub fn path_to_dir(positions: &[IVec2]) -> Vec<Dir> {
    let mut moves = vec![];

    for i in 0..positions.len() - 1 {
        let diff = positions[i + 1] - positions[i];
        let dir = match diff {
            d if d == IVec2::X => Dir::Right,
            d if d == IVec2::NEG_X => Dir::Left,
            d if d == IVec2::Y => Dir::Down,
            d if d == IVec2::NEG_Y => Dir::Up,
            d => panic!("Invalid direction {d}"),
        };
        moves.push(dir);
    }

    moves
}
