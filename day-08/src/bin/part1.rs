use std::collections::HashMap;

use glam::{ivec2, IVec2};

fn main() {
    let input = include_str!("../../input0.txt");

    let width = input.lines().next().map(|e| e.chars().count()).unwrap();
    let height = input.lines().count();

    let antennas: HashMap<char, Vec<IVec2>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c.is_ascii_alphanumeric() {
                    Some((c, ivec2(x as i32, y as i32)))
                } else {
                    None
                }
            })
        })
        .fold(HashMap::new(), |mut acc, (c, pos)| {
            acc.entry(c).or_default().push(pos);
            acc
        });
}
