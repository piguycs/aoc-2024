use day_17::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let mut _program = Program::parse(input).expect("invalid input provided");
}

#[test]
fn test() {
    let input = include_str!("../../input0.txt");
    let mut program = Program::parse(input).expect("invalid input provided");
    program.code.reverse();

    todo!();
}
