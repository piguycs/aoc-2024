use glam::IVec2;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct Key {
    key: char,
    pos: IVec2,
}

impl Key {
    pub fn new(key: char, x: i32, y: i32) -> Self {
        Self {
            key,
            pos: IVec2 { x, y },
        }
    }
}

#[derive(Debug, Clone, Default)]
struct KeyPad {
    keys: HashMap<char, Key>,
}

impl KeyPad {
    pub fn new() -> Self {
        let mut pad = KeyPad::default();

        pad.insert('0', 3, 1);
        pad.insert('A', 3, 2);

        pad.insert('1', 2, 0);
        pad.insert('2', 2, 1);
        pad.insert('3', 2, 2);

        pad.insert('4', 1, 0);
        pad.insert('5', 1, 1);
        pad.insert('6', 1, 2);

        pad.insert('7', 0, 0);
        pad.insert('8', 0, 1);
        pad.insert('9', 0, 2);

        pad.insert('^', 0, 1);
        pad.insert('B', 0, 2);

        pad.insert('<', 1, 0);
        pad.insert('v', 1, 1);
        pad.insert('>', 1, 2);

        pad
    }

    pub fn insert(&mut self, key: char, x: i32, y: i32) {
        self.keys.insert(key, Key::new(key, x, y));
    }
}

#[derive(Debug, Clone, Default)]
struct Solution {
    graph: HashMap<char, Vec<char>>,
    path_memo: HashMap<String, Vec<String>>,
    keypad_map: HashMap<char, Key>,
}

impl Solution {
    fn calculate_len(&mut self, depth: usize) -> HashMap<String, usize> {
        let mut len_map = HashMap::new();

        let sources = ['>', 'v', '<', 'B', '^'];
        let targets = ['>', 'v', '<', 'B', '^'];

        for level in 1..=depth {
            for source in sources {
                for target in targets {
                    let key = Self::form_key(source, target, level as i32);
                    let moves = self.find_and_translate(source, target);

                    if level == 1 {
                        len_map.insert(key, moves[0].len());
                    } else {
                        let mut shortest_len = usize::MAX;
                        for m in &moves {
                            let mut source2 = 'B';
                            let mut move_len = 0;

                            for target2 in m.chars() {
                                let prev_key = Self::form_key(source2, target2, (level - 1) as i32);
                                move_len += len_map.get(&prev_key).unwrap();
                                source2 = target2;
                            }

                            shortest_len = shortest_len.min(move_len);
                        }

                        len_map.insert(key, shortest_len);
                    }
                }
            }
        }

        len_map
    }

    fn find_and_translate(&mut self, source: char, target: char) -> Vec<String> {
        let key = format!("{}{}", source, target);
        if let Some(moves) = self.path_memo.get(&key) {
            moves.clone()
        } else {
            let moves = self.translate_paths(self.find_all_paths(source, target));
            self.path_memo.insert(key, moves.clone());
            moves
        }
    }

    fn translate_paths(&self, paths: Vec<Vec<char>>) -> Vec<String> {
        let mut list = vec![];

        for path in paths {
            if path.is_empty() {
                continue;
            }

            let mut source = path[0];
            let mut moves = String::new();

            for target in path.iter().skip(1) {
                moves.push(self.get_moves(source, *target));
                source = *target;
            }

            moves.push('B');
            list.push(moves);
        }

        list
    }

    #[allow(clippy::comparison_chain, clippy::needless_return)]
    fn get_moves(&self, source: char, target: char) -> char {
        let source_keypad = self.keypad_map.get(&source).unwrap();
        let target_keypad = self.keypad_map.get(&target).unwrap();

        let horizontal = target_keypad.pos.y - source_keypad.pos.y;
        if horizontal > 0 {
            return '>';
        } else if horizontal < 0 {
            return '<';
        }

        let vertical = target_keypad.pos.x - source_keypad.pos.x;
        if vertical > 0 {
            return 'v';
        } else {
            return '^';
        }
    }

    fn find_all_paths(&self, start: char, end: char) -> Vec<Vec<char>> {
        let mut all_paths = vec![];
        let mut current_path = vec![];
        let mut visited = HashMap::new();

        self.dfs(start, end, &mut visited, &mut current_path, &mut all_paths);

        let shortest_len = all_paths
            .iter()
            .map(|path| path.len())
            .min()
            .unwrap_or(usize::MAX);

        all_paths
            .into_iter()
            .filter(|path| path.len() == shortest_len)
            .collect()
    }

    fn dfs(
        &self,
        current: char,
        end: char,
        visited: &mut HashMap<char, bool>,
        current_path: &mut Vec<char>,
        all_paths: &mut Vec<Vec<char>>,
    ) {
        current_path.push(current);
        visited.insert(current, true);

        if current == end {
            all_paths.push(current_path.clone());
        } else if let Some(neighbors) = self.graph.get(&current) {
            for &neighbor in neighbors {
                if !visited.get(&neighbor).unwrap_or(&false) {
                    self.dfs(neighbor, end, visited, current_path, all_paths);
                }
            }
        }

        current_path.pop();
        visited.insert(current, false);
    }

    fn form_key(source: char, target: char, level: i32) -> String {
        format!("{}{}{:02}", source, target, level)
    }
}

pub fn solve(data: Vec<&str>, depth: usize) -> usize {
    let depth = depth + 1;

    let keypad = KeyPad::new();

    let mut graph = HashMap::new();
    graph.insert('A', vec!['0', '3']);
    graph.insert('0', vec!['A', '2']);
    graph.insert('3', vec!['A', '2', '6']);
    graph.insert('2', vec!['1', '5', '3', '0']);
    graph.insert('1', vec!['4', '2']);
    graph.insert('6', vec!['9', '5', '3']);
    graph.insert('5', vec!['8', '4', '2', '6']);
    graph.insert('4', vec!['7', '5', '1']);
    graph.insert('9', vec!['8', '6']);
    graph.insert('8', vec!['7', '5', '9']);
    graph.insert('7', vec!['4', '8']);
    graph.insert('>', vec!['B', 'v']);
    graph.insert('v', vec!['>', '^', '<']);
    graph.insert('<', vec!['v']);
    graph.insert('B', vec!['^', '>']);
    graph.insert('^', vec!['B', 'v']);

    let mut solution = Solution {
        graph,
        path_memo: HashMap::default(),
        keypad_map: keypad.keys,
    };

    let len_map = solution.calculate_len(depth);
    let mut source = 'A';
    let mut total = 0;

    for (i, data) in data.iter().enumerate() {
        let number: usize = data[0..3].parse().expect("Invalid number format");
        let mut total_length = 0;

        for target in data.chars() {
            let moves = solution.find_and_translate(source, target);
            let mut shortest_length = usize::MAX;

            for move_seq in moves {
                let mut move_length = 0;
                let mut source2 = 'B';

                for target2 in move_seq.chars() {
                    let key = Solution::form_key(source2, target2, (depth - 1) as i32);
                    move_length += len_map.get(&key).expect("Key not found in length map");
                    source2 = target2;
                }

                shortest_length = shortest_length.min(move_length);
            }

            println!(
                "source={} target={} shortestLength={}",
                source, target, shortest_length
            );
            total_length += shortest_length;
            source = target;
        }

        println!("dataArray[{}]={} totalLength = {}", i, data, total_length);
        total += number * total_length;
    }

    total
}
