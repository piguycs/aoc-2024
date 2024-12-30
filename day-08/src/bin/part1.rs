use day_08::Antennas;

fn main() {
    let input = include_str!("../../input1.txt");

    let antennas = Antennas::parse(input);

    println!("{}", antennas.num_antinodes());
}
