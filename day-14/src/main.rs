mod gpt;

use indicatif::ProgressBar;
use plotters::prelude::*;
use regex::Regex;

use std::error::Error;
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
    #[allow(dead_code)]
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

    fn peak_density(&self) -> usize {
        let positions: Vec<_> = self.0.iter().map(|e| e.pos).collect();

        gpt::find_largest_cluster(positions)
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

fn plot(peak_densities: Vec<usize>) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("peak_densities.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Peak Densities", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0..peak_densities.len() as u32,
            0..*peak_densities.iter().max().unwrap_or(&0) as u32,
        )?;

    chart.draw_series(
        peak_densities
            .iter()
            .enumerate()
            .map(|(x, &y)| Circle::new((x as u32, y as u32), 5, RED.filled())),
    )?;

    // I retroactively added x + 1 and the limiter for y values once I got my answer
    // this is so that the graph I generate looks good when presenting to others
    for (x, &y) in peak_densities.iter().enumerate() {
        if y > 200 {
            chart.draw_series(std::iter::once(Text::new(
                format!("({},{})", x + 1, y),
                (x as u32, y as u32),
                ("monospace", 16).into_font(),
            )))?;
        }
    }

    chart.configure_mesh().draw()?;

    root.present()?;
    Ok(())
}

fn part2(mut data: Vec<Robot>, grid_width: i32, grid_height: i32) {
    let mut peak_densities = vec![];

    let pb = ProgressBar::new(10000);
    for _ in 0..10000 {
        for robot in &mut data {
            robot.tick(grid_width, grid_height);
        }
        let board = Board(data.clone());
        peak_densities.push(board.peak_density());
        pb.inc(1);
    }

    pb.finish();

    plot(peak_densities).unwrap();
}

fn show_val(mut data: Vec<Robot>, grid_width: i32, grid_height: i32, val: usize) {
    println!();
    for _ in 0..val {
        for robot in &mut data {
            robot.tick(grid_width, grid_height);
        }
    }
    let board = Board(data.clone());
    board.print(grid_width, grid_height);
}

fn main() {
    let inputs = [(INPUT_TEST, 11, 7), (INPUT_REAL, 101, 103)];
    let (file, grid_width, grid_height) = inputs[1];

    let input = common::get_input(14, file);
    let data = get_data(&input);

    part1(data.clone(), grid_width, grid_height);
    part2(data.clone(), grid_width, grid_height);

    // this was my answer
    show_val(data, grid_width, grid_height, 7502);
}
