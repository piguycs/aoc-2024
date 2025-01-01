use glam::{ivec2, IVec2};
use itertools::Itertools;
use pathfinding::prelude::*;

use crate::Dir;

pub struct NumPad {
    pub keypad: [(char, IVec2); 11],
    pub action: IVec2,
}

impl Default for NumPad {
    fn default() -> Self {
        let action = ivec2(2, 3);

        #[rustfmt::skip]
        let keypad = [
            ('7', ivec2(0, 0)), ('8', ivec2(1, 0)), ('9', ivec2(2, 0)),
            ('4', ivec2(0, 1)), ('5', ivec2(1, 1)), ('6', ivec2(2, 1)),
            ('1', ivec2(0, 2)), ('2', ivec2(1, 2)), ('3', ivec2(2, 2)),
                                ('0', ivec2(1, 3)), ('A', action),
        ];

        Self { keypad, action }
    }
}

impl NumPad {
    pub fn at(&self, pos: IVec2) -> Option<char> {
        self.keypad
            .iter()
            .find(|&&(_, coord)| coord == pos)
            .map(|&(e, _)| e)
    }

    pub fn ta(&self, c: char) -> Option<IVec2> {
        self.keypad
            .iter()
            .find(|&&(curr, _)| curr == c)
            .map(|&(_, e)| e)
    }

    pub fn get_moves(&self, seq: &str) -> Vec<Dir> {
        seq.chars()
            .scan(self.action, |state, c| {
                let moves = self.get_moves_c(c, *state);
                *state = self.ta(c).unwrap(); // infailable
                Some(moves)
            })
            .flatten()
            .collect()
    }

    pub fn get_moves_c(&self, c: char, start: IVec2) -> Vec<Dir> {
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

        moves.push(Dir::Action);

        moves
    }

    pub fn get_moves_c_all(&self, c: char, start: IVec2) -> Vec<Vec<Dir>> {
        let mut moves: Vec<Dir> = vec![];

        let path = astar_bag_collect(
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
            |_| 0,
            |&pos| self.at(pos) == Some(c),
        )
        .expect("no valid path found");

        let mut all_moves = vec![];

        for positions in &path.0 {
            let mut moves = super::dirpad::path_to_dir(positions);
            moves.push(Dir::Action);
            all_moves.push(moves);
        }

        all_moves
    }

    pub fn get_moves_all(&self, seq: &str) -> Vec<Vec<Dir>> {
        let c1 = seq.chars().nth(0).unwrap();
        let c2 = seq.chars().nth(1).unwrap();
        let c3 = seq.chars().nth(2).unwrap();
        let c4 = seq.chars().nth(3).unwrap(); // always going to be 'A'

        let m1 = self.get_moves_c_all(c1, self.action);
        let m2 = self.get_moves_c_all(c2, self.ta(c1).unwrap());
        let m3 = self.get_moves_c_all(c3, self.ta(c2).unwrap());
        let m4 = self.get_moves_c_all(c4, self.ta(c3).unwrap());

        let directions = vec![m1, m2, m3, m4];

        combine_directions(&directions, 0, vec![])
    }
}

pub fn combine_directions(
    directions: &Vec<Vec<Vec<Dir>>>,
    depth: usize,
    current: Vec<Dir>,
) -> Vec<Vec<Dir>> {
    if depth == directions.len() {
        return vec![current];
    }

    let mut result = Vec::new();
    for subarray in &directions[depth] {
        let mut new_current = current.clone();
        new_current.extend_from_slice(subarray);
        result.extend(combine_directions(directions, depth + 1, new_current));
    }

    result
}
