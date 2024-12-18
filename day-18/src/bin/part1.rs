use day_18::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let board = Board::parse(input, 70, 1024);

    let ans = board.traverse();

    println!("part 1: {ans}");
}

#[test]
fn test() {
    let input = include_str!("../../input0.txt");

    let board = Board::parse(input, 6, 12);

    assert_eq!(board.corruption.len(), 12);

    let score = board.traverse();

    assert_eq!(score, 22);
}
