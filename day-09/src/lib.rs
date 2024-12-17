#![allow(unused)]
#![allow(clippy::needless_range_loop)]

use std::char;

use itertools::Itertools;

pub fn parse(input: &str) {
    let mut digits = input
        .chars()
        .filter_map(|e| e.to_digit(10).map(|e| e as usize))
        .collect_vec();

    // this is the maximum possible iterations I would need
    // in my old implimentation, this would have been the length of the uncompressed array
    let max: usize = digits.iter().sum();

    let mut end_idx = digits.len() - 1;

    for i in 0..max {
        if i >= digits.len() || end_idx == 0 {
            break;
        }

        let is_free = i % 2 != 0;
        let is_file = !is_free;

        let space = digits[i];

        if is_free {
            let last_space = digits[end_idx - 1];

            if last_space >= space {
                let remaining_space = last_space - space;
                digits.insert(i + 1, remaining_space);
                digits.insert(i + 2, 0);
                digits[end_idx] = last_space - remaining_space;
            } else {
                let remaining_space = space - last_space;
                digits.insert(i + 1, last_space);
                digits.insert(i + 2, remaining_space);
                digits[end_idx] = 0;
            }

            // set current space to nul
            digits[i] = 0;
            // decrease end index by 1
            end_idx -= 1;
        }
    }
}
