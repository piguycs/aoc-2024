use day_23::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let groups = groups(parse(input));

    println!("len: {}", count_containing(groups, "t"));
}

#[test]
fn test() {
    let input = include_str!("../../input0.txt");
    let connections = parse(input);

    assert_eq!(connections.len(), 32);
    assert_eq!(connections[0].0, "kh");
    assert_eq!(connections[0].1, "tc");

    let groups = groups(connections);

    assert_eq!(groups.len(), 12);
    assert_eq!(count_containing(groups, "t"), 7);
}
