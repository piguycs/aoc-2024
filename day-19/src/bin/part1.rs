use day_19::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let data = parse(input);

    let mut count = 0;

    for needle in data.1 {
        if find_match(&data.0, needle) {
            count += 1;
        }
    }

    println!("part1: {count}");
}

#[test]
fn test() {
    let input = include_str!("../../input0.txt");
    let data = parse(input);

    assert_eq!(data.0.len(), 8);
    assert_eq!(data.1.len(), 8);

    let negatives = [4, 7];

    for i in 0..data.1.len() {
        let needle = data.1[i];
        if negatives.contains(&i) {
            assert!(
                !find_match(&data.0, needle),
                "n{i} {} should not be found",
                needle
            );
        } else {
            assert!(
                find_match(&data.0, needle),
                "n{i} {} should be found",
                needle
            );
        }
    }
}
