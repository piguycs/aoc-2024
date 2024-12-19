use day_19::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let data = parse(input);

    let mut count = 0;

    for needle in data.1 {
        if let Some(combinations) = find_combinations(&data.0, needle) {
            count += combinations;
        }
    }

    println!("part2: {count}");
}
