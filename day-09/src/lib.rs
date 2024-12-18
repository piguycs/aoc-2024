#![allow(unused)]
#![allow(clippy::needless_range_loop)]

use std::char;

use itertools::Itertools;

pub fn print(digits: &[(bool, usize)]) {
    let mut id = 0;

    for &(is_file, space) in digits {
        let o = if is_file {
            let o = format!("{id}");
            id += 1;
            o
        } else {
            ".".to_string()
        };

        for _ in 0..space {
            print!("{o}");
        }
    }
    println!();
}

pub fn process(input: &str) -> usize {
    let mut digits = input
        .chars()
        .filter_map(|e| e.to_digit(10))
        .enumerate()
        .map(|(i, e)| (i % 2 == 0, e as usize))
        .collect_vec();

    let max: usize = digits.iter().map(|(_, e)| e).sum();

    let mut last_idx = digits.len() - 1;

    print(&digits);

    for i in 0..max {
        if let Some(&(is_file, space)) = digits.get(i) {
            if !is_file {
                println!("⎡space: {space}");
                let (_, last_space) = digits[last_idx];
                println!("⎣last file space: {}", last_space);

                if space > last_space {
                    let diff = space - last_space;
                }
            }
        } else {
            break;
        }
    }

    print(&digits);

    0
}
