use std::str::FromStr;

const INPUT: &str = "input1.txt";

const PART_ONE_OPS: [char; 2] = ['+', '*'];
const PART_TWO_OPS: [char; 3] = ['+', '*', 'c'];

// NOTE: we use c instead of || to indicate concat, since I use char for my operations instead of &str

fn concat(a: usize, b: usize) -> usize {
    let mut factor = 1;

    while b >= factor {
        factor *= 10;
    }

    a * factor + b
}

fn eval(numbers: &[usize], operations: &[char]) -> usize {
    let mut result = numbers[0];

    for (i, &op) in operations.iter().enumerate() {
        result = match op {
            '+' => result + numbers[i + 1],
            '*' => result * numbers[i + 1],
            'c' => concat(result, numbers[i + 1]),
            _ => result,
        };
    }

    result
}

fn find_right_result(numbers: &[usize], target: usize, operations: Vec<char>) -> Option<usize> {
    if operations.len() == numbers.len() - 1 {
        // this will usually fail for the first call, since we pass no operations
        let result = eval(numbers, &operations);
        if result == target {
            return Some(result);
        } else {
            return None;
        }
    }

    // Recurse by trying both '+' and '*' at the current position.
    // AND ALSO ||, but we represent it using c
    let possible_ops = PART_TWO_OPS;

    for op in possible_ops {
        let mut new_ops = operations.clone();
        new_ops.push(op);
        if let Some(result) = find_right_result(numbers, target, new_ops) {
            return Some(result);
        }
    }

    None
}

fn get_data(input: &str) {
    let mut ret = 0;

    input.lines().for_each(|e| {
        let values: Vec<_> = e.split(": ").collect();

        let sum = usize::from_str(values[0]).expect("non number found in the input");
        let numbers: Vec<_> = values[1]
            .split(" ")
            .map(|e| usize::from_str(e).expect("non number found in input"))
            .collect();

        if let Some(result) = find_right_result(&numbers, sum, vec![]) {
            ret += result;
        }
    });

    println!("{ret}");
}

fn main() {
    let input = common::get_input(7, INPUT);
    get_data(&input);
}
