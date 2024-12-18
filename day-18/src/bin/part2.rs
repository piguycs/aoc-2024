use day_18::*;

fn main() {
    let input = include_str!("../../input1.txt");

    let max_len = input.lines().count() as u32;

    let mut i = max_len;

    while i > 0 {
        let board = Board::parse(input, 70, i);

        if board.traverse().is_some() {
            let ans = input.lines().nth(i as usize);
            println!("part 2: {ans:?}");
            break;
        }

        i -= 1;
    }
}
