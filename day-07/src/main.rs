use itertools::Itertools;

use std::str::FromStr;

const INPUT: &str = "input1.txt";

fn get_data(input: &str) {
    let mut ret = 0;

    input.lines().for_each(|e| {
        let values: Vec<_> = e.split(": ").collect();

        let sum = usize::from_str(values[0]).expect("non number found in the input");
        let numbers: Vec<_> = values[1]
            .split(" ")
            .map(|e| usize::from_str(e).expect("non number found in input"))
            .collect();

        let mut operations = vec![];

        let len_numbers = numbers.len();

        for _ in 0..len_numbers {
            operations.push('*');
            operations.push('+');
        }

        let combos = operations.iter().cloned().permutations(len_numbers - 1);

        for ops in combos {
            let mut result = numbers[0];

            // Combine numbers with operations
            for i in 1..len_numbers {
                match ops[i - 1] {
                    '+' => result += numbers[i],
                    '*' => result *= numbers[i],
                    _ => unreachable!(),
                };
            }

            if result == sum {
                ret += result;
                break;
            }
        }
    });

    println!("{ret}");
}

fn main() {
    let input = common::get_input(7, INPUT);
    get_data(&input);
}
