use day_21::*;

fn main() {
    let input = include_str!("../../input0.txt");
    let nums = parse(input);

    let keypad = Keypad::default();
    let dirpad = DirPad::default();

    let (paths, score) = keypad.find(nums[0]);
    for path in paths {
        dirpad.parse_path((path, score));
    }
}

#[test]
fn test() {
    let input = include_str!("../../input0.txt");
    let nums = parse(input);

    assert_eq!(nums[0], 29);

    let keypad = Keypad::default();
    let path = keypad.find(29);

    dbg!(path);

    todo!();
}
