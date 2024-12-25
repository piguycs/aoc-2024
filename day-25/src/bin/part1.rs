#![allow(unused)]

use day_25::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let data = parse(input);

    println!("{}", part1(data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[rstest::fixture]
    fn locks_and_keys() -> (Vec<Lock>, Vec<Key>) {
        let input = include_str!("../../input0.txt");
        parse(input)
    }

    #[rstest::rstest]
    fn test_parse(locks_and_keys: (Vec<Lock>, Vec<Key>)) {
        let (locks, keys) = locks_and_keys;

        assert_eq!(locks[0], (0, 5, 3, 4, 3));
        assert_eq!(locks[1], (1, 2, 0, 5, 3));

        assert_eq!(keys[0], (5, 0, 2, 1, 3));
        assert_eq!(keys[1], (4, 3, 4, 0, 2));
        assert_eq!(keys[2], (3, 0, 2, 0, 1));
    }

    #[rstest::rstest]
    fn test_outcome(locks_and_keys: (Vec<Lock>, Vec<Key>)) {
        assert_eq!(part1(locks_and_keys), 3);
    }
}
