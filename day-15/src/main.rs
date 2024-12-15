#![allow(unused)]

use std::fmt::Display;

use glam::Vec2;

const INPUT: &str = "input1.txt";

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Movement::Up => '^',
            Movement::Right => '>',
            Movement::Down => 'v',
            Movement::Left => '<',
        };
        write!(f, "{c}")
    }
}

impl Movement {
    fn parse(i: char) -> Option<Self> {
        match i {
            '^' => Some(Self::Up),
            '>' => Some(Self::Right),
            'v' => Some(Self::Down),
            '<' => Some(Self::Left),
            '\n' => None,
            m => panic!("invalid movement input {m:?}"),
        }
    }

    fn direction(&self) -> Vec2 {
        match self {
            Movement::Up => Vec2::new(0.0, -1.0),
            Movement::Right => Vec2::new(1.0, 0.0),
            Movement::Down => Vec2::new(0.0, 1.0),
            Movement::Left => Vec2::new(-1.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Object {
    Wall,
    Box,
}

#[derive(Debug, Clone)]
struct Board {
    objects: Vec<(Object, Vec2)>,
    movement: Vec<Movement>,
    robot: Vec2,
    width: usize,
    height: usize,
}

impl Board {
    #[allow(dead_code)]
    fn print(&self) {
        let mut board = vec![vec![' '; self.width]; self.height];

        for (object, position) in &self.objects {
            let x = position.x as usize;
            let y = position.y as usize;
            board[y][x] = match object {
                Object::Wall => '#',
                Object::Box => 'O',
            };
        }

        let robot_x = self.robot.x as usize;
        let robot_y = self.robot.y as usize;
        board[robot_y][robot_x] = '@';

        for line in board {
            println!("{}", line.iter().collect::<String>());
        }
    }

    fn step(&mut self, movement: Movement) {
        let direction = movement.direction();

        let mut current_position = self.robot + direction;
        let mut positions_to_move = Vec::new();

        while let Some((index, obj)) = self
            .objects
            .iter()
            .enumerate()
            .find(|(_, (_, pos))| *pos == current_position)
        {
            match obj {
                (Object::Wall, _) => return,
                (Object::Box, _) => {
                    positions_to_move.push(index);
                    current_position += direction;
                }
            }
        }

        if self
            .objects
            .iter()
            .any(|(obj, pos)| *pos == current_position && matches!(obj, Object::Wall | Object::Box))
        {
            return;
        }

        for &index in positions_to_move.iter().rev() {
            self.objects[index].1 += direction;
        }

        self.robot += direction;
    }

    fn stepall(&mut self) {
        let movements = self.movement.clone();

        for movement in movements {
            self.step(movement);
        }
    }

    fn boxes(&self) -> Vec<Vec2> {
        self.objects
            .iter()
            .filter_map(|&(obj, pos)| {
                if matches!(obj, Object::Box) {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn get_data(input: &str) -> Board {
    let parts: Vec<_> = input.split("\n\n").collect();

    let mut objects = Vec::new();
    let mut robot = None;

    for (y, line) in parts[0].lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let position = Vec2::new(x as f32, y as f32);
            match ch {
                '#' => objects.push((Object::Wall, position)),
                'O' => objects.push((Object::Box, position)),
                '@' => robot = Some(position),
                _ => (),
            }
        }
    }

    let width = parts[0].lines().next().map_or(0, |line| line.len());
    let height = parts[0].lines().count();

    let robot = robot.expect("robot was not present in the input");

    let movement = parts[1].chars().filter_map(Movement::parse).collect();

    Board {
        objects,
        movement,
        robot,
        width,
        height,
    }
}

fn part1(mut data: Board) {
    let movements = data.movement.clone();

    data.stepall();

    let ans: f32 = data
        .boxes()
        .iter()
        .map(|Vec2 { x, y }| (y * 100.0) + x)
        .sum();

    println!("part 1: {}", ans as usize);
}

fn main() {
    let input = common::get_input(15, INPUT);
    let data = get_data(&input);

    part1(data.clone());
}
