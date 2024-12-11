// I keep this one here, cos it is still cool
#[allow(unused)]
mod old;

const INPUT: &str = "input1.txt";
const BLINKS: usize = 25;

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

#[derive(Debug, Clone)]
struct Data {
    stones: Vec<usize>,
}

fn get_data(input: &str) -> Data {
    let stones = input
        .replace("\n", "")
        .split(" ")
        .map(|e| e.parse::<usize>().expect("non number detected"))
        .collect();

    Data { stones }
}

fn algorithm(data: Data) -> Data {
    let mut new_stones = vec![];

    for value in data.stones.into_iter() {
        if value == 0 {
            new_stones.push(1);
        } else if let Some((left, right)) = get_split_digits(value) {
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(value * 2024);
        }
    }

    //for x in &new_stones {
    //    print!("{x} ");
    //}
    //println!();

    Data { stones: new_stones }
}

fn part1(mut data: Data) {
    let mut count = BLINKS;
    while count > 0 {
        data = algorithm(data);
        count -= 1;
    }

    println!("{}", data.stones.len());
}

fn main() {
    let input = common::get_input(11, INPUT);
    let data = get_data(&input);

    part1(data);
}
