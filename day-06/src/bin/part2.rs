use day_06::Board;

fn main() {
    let input = include_str!("../../input1.txt");
    let board = Board::parse(input);
    println!("{}", board.paradox());
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
    fn obstacle(board: Board) {
        let path = board.dijkstra_with_obstacle(Some(IVec2 { x: 3, y: 6 }));
        assert!(path.is_none());
    }

    #[rstest::rstest]
    fn paradox(board: Board) {
        assert_eq!(board.paradox(), 6);
    }
}
