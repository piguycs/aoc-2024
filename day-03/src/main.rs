use std::{fs::read_to_string, path::PathBuf, str::FromStr};

use regex::Regex;

const INPUT: &str = "day-03/input1.txt";

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn get_data() -> Result<String> {
    let path = PathBuf::from_str(INPUT)?;
    let ret = read_to_string(path)?;

    Ok(ret)
}

fn part1(data: String) -> Result<isize> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let mut answer = 0;

    for (_, [one, two]) in re.captures_iter(&data).map(|c| c.extract()) {
        let one = isize::from_str(one)?;
        let two = isize::from_str(two)?;

        answer += one * two;
    }

    Ok(answer)
}

fn part2(data: String) -> Result<isize> {
    let re = Regex::new(r"(don't\(\))|(do\(\))|(mul\(\d+,\d+\))")?;
    let re_mul_vals = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let mut switch = true;
    let mut answer = 0;

    for (_, [cap]) in re.captures_iter(&data).map(|c| c.extract()) {
        if cap == "do()" {
            switch = true;
        } else if cap == "don't()" {
            switch = false;
        } else if switch {
            if let Some(cap) = re_mul_vals.captures(cap) {
                let (_, [one, two]) = cap.extract();

                let one = isize::from_str(one)?;
                let two = isize::from_str(two)?;

                answer += one * two;
            }
        }
    }

    Ok(answer)
}

fn main() -> Result<()> {
    let data = get_data()?;

    let answer = part1(data.clone())?;
    println!("answer part1: {answer}");

    let answer = part2(data)?;
    println!("answer part2: {answer}");

    Ok(())
}
