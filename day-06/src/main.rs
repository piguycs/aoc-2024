#![allow(unused)]

use std::{collections::HashMap, ops::Add};

const INPUT: &str = "input0.txt";
const OBSTACLE: char = '#';

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    // unsafe function but we roll with it, I know what I am doing
    fn add(&mut self, direction: Direction) {
        let x = (self.x as isize) + direction.x;
        let y = (self.y as isize) + direction.y;

        self.x = x as usize;
        self.y = y as usize;
    }
}

#[derive(Debug, Clone, Copy)]
struct Direction {
    x: isize, // -1, 0 or 1
    y: isize, // -1, 0 or 1
}

impl Direction {
    fn xy(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn turn_90(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        let x = self.x as isize + rhs.x;
        let y = self.y as isize + rhs.y;

        Self {
            x: x as usize,
            y: y as usize,
        }
    }
}

#[derive(Debug, Clone)]
struct Board<'a> {
    data: Vec<&'a str>,
    cursor: Coord,
    /// max_y
    height: usize,
    /// max_x  
    width: usize,
}

impl<'a> Board<'a> {
    fn new(data: Vec<&'a str>) -> Self {
        let width = data[0].bytes().len();
        let height = data.len();

        Self {
            data,
            cursor: Coord { x: 0, y: 0 },
            width,
            height,
        }
    }

    fn at(&self, coord: Coord) -> char {
        let a = self.data[coord.y];
        a.as_bytes()[coord.x] as char
    }

    fn guard(&self, positions: HashMap<char, Direction>) -> (Direction, Coord) {
        let mut dummy = self.clone();

        let guard = dummy
            .find(|e| positions.contains_key(&e.item))
            .expect("no guard found on the board");

        let direction = *positions
            .get(&guard.item)
            .expect("guard position is not pre-set");

        let coord = guard.coord;

        (direction, coord)
    }
}

#[derive(Debug, Clone, Copy)]
struct Thing {
    item: char,
    coord: Coord,
}

impl Iterator for Board<'_> {
    type Item = Thing;

    fn next(&mut self) -> Option<Self::Item> {
        let thing = Thing {
            item: self.at(self.cursor),
            coord: self.cursor,
        };

        // first traverse the width
        if self.cursor.x >= self.width {
            self.cursor.x = 0;
            self.cursor.y += 1;
        } else {
            self.cursor.x += 1;
        }

        if self.cursor.y >= self.height {
            None
        } else {
            Some(thing)
        }
    }
}

//  0--->+x
//  |
//  |+y
//  v
//     (x, y)
// < : (-1, 0)
// ^ : (0, -1)
// > : (1, 0)
// v : (0, 1)

fn part1(input: &str) -> usize {
    let board = Board::new(input.lines().collect());

    let mut directions = HashMap::new();
    directions.insert('<', Direction::xy(-1, 0));
    directions.insert('^', Direction::xy(0, -1));
    directions.insert('>', Direction::xy(1, 0));
    directions.insert('v', Direction::xy(0, 1));

    let (guard_direction, guard_position) = board.guard(directions);

    let next_guard_pos = guard_position + guard_direction;
    let a = board.at(next_guard_pos);
    println!("{a}");

    todo!()
}

fn main() {
    let input = common::get_input(6, INPUT);
    println!("{}", part1(&input));
}
