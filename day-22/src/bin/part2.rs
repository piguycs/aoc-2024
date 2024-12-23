#![allow(unused)]

use itertools::Itertools;

use day_22::*;

fn main() {
    let input = include_str!("../../input.txt");
    let nums = get_nums(input);

    for num in nums {
        let mut secret = Secret::new(num);

        let result = secret
            .take(2000)
            .map(|num| num % 10)
            .tuple_windows()
            .map(|(prev, curr)| (curr as isize - prev as isize, curr))
            .tuple_windows()
            .map(|(a, b, c, d)| {})
            .collect_vec();
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
