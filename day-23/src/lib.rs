use itertools::Itertools;

use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|e| e.split("-"))
        .map(|mut e| (e.next().unwrap(), e.next().unwrap())) // infailable
        .collect_vec()
}

pub fn groups(connections: Vec<(&str, &str)>) -> HashSet<(String, String, String)> {
    let mut map = HashMap::new();

    for (h1, h2) in connections {
        map.entry(h1).or_insert(vec![]).push(h2);
        map.entry(h2).or_insert(vec![]).push(h1);
    }

    let mut groups = HashSet::new();

    for (host1, val) in &map {
        for i in 0..val.len() {
            for j in i + 1..val.len() {
                let host2 = &val[i];
                let host3 = &val[j];

                if map.get(host2).is_some_and(|n| n.contains(host3)) {
                    let mut triplet = [host1, host2, host3];
                    triplet.sort(); // deterministic output for tests
                    groups.insert((
                        triplet[0].to_string(),
                        triplet[1].to_string(),
                        triplet[2].to_string(),
                    ));
                }
            }
        }
    }

    groups
}

pub fn count_containing(groups: HashSet<(String, String, String)>, containing: &str) -> usize {
    groups
        .iter()
        .filter(|(h1, h2, h3)| {
            h1.starts_with(containing) || h2.starts_with(containing) || h3.starts_with(containing)
        })
        .count()
}
