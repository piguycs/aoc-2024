#![allow(unused)]

use std::{collections::HashMap, iter::repeat};

use indicatif::ProgressBar;
use itertools::Itertools;

const INPUT: &str = "input0.txt";

// instead of using a string ".", we use usize::MAX
// this is unlikely to ever be a real block id, so we are kind of safe
// and this still impliments Copy, so we dont have to clone
// alternative would be to start block_id at 1, and set value of this field to 0
const EMPTY: usize = usize::MAX;

fn get_data(input: &str) -> Vec<usize> {
    let mut block_id = 0;

    let disk: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .tuples()
        .flat_map(|(file_size, free_space)| {
            // using "sector" to represent file followed by free space
            // with capacity so all the allocations happen at once
            // this is to speed up the algorithm for the massive input file (hopefully)
            let mut sector = Vec::with_capacity((file_size + free_space) as usize);

            sector.extend(repeat(block_id).take(file_size as usize));
            block_id += 1;

            sector.extend(repeat(EMPTY).take(free_space as usize));

            sector
        })
        .collect();

    disk
}

fn checksum(disk: Vec<usize>) -> usize {
    disk.iter()
        .filter(|e| **e != EMPTY)
        .enumerate()
        .map(|(i, num)| num * i)
        .sum()
}

fn part1(mut disk: Vec<usize>) {
    let mut cursor = disk.len() as isize - 1;

    let pb = ProgressBar::new(cursor as u64);

    while cursor >= 0 {
        // this is the last element
        let to_move = disk[cursor as usize];

        // we set the last element to be empty
        disk[cursor as usize] = EMPTY;

        for c in disk.iter_mut() {
            if *c == EMPTY {
                *c = to_move;
                break;
            }
        }

        cursor -= 1;
        pb.inc(1);
    }

    pb.finish();

    println!("part 1: {}", checksum(disk));
}

fn part2(mut disk: Vec<usize>) {
    //
}

fn main() {
    let input = common::get_input(9, INPUT);
    let disk = get_data(&input);

    //part1(disk.clone());
    part2(disk);
}
