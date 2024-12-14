use regex::Regex;

use std::sync::LazyLock;

const INPUT_TEST: &str = "input0.txt";
const INPUT_REAL: &str = "input1.txt";

const PATTERN: &str = r"-?\d+";
static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(PATTERN).unwrap());

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    pub fn tick(&mut self, max_width: i32, max_height: i32) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;

        while self.pos.0 < 0 {
            self.pos.0 += max_width;
        }

        while self.pos.0 > (max_width - 1) {
            self.pos.0 -= max_width;
        }

        while self.pos.1 < 0 {
            self.pos.1 += max_height;
        }

        while self.pos.1 > (max_height - 1) {
            self.pos.1 -= max_height;
        }
    }

    fn parse(haystack: &str) -> Robot {
        let nums: Vec<_> = REGEX
            .find_iter(haystack)
            .map(|mat| mat.as_str().replace("\n", ""))
            .map(|s| s.parse::<i32>().expect("non digit detected"))
            .collect();

        Robot {
            pos: (nums[0], nums[1]),
            velocity: (nums[2], nums[3]),
        }
    }
}

struct Board(Vec<Robot>);

impl Board {
    fn print(&self, width: i32, height: i32) {
        let mx = width / 2;
        let my = height / 2;

        for y in 0..height {
            if y == my {
                println!();
                continue;
            }
            for x in 0..width {
                if x == mx {
                    print!(" ");
                    continue;
                }

                let n = self.num_at(x, y).len();
                if n == 0 {
                    print!(".");
                } else if n > 9 {
                    print!("+");
                } else {
                    print!("{n}");
                }
            }
            println!();
        }
    }

    fn num_at(&self, x: i32, y: i32) -> Vec<Robot> {
        self.0.iter().filter(|e| e.pos == (x, y)).copied().collect()
    }

    fn quadrants(&self, width: i32, height: i32) -> Vec<Vec<Robot>> {
        let mx = width / 2;
        let my = height / 2;

        let mut q1 = vec![];
        let mut q2 = vec![];
        let mut q3 = vec![];
        let mut q4 = vec![];

        for robot in &self.0 {
            let (rx, ry) = robot.pos;
            if rx < mx && ry < my {
                q1.push(*robot);
            } else if rx > mx && ry < my {
                q2.push(*robot);
            } else if rx > mx && ry > my {
                q3.push(*robot);
            } else if rx < mx && ry > my {
                q4.push(*robot);
            }
        }

        vec![q1, q2, q3, q4]
    }
}

fn get_data(input: &str) -> Vec<Robot> {
    input.lines().map(Robot::parse).collect()
}

fn part1(mut data: Vec<Robot>, grid_width: i32, grid_height: i32) {
    for _ in 0..100 {
        for robot in &mut data {
            robot.tick(grid_width, grid_height);
        }
    }

    let board = Board(data.clone());

    let quads = board.quadrants(grid_width, grid_height);

    let values: Vec<_> = quads.iter().map(|e| e.len()).collect();

    let mut p1 = 1;

    for v in values {
        p1 *= v;
    }

    println!("part 1: {p1}");
}

fn main() {
    let inputs = [(INPUT_TEST, 11, 7), (INPUT_REAL, 101, 103)];
    let (file, grid_width, grid_height) = inputs[0];

    let input = common::get_input(14, file);
    let data = get_data(&input);

    part1(data.clone(), grid_width, grid_height);
}
