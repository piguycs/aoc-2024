use day_24::Solver;

fn main() {
    let mut solver = Solver::load(include_str!("../../input1.txt"));
    println!("{}", solver.solve());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let mut solver = Solver::load(include_str!("../../input0.txt"));
        assert_eq!(solver.solve(), 2024);
    }
}
