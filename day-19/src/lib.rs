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
    find_match_memo(towels, needle, &mut memo)
}

fn find_match_memo(towels: &[&str], needle: &str, memo: &mut HashMap<String, bool>) -> bool {
    if let Some(&result) = memo.get(needle) {
        return result;
    }

    for &towel in towels {
        if let Some(trunc) = needle.strip_prefix(towel) {
            if trunc.is_empty() || find_match_memo(towels, trunc, memo) {
                memo.insert(needle.to_string(), true);
                return true;
            }
        }
    }

    memo.insert(needle.to_string(), false);
    false
}
