use day_21::*;

fn main() {
    let input = include_str!("../../input0.txt");
    parse(input);

    let keypad = Keypad::default();
    let path = keypad.find(29);

    let dirpad = DirPad::default();
    dirpad.find(path.0);
}

#[test]
fn test() {
    let input = include_str!("../../input0.txt");
    let nums = parse(input);

    assert_eq!(nums[0], 29);

    let keypad = Keypad::default();
    let path = keypad.find(29);

    let dirpad = DirPad::default();
    dirpad.find(path.0);

    assert_eq!(path.1, 8);
}
