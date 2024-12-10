use std::collections::HashSet;

const INPUT: &str = "input1.txt";

struct Data {
    input: String,
}

impl Data {
    pub fn at(&self, x: usize, y: usize) -> Option<usize> {
        self.input
            .lines()
            .nth(y)
            .and_then(|e| e.chars().nth(x))
            .and_then(|e| e.to_digit(10))
            .map(|e| e as usize)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    fn next_coords(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Self::Up => Some((x, y.checked_sub(1)?)),
            Self::Right => Some((x + 1, y)),
            Self::Down => Some((x, y + 1)),
            Self::Left => Some((x.checked_sub(1)?, y)),
        }
    }
}

fn recurse(
    input: &Data,
    start_num: isize,
    x: usize,
    y: usize,
    visited: &mut HashSet<(usize, usize)>,
    allow_unique: bool,
) -> usize {
    let mut count = 0;

    if visited.contains(&(x, y)) {
        return 0;
    }

    if let Some(num) = input.at(x, y) {
        if num as isize - start_num != 1 {
            return 0;
        }

        if num == 9 {
            if !allow_unique {
                visited.insert((x, y));
            }
            return 1;
        }

        for dir in Direction::ALL {
            if let Some((newx, newy)) = dir.next_coords(x, y) {
                let next = recurse(input, num as isize, newx, newy, visited, allow_unique);
                count += next;
            }
        }
    }

    count
}

fn part1(input: &str) {
    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '0' {
                let input = input.to_string();
                let mut visited = HashSet::new();
                let count = recurse(&Data { input }, -1, x, y, &mut visited, false);
                sum += count;
            }
        }
    }

    println!("part1: {sum}");
}

fn part2(input: &str) {
    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '0' {
                let input = input.to_string();
                let mut visited = HashSet::new();
                let count = recurse(&Data { input }, -1, x, y, &mut visited, true);
                sum += count;
            }
        }
    }

    println!("part2: {sum}");
}

fn main() {
    let input = common::get_input(10, INPUT);
    part1(&input);
    part2(&input);
}
