use day_22::*;

fn main() {
    let input = include_str!("../../input.txt");
    let nums = get_nums(input);

    let sum: usize = nums
        .iter()
        .map(|&num| {
            let secret = Secret::new(num);
            secret.take(2000).last().expect("infaliable")
        })
        .sum();

    println!("part1: {sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[rstest::rstest]
    #[case(1, 8685429)]
    #[case(10, 4700978)]
    #[case(100, 15273692)]
    #[case(2024, 8667524)]
    fn test_secret(#[case] input: usize, #[case] expect: usize) {
        let secret = Secret::new(input);
        assert_eq!(Some(expect), secret.take(2000).last());
    }
}
