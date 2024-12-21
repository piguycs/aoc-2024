use day_20::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let board = Board::parse(input);

    let scores = board.traverse();

    let a = scores.iter().filter(|&&e| e == 100).count();

    println!("{a}");
}

#[test]
fn test() {
    let input = include_str!("../../input0.txt");
    let board = Board::parse(input);

    assert_eq!(board.start, glam::ivec2(1, 3));
    assert_eq!(board.end, glam::ivec2(5, 7));

    let scores = board.traverse();

    let twaalf = scores.iter().filter(|&&e| e == 12).count();

    assert_eq!(twaalf, 3);
}
