#![allow(unused)]

use day_09::process;

fn main() {
    let input = include_str!("../../input1.txt");
    let input = "2333133121414131402";
    process(input);
}

#[test]
fn test() {
    let input = "2333133121414131402";
    let out = 1928;
    //assert_eq!(process(input), out);
}
