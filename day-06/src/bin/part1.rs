use day_06::Board;

fn main() {
    let input = include_str!("../../input1.txt");
    let board = Board::parse(input);
    println!("{}", board.traverse());
}

#[cfg(test)]
mod test {
    use glam::IVec2;

    use super::*;

    #[rstest::fixture]
    fn board() -> Board {
        let input = include_str!("../../input0.txt");
        Board::parse(input)
    }

    #[rstest::rstest]
    fn test_parse(board: Board) {
        assert_eq!(board.width, 10);
        assert_eq!(board.height, 10);

        assert_eq!(board.guard, IVec2 { x: 4, y: 6 });
        assert_eq!(board.guard_dir, IVec2::NEG_Y);
    }

    #[rstest::rstest]
    fn test_traverse(board: Board) {
        assert_eq!(board.traverse(), 41);
    }
}
