use day_18::*;

fn main() {
    let input = include_str!("../../input0.txt");
    parse(input, 70, 1024);
}

#[test]
fn test() {
    let input = include_str!("../../input0.txt");

    let board = parse(input, 6, 22);

    assert_eq!(board.corruption.len(), 22);
}
