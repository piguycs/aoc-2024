use std::{fs::read_to_string, path::PathBuf, str::FromStr};

const INPUT: &str = "input1.txt";
const NEEDLE: &str = "XMAS";
const NEEDLE_REV: &str = "SAMX";

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
struct Data {
    text: String,
    num_lines: usize,
    num_chars: usize,
}

impl Data {
    fn at(&self, row: usize, col: usize) -> Option<char> {
        if row < self.num_lines && col < self.num_chars {
            Some(self.text.as_bytes()[row * self.num_chars + col] as char)
        } else {
            None
        }
    }
}

fn get_data(filename: &str) -> Result<Data> {
    let path = PathBuf::from_str(filename)?;
    let content = read_to_string(path)?;

    let text = content.lines().collect();
    let num_lines = content.lines().count();
    let num_chars = content.lines().next().map(|line| line.len()).unwrap_or(0);

    Ok(Data {
        text,
        num_lines,
        num_chars,
    })
}

fn check_direction(data: &Data, x: usize, y: usize, dx: isize, dy: isize, needle: &str) -> bool {
    for (i, curr_char) in needle.chars().enumerate() {
        let x = x as isize + dx * i as isize;
        let y = y as isize + dy * i as isize;

        let num_lines = data.num_lines as isize;
        let num_chars = data.num_chars as isize;

        if x < 0 || y < 0 || x >= num_lines || y >= num_chars {
            return false;
        }

        if data.at(x as usize, y as usize) != Some(curr_char) {
            return false;
        }
    }

    true
}

fn part1(data: &Data) -> usize {
    let mut count = 0;

    // (dx, dy) values of the 4 directions we would traverse
    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];

    for needle in [NEEDLE, NEEDLE_REV] {
        for x in 0..data.num_lines {
            for y in 0..data.num_chars {
                for (dx, dy) in directions {
                    if check_direction(data, x, y, dx, dy, needle) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let data = get_data(INPUT)?;
    let num = part1(&data);
    println!("Part 1: {}", num);

    Ok(())
}
