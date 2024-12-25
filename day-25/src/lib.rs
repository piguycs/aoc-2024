use std::collections::HashSet;

const NUM_COLUMNS: usize = 5;

pub type Lock = (usize, usize, usize, usize, usize);
pub type Key = (usize, usize, usize, usize, usize);

pub fn part1(data: (Vec<Lock>, Vec<Key>)) -> usize {
    let (locks, keys) = data;

    let mut unique_combo = HashSet::<(Lock, Key)>::new();

    for &lock in &locks {
        for &key in &keys {
            let (a, b, c, d, e) = check_fits(lock, key);
            let fits = [a, b, c, d, e].iter().all(|&e| e <= 5);

            if fits {
                unique_combo.insert((lock, key));
            }
        }
    }

    unique_combo.len()
}

pub fn check_fits(lock: Lock, key: Key) -> (usize, usize, usize, usize, usize) {
    let (l1, l2, l3, l4, l5) = lock;
    let (k1, k2, k3, k4, k5) = key;

    (l1 + k1, l2 + k2, l3 + k3, l4 + k4, l5 + k5)
}

pub fn parse(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let (locks, keys): (Vec<_>, Vec<_>) = input.split("\n\n").partition(|&pattern| {
        if let Some(first) = pattern.lines().next() {
            if first.chars().all(|c| c == '#') {
                return true;
            }
        }

        if let Some(last) = pattern.lines().last() {
            if last.chars().all(|c| c == '#') {
                return false;
            }
        }

        unreachable!("neither lock or key pattern found\npattern: \n{pattern}");
    });

    let locks: Vec<_> = locks
        .iter()
        .map(|&lock| {
            let count: Vec<usize> = (0..NUM_COLUMNS)
                .map(|col| {
                    lock.lines()
                        .filter(|row| row.chars().nth(col) == Some('#'))
                        .count()
                        - 1
                })
                .collect();

            (count[0], count[1], count[2], count[3], count[4])
        })
        .collect();

    let keys: Vec<_> = keys
        .iter()
        .map(|&keys| {
            let count: Vec<usize> = (0..NUM_COLUMNS)
                .map(|col| {
                    keys.lines()
                        .filter(|row| row.chars().nth(col) == Some('#'))
                        .count()
                        - 1
                })
                .collect();

            (count[0], count[1], count[2], count[3], count[4])
        })
        .collect();

    (locks, keys)
}
