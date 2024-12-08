#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

const INPUT: &str = "input0.txt";
const FREQUENCIES: RangeInclusive<char> = '0'..='z';
const EMPTY: char = '.';
const BLOCK: char = '#';

fn get_data(input: String) {
    let unique: HashSet<char> = input.chars().filter(|e| FREQUENCIES.contains(e)).collect();

    let mut antennas: HashMap<_, Vec<String>> = unique.iter().map(|e| (*e, vec![])).collect();
    antennas.iter_mut().for_each(|(key, val)| {
        let filtered: String = input
            .chars()
            .map(|e| if [*key, '\n'].contains(&e) { e } else { EMPTY })
            .collect();

        println!("{filtered}");

        *val = filtered.lines().map(String::from).collect();
    });
}

fn main() {
    let input = common::get_input(8, INPUT);

    get_data(input);
}
