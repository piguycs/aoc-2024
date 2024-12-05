use std::{cmp::Ordering, fs::read_to_string, path::PathBuf, str::FromStr};

const INPUT: &str = "input1.txt";

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Clone, Copy)]
struct Rule {
    before: usize,
    after: usize,
}

#[derive(Clone)]
struct Rules(Vec<Rule>);

impl Rules {
    pub fn cmp(&self, a: usize, b: usize) -> bool {
        let rules = &self.0;

        let failures = rules
            .iter()
            .filter_map(|rule| {
                if a == rule.after && b == rule.before {
                    Some(())
                } else {
                    None
                }
            })
            .count();

        failures == 0
    }

    pub fn sort(&self, a: usize, b: usize) -> Ordering {
        let rules = &self.0;

        let failures = rules
            .iter()
            .filter_map(|rule| {
                if a == rule.after && b == rule.before {
                    Some(())
                } else {
                    None
                }
            })
            .count();

        if failures == 0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Clone)]
struct Data {
    rules: Rules,
    pages: Vec<String>,
}

fn get_data() -> Result<Data> {
    let path = PathBuf::from_str(INPUT)?;
    let ret = read_to_string(path)?;

    let data: Vec<_> = ret.split("\n\n").collect();

    let rules = data[0]
        .lines()
        .map(|e| {
            let pages: Vec<_> = e
                .splitn(2, "|")
                .filter_map(|e| usize::from_str(e).ok())
                .collect();

            Rule {
                before: pages[0],
                after: pages[1],
            }
        })
        .collect();

    let rules = Rules(rules);
    let pages = data[1].lines().map(String::from).collect();

    Ok(Data { rules, pages })
}

fn mid_value<T: Copy>(array: &[T]) -> T {
    array[array.len() / 2]
}

fn part1(data: &Data) -> usize {
    let mut count = 0;

    for update in &data.pages {
        let values: Vec<_> = update
            .split(",")
            .filter_map(|e| usize::from_str(e).ok())
            .collect();

        if values.is_sorted_by(|a, b| data.rules.cmp(*a, *b)) {
            count += mid_value(&values);
        }
    }

    count
}

fn part2(data: &Data) -> usize {
    let mut count = 0;

    for update in &data.pages {
        let mut values: Vec<_> = update
            .split(",")
            .filter_map(|e| usize::from_str(e).ok())
            .collect();

        if values.is_sorted_by(|a, b| data.rules.cmp(*a, *b)) {
            //count += mid_value(&values);
        } else {
            values.sort_by(|a, b| data.rules.sort(*a, *b));
            count += mid_value(&values);
        }
    }

    count
}

fn main() -> Result<()> {
    let data = get_data()?;

    let count = part1(&data);
    println!("part 1: {count}");

    let count = part2(&data);
    println!("part 2: {count}");

    Ok(())
}
