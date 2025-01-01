#![allow(unused)]

use day_21::*;

fn main() {
    let input = include_str!("../../input1.txt");

    let mut moves = NUMPAD.get_moves("029A");

    let k = numpad::Keypad::default();
    k.find(29);
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
    fn dirpad_memory_gen() {
        assert_eq!(DIRPAD.mem.len(), 25)
    }

    // idk about other test cases
    #[rstest::rstest]
    #[case("029A", 28, 68)]
    fn dirpad_traverse(#[case] num: &str, #[case] len1: usize, #[case] len2: usize) {
        let moves = NUMPAD.get_moves(num);
        //dbg!(&moves);

        let new_moves = DIRPAD.traverse_seq(moves);
        assert_eq!(new_moves.len(), len1);

        let new_moves = DIRPAD.traverse_seq(new_moves);
        assert_eq!(new_moves.len(), len2);
    }
}
