use itertools::Itertools;

use day_17::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let mut program = Program::parse(input).expect("invalid input provided");
    let out = program.run();

    let out = &out.iter().map(|e| e.to_string()).collect_vec().join(",");

    println!("{out}");
}

#[test]
fn test() {
    let input = include_str!("../../input0.txt");
    let program = Program::parse(input);

    assert!(program.is_some(), "program was parsed incorrectly");
    let mut program = program.unwrap();

    assert_eq!(program.reg_a, 729);
    assert_eq!(program.reg_b, 0);
    assert_eq!(program.reg_c, 0);

    assert_eq!(program.code, [0, 1, 5, 4, 3, 0]);

    let out = program.run();

    assert_eq!(out, [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
}
