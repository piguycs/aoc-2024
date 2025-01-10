use day_24::Solver;

fn main() {
    let mut solver = Solver::load(include_str!("../../input1.txt"));
    solver.solve();
    println!("{}", solver.get_z());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let mut solver = Solver::load(include_str!("../../input0.txt"));
        solver.solve();
        assert_eq!(solver.get_z(), 2024);
    }
}
