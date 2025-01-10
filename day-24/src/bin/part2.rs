use day_24::Solver;

fn main() {
    let mut solver = Solver::load(include_str!("../../input1.txt"));
    solver.solve();

    println!("{}", solver.untangle());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let mut solver = Solver::load(include_str!("../../input0.txt"));
        solver.solve();
        assert_eq!(
            solver.untangle(),
            "mjb,tgd,wpb,z02,z03,z05,z06,z07,z08,z10,z11"
        );
    }
}
