use std::collections::HashMap;

const INPUT: &str = "input1.txt";

fn get_split_digits(num: usize) -> Option<(usize, usize)> {
    // fun trivia, log10 of a number returns num_digits - 1
    // look it up, I am not lying
    let digit_count = (num as f64).log10().floor() as usize + 1;

    if digit_count % 2 == 0 {
        let half = digit_count / 2;

        let first = num / usize::pow(10, half as u32);
        let second = num % usize::pow(10, half as u32);

        return Some((first, second));
    }

    None
}

fn get_data(input: &str) -> Vec<usize> {
    let stones = input
        .replace("\n", "")
        .split(" ")
        .map(|e| e.parse::<usize>().expect("non number detected"))
        .collect();

    stones
}

// records how many times any number is present
type Data = HashMap<usize, usize>;

fn algorithm(stones: Data) -> Data {
    let mut new_stones = Data::new();

    for (stone, count) in stones {
        if stone == 0 {
            let new_stone = 1;
            *new_stones.entry(new_stone).or_insert(0) += count;
        } else if let Some((left, right)) = get_split_digits(stone) {
            *new_stones.entry(left).or_insert(0) += count;
            *new_stones.entry(right).or_insert(0) += count;
        } else {
            let new_stone = stone * 2024;
            *new_stones.entry(new_stone).or_insert(0) += count;
        }
    }

    new_stones
}

fn blink_n_times(stones: Vec<usize>, blinks: usize) {
    let mut count = blinks;

    let mut data = Data::new();

    for stone in stones {
        *data.entry(stone).or_insert(0) += 1;
    }

    while count > 0 {
        data = algorithm(data);

        count -= 1;
    }

    println!("{}", data.values().sum::<usize>());
}

fn main() {
    let input = common::get_input(11, INPUT);
    let data = get_data(&input);

    // part1: 25
    // part2: 75

    blink_n_times(data, 75);
}
