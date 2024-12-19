use std::collections::HashMap;

pub fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let input: Vec<_> = input.split("\n\n").collect();

    let towel_str = input[0];
    let req_str = input[1];

    let towels = towel_str.split(", ").collect();
    let requirements = req_str.lines().collect();

    (towels, requirements)
}

pub fn find_match(towels: &[&str], needle: &str) -> bool {
    let mut memo = HashMap::new();
    let mut matches = HashMap::new();
    find_match_memo(towels, needle, &mut memo, &mut matches)
}

pub fn find_combinations(towels: &[&str], needle: &str) -> Option<usize> {
    let mut memo = HashMap::new();
    let mut matches = HashMap::new();
    find_match_memo(towels, needle, &mut memo, &mut matches);

    matches.get(needle).copied()
}

fn find_match_memo(
    towels: &[&str],
    needle: &str,
    memo: &mut HashMap<String, bool>,
    matches: &mut HashMap<String, usize>,
) -> bool {
    if let Some(&result) = memo.get(needle) {
        return result;
    }

    let mut count = 0;

    for &towel in towels {
        if let Some(trunc) = needle.strip_prefix(towel) {
            if trunc.is_empty() {
                count += 1;
            } else if find_match_memo(towels, trunc, memo, matches) {
                count += matches.get(trunc).copied().unwrap_or(0);
            }
        }
    }

    memo.insert(needle.to_string(), count > 0);
    matches.insert(needle.to_string(), count);

    count > 0
}
