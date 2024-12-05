use std::{fs::read_to_string, path::PathBuf, str::FromStr};

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

fn part1(data: &Data) {
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

    println!("{count}");
}

fn main() -> Result<()> {
    let data = get_data()?;

    part1(&data);

    Ok(())
}
