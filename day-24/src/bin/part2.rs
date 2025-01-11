use day_24::*;

fn main() {
    //let mut solver = Solver::load(include_str!("../../input1.txt"));
    //solver.solve();

    //println!("{}", solver.untangle());
    let a = edge::Asm::parse(include_str!("../../input1.txt"));
    println!("{}", a.solve());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let mut solver = Solver::load(include_str!("../../input1.txt"));
        solver.solve();
        assert_eq!(solver.untangle(), "cqr,ncd,nfj,qnw,vkg,z15,z20,z37");
    }
}
