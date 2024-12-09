use std::iter::repeat;

use indicatif::ProgressBar;
use itertools::Itertools;

const INPUT: &str = "input1.txt";

fn algorithm(input: &str) {
    let mut block_id = 0;

    let mut disk: Vec<_> = input
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

            // instead of using a string ".", we use max USIZE
            // this is unlikely to ever be a real block id, so we are kind of safe
            // and this still impliments Copy, so we dont have to clone
            sector.extend(repeat(usize::MAX).take(free_space as usize));

            sector
        })
        .collect();

    let mut cursor = disk.len() as isize - 1;

    let pb = ProgressBar::new(disk.len() as u64);

    while cursor >= 0 {
        let to_move = disk[cursor as usize];
        let mut moved = false;

        disk[cursor as usize] = usize::MAX;

        for c in &mut disk {
            if *c == usize::MAX && !moved {
                *c = to_move;
                moved = true;
            }
        }

        cursor -= 1;
        pb.inc(1);
    }

    let checksum: usize = disk
        .iter()
        .filter(|e| **e != usize::MAX)
        .enumerate()
        .map(|(i, num)| num * i)
        .sum();

    println!("part 1: {checksum}");
}

fn main() {
    let input = common::get_input(9, INPUT);
    algorithm(&input);
}
