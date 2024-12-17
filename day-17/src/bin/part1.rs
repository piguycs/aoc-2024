use itertools::Itertools;

use day_17::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let mut program = Program::parse(input).expect("invalid input provided");
    let out = program.run();

    let out = &out.iter().map(|e| e.to_string()).collect_vec().join(",");

    println!("{out}");
}
