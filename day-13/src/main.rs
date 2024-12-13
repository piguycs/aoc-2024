#![allow(unused)]

use itertools::Itertools;
use regex::Regex;

use std::{collections::HashMap, sync::LazyLock};

const INPUT: &str = "input1.txt";
const PATTERN: &str = r"\d+";
static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(PATTERN).unwrap());

const COST_A: i32 = 3;
const COST_B: i32 = 1;

#[derive(Debug)]
struct Data {
    a: (i32, i32),
    b: (i32, i32),
    target: (i32, i32),
}

fn parse_data(haystack: &str) -> Data {
    let nums = REGEX
        .find_iter(haystack)
        .map(|mat| mat.as_str().replace("\n", ""))
        .map(|s| s.parse::<i32>().expect("non digit detected"))
        .collect_vec();

    Data {
        a: (nums[0], nums[1]),
        b: (nums[2], nums[3]),
        target: (nums[4], nums[5]),
    }
}

fn get_data(input: &str) -> Vec<Data> {
    let re = Regex::new(PATTERN).expect("could not construct regex");
    input.split("\n\n").map(parse_data).collect_vec()
}

fn part1(data: &[Data]) -> usize {
    let presses = (0..=100)
        .permutations(2)
        .map(|e| (e[0], e[1]))
        .collect_vec();

    let mut cost = 0;

    // safe to assume MAX is an invalid number
    let mut min_cost = i32::MAX;

    for data in data {
        let (ax, ay) = data.a;
        let (bx, by) = data.b;
        let (tx, ty) = data.target;

        let mut solutions = vec![];

        for &(press_a, press_b) in &presses {
            let nx = (ax * press_a) + (bx * press_b);
            let ny = (ay * press_a) + (by * press_b);

            if (nx, ny) == data.target {
                solutions.push((press_a, press_b));
            }
        }

        let curr_cost = solutions
            .iter()
            .map(|(press_a, press_b)| (COST_A * press_a) + (COST_B * press_b))
            .min();

        if let Some(curr_cost) = curr_cost {
            cost += curr_cost;
        }
    }

    cost as usize
}

fn main() {
    let input = common::get_input(13, INPUT);
    let data = get_data(&input);

    let ans = part1(&data);
    println!("part 1: {ans}");
}
