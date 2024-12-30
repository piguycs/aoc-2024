use std::collections::HashMap;

use glam::{ivec2, IVec2};
use itertools::Itertools;

fn main() {
    let input = include_str!("../../input1.txt");

    let width = input.lines().next().map(|e| e.chars().count()).unwrap() as i32;
    let height = input.lines().count() as i32;

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

    let ans = antennas
        .values()
        .flat_map(|positions| {
            positions
                .iter()
                .combinations(2)
                .flat_map(|poss| {
                    let spacing = poss[0] - poss[1];
                    [poss[0] + spacing, poss[1] - spacing]
                })
                .filter(|&IVec2 { x, y }| (0..width).contains(&x) && (0..height).contains(&y))
        })
        .unique()
        .count();

    println!("{ans}");
}
