use std::{collections::HashMap, fs::read_to_string, path::PathBuf, str::FromStr};

const DELIMITER: &str = "   ";

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn get_lists() -> Result<(Vec<usize>, Vec<usize>)> {
    let path = PathBuf::from_str("day-01/input1.txt")?;

    let mut one = vec![];
    let mut two = vec![];

    for line in read_to_string(path)?.lines() {
        let line: Vec<_> = line.split(DELIMITER).take(2).collect();

        one.push(usize::from_str(line[0])?);
        two.push(usize::from_str(line[1])?);
    }

    Ok((one, two))
}

fn part1() -> Result<()> {
    let (mut one, mut two) = get_lists()?;

    one.sort();
    two.sort();

    let final_list: Vec<_> = one
        .into_iter()
        .zip(two)
        .map(|(e_one, e_two)| e_one.abs_diff(e_two))
        .collect();

    let answer: usize = final_list.iter().sum();
    println!("part1 answer: {}", answer);

    Ok(())
}

fn part2() -> Result<()> {
    let (mut one, mut two) = get_lists()?;

    one.sort();
    two.sort();

    let mut map: HashMap<usize, usize> = HashMap::new();

    let mut final_list = vec![];

    for item in one {
        let count = if let Some(count) = map.get(&item) {
            *count
        } else {
            let count = two.iter().filter(|e| **e == item).count();
            map.insert(item, count);
            count
        };

        final_list.push(item * count);
    }

    let answer: usize = final_list.iter().sum();
    println!("part2 answer: {}", answer);

    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()?;

    Ok(())
}
