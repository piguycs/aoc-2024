use glam::{ivec2, IVec2};
use pathfinding::prelude::*;

const A: usize = usize::MAX;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Num {
    hundreds: Option<usize>,
    tens: Option<usize>,
    ones: Option<usize>,

    pub h_moves: Vec<IVec2>,
    pub t_moves: Vec<IVec2>,
    pub o_moves: Vec<IVec2>,
    pub action_moves: Vec<IVec2>,
}

impl Num {
    /// ```rust
    /// return [self.h_moves, self.t_moves, self.o_moves, self.action_moves]
    /// ```
    pub fn moves(self) -> [Vec<IVec2>; 4] {
        [self.h_moves, self.t_moves, self.o_moves, self.action_moves]
    }

    fn parse(num: usize) -> Self {
        assert!(num <= 999, "number too big");

        let hundreds = Some((num / 100) % 10);
        let tens = Some((num / 10) % 10);
        let ones = Some(num % 10);

        Self {
            hundreds,
            tens,
            ones,

            h_moves: vec![],
            t_moves: vec![],
            o_moves: vec![],
            action_moves: vec![],
        }
    }

    fn found_all(&self) -> bool {
        self.next().is_none()
    }

    #[allow(clippy::manual_map)]
    fn next(&self) -> Option<usize> {
        if let Some(hundreds) = self.hundreds {
            Some(hundreds)
        } else if let Some(tens) = self.tens {
            Some(tens)
        } else if let Some(ones) = self.ones {
            Some(ones)
        } else {
            None
        }
    }

    fn add_move(&mut self, moves: IVec2) {
        if let Some(_hundreds) = self.hundreds {
            self.h_moves.push(moves);
        } else if let Some(_tens) = self.tens {
            self.t_moves.push(moves);
        } else if let Some(_ones) = self.ones {
            self.o_moves.push(moves);
        } else {
            unreachable!()
        }
    }

    fn change_next(&mut self) {
        if let Some(_hundreds) = self.hundreds {
            self.hundreds = None
        } else if let Some(_tens) = self.tens {
            self.tens = None
        } else if let Some(_ones) = self.ones {
            self.ones = None
        } else {
            unreachable!()
        }
    }
}

pub struct Keypad {
    keys: [(usize, IVec2); 11],
    action: IVec2,
}

impl Default for Keypad {
    fn default() -> Self {
        #[rustfmt::skip]
        let keys = [
            (7, ivec2(0, 0)), (8, ivec2(1, 0)), (9, ivec2(2, 0)),
            (4, ivec2(0, 1)), (5, ivec2(1, 1)), (6, ivec2(2, 1)),
            (1, ivec2(0, 2)), (2, ivec2(1, 2)), (3, ivec2(2, 2)),
                              (0, ivec2(1, 3)), (A, ivec2(2, 3))
        ];

        let action = ivec2(2, 3);

        Self { keys, action }
    }
}

impl Keypad {
    pub fn at(&self, pos: IVec2) -> usize {
        self.keys
            .iter()
            .find(|(_, num_pos)| *num_pos == pos)
            .map(|&(e, _)| e)
            .expect("invalid position")
    }

    fn positions(&self) -> [IVec2; 11] {
        self.keys.map(|(_, pos)| pos)
    }

    fn successor(&self, pos: IVec2, num: &Num) -> Vec<((IVec2, Num), usize)> {
        [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y]
            .iter()
            .filter_map(|direction| {
                let new_pos = pos + direction;

                if !self.positions().contains(&new_pos) {
                    return None;
                }

                let mut new_num = num.clone();

                if num.found_all() {
                    new_num.action_moves.push(*direction);
                    return Some(((new_pos, new_num), 1));
                } else if let Some(next_num) = num.next() {
                    new_num.add_move(*direction);

                    if self.at(new_pos) == next_num {
                        new_num.change_next();
                    }

                    return Some(((new_pos, new_num), 1));
                }

                None
            })
            .collect()
    }

    pub fn find(&self, num: usize) -> (Vec<(IVec2, Num)>, usize) {
        let num = Num::parse(num);

        let paths = astar_bag(
            &(self.action, num),
            |(pos, num)| self.successor(*pos, num),
            |_| 0,
            |(pos, num)| num.found_all() && *pos == self.action,
        )
        .expect("no path found");

        let possible_paths: Vec<_> = paths.0.collect();

        (possible_paths[0].clone(), paths.1)
    }
}
