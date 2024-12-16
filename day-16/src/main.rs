#![allow(unused)]

use std::collections::HashSet;

use glam::IVec2 as Vec2;
use pathfinding::prelude::*;

const INPUT: &str = "input1.txt";

const COST_TURN: usize = 1000;
const COST_MOVE: usize = 1;

#[derive(Debug, Clone)]
struct Board {
    walls: Vec<Vec2>,
    start: Vec2,
    end: Vec2,

    // for printing
    width: usize,
    height: usize,
}

impl Board {
    fn parse(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().nth(0).map(|e| e.chars().count()).unwrap();

        let mut walls = vec![];

        let mut start = None;
        let mut end = None;

        for y in 0..height {
            for x in 0..width {
                let c = input.lines().nth(y).and_then(|e| e.chars().nth(x)).unwrap();
                let pos = Vec2::new(x as i32, y as i32);

                match c {
                    '#' => walls.push(pos),
                    'S' => start = Some(pos),
                    'E' => end = Some(pos),
                    _ => (),
                }
            }
        }

        let start = start.expect("no start pos found");
        let end = end.expect("no end pos found");

        Self {
            height,
            width,
            walls,
            start,
            end,
        }
    }

    fn print(&self, player: Vec2) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Vec2::new(x as i32, y as i32);

                if pos == player {
                    print!("\x1b[7m");
                }

                if self.walls.contains(&pos) {
                    print!("#");
                } else if self.start == pos {
                    print!("S");
                } else if self.end == pos {
                    print!("E");
                } else {
                    print!(" ");
                }

                print!("\x1b[0m");
            }
            println!();
        }
    }
}

fn part1n2(board: &Board) {
    // start position, facing east (positive X)
    let start = (board.start, Vec2::X);

    // credit: Chris Biscardi
    let res = astar_bag(
        &start,
        |(position, direction)| {
            let next_pos = position + direction;

            // turn clockwise and counterclockwise
            let rotations = vec![
                ((*position, direction.perp()), COST_TURN),
                ((*position, -direction.perp()), COST_TURN),
            ];

            if board.walls.contains(&next_pos) {
                // without moving forward
                rotations
            } else {
                // move forward
                let mut moves = vec![((next_pos, *direction), COST_MOVE)];
                moves.extend(rotations); // and turn too
                moves
            }
        },
        |_| 0,
        |&(pos, _)| pos == board.end,
    )
    .unwrap();

    println!("part1: {}", res.1);

    let paths = res.0;
    let paths: HashSet<_> = paths
        .flat_map(|path| path.into_iter().map(|(position, _)| position))
        .collect();

    println!("part2: {}", paths.len());
}

fn main() {
    let input = common::get_input(16, INPUT);
    let board = Board::parse(&input);

    part1n2(&board);
}
