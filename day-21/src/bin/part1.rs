#![allow(unused)]

use day_21::*;

fn main() {
    let input = include_str!("../../input1.txt");
}

#[cfg(test)]
mod test {
    use super::*;

    #[rstest::rstest]
    #[case("029A", 12)]
    #[case("980A", 12)]
    #[case("179A", 14)]
    #[case("456A", 12)]
    #[case("379A", 14)]
    fn move_numpad(#[case] num: &str, #[case] move_len: usize) {
        assert_eq!(NUMPAD.get_moves(num).len(), move_len);
    }

    #[rstest::rstest]
    fn dirpad_memory() {
        assert_eq!(DIRPAD.build_memory().len(), 25)
    }
}
