#![allow(unused)]

use day_09::parse;

fn main() {
    let input = include_str!("../../input1.txt");
    parse(input);
}

#[test]
fn test() {
    let input = "2333133121414131402";
    let out = 1928;
    parse(input);
}
