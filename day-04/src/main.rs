use std::{fs::read_to_string, path::PathBuf, str::FromStr};

const INPUT: &str = "day-04/input1.txt";
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
    fn at(&self, x: usize, y: usize) -> Option<char> {
        if x < self.num_lines && y < self.num_chars {
            Some(self.text.as_bytes()[x * self.num_chars + y] as char)
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

fn part2(data: &Data) -> usize {
    let mut count = 0;

    let width = data.num_chars;
    let height = data.num_lines;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let edges = [
                ((x - 1, y - 1), (x + 1, y + 1)),
                ((x - 1, y + 1), (x + 1, y - 1)),
            ];

            let ((x1a, y1a), (x2a, y2a)) = edges[0];
            let ((x1b, y1b), (x2b, y2b)) = edges[1];

            let line_1 = (data.at(x1a, y1a), data.at(x, y), data.at(x2a, y2a));
            let line_2 = (data.at(x1b, y1b), data.at(x, y), data.at(x2b, y2b));

            let match_1 = matches!(
                line_1,
                (Some('M'), Some('A'), Some('S')) | (Some('S'), Some('A'), Some('M'))
            );

            let match_2 = matches!(
                line_2,
                (Some('M'), Some('A'), Some('S')) | (Some('S'), Some('A'), Some('M'))
            );

            if match_1 && match_2 {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let data = get_data(INPUT)?;
    let num = part1(&data);
    println!("Part 1: {}", num);

    let data = get_data(INPUT)?;
    let num = part2(&data);
    println!("Part 2: {}", num);

    Ok(())
}
