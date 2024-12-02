use std::{fs::read_to_string, ops::RangeInclusive, path::PathBuf, str::FromStr};

const INPUT: &str = "input1.txt";
const DELIMITER: &str = " ";
const SAFE_RANGE: RangeInclusive<usize> = 0..=3;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn get_data() -> Result<Vec<Vec<usize>>> {
    let path = PathBuf::from_str(INPUT)?;

    let mut data = vec![];

    for line in read_to_string(path)?.lines() {
        let line: Vec<_> = line
            .split(DELIMITER)
            .map(usize::from_str)
            .filter_map(|e| e.ok())
            .collect();

        data.push(line);
    }

    Ok(data)
}

fn part1(data: Vec<Vec<usize>>) -> Result<usize> {
    let mut safe = 0;

    let is_increasing = |arr: &Vec<usize>| arr.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = |arr: &Vec<usize>| arr.windows(2).all(|w| w[0] > w[1]);

    let is_in_range = |arr: &Vec<usize>| {
        arr.windows(2).all(|w| {
            let diff = w[0] - w[1];
            (SAFE_RANGE).contains(&diff)
        })
    };

    for row in data {
        let is_safe = if is_increasing(&row) {
            let row: Vec<_> = row.into_iter().rev().collect();
            is_in_range(&row)
        } else if is_decreasing(&row) {
            is_in_range(&row)
        } else {
            false
        };

        if is_safe {
            safe += 1;
        }
    }

    Ok(safe)
}

fn main() -> Result<()> {
    let data = get_data()?;

    let safe = part1(data)?;
    println!("Part1: number of safe reports: {}", safe);

    Ok(())
}
