use std::collections::HashMap;

type Op = fn(bool, bool) -> bool;

fn and(a: bool, b: bool) -> bool {
    a && b
}
fn or(a: bool, b: bool) -> bool {
    a || b
}
fn xor(a: bool, b: bool) -> bool {
    a ^ b
}

#[derive(Debug, Clone)]
pub struct Solver {
    registers: HashMap<String, bool>,
    asm: HashMap<String, (String, String, Op)>,
}

impl Solver {
    pub fn load(input: &str) -> Self {
        let (reg_input, asm_input) = input.split_once("\n\n").unwrap();

        let mut registers = HashMap::new();
        let mut asm = HashMap::new();

        for line in reg_input.lines() {
            let (name, val) = line.split_once(": ").unwrap();
            registers.insert(name.to_string(), val == "1");
        }

        for line in asm_input.lines() {
            let line = line.replace(" ->", "");
            let asm_line: Vec<_> = line.split(" ").collect();
            assert_eq!(asm_line.len(), 4);

            let op = match asm_line[1] {
                "AND" => and,
                "OR" => or,
                "XOR" => xor,
                _ => unreachable!(),
            };
            let a = asm_line[0].to_string();
            let b = asm_line[2].to_string();
            let out = asm_line[3].to_string();

            asm.insert(out, (a, b, op));
        }

        Self { registers, asm }
    }

    pub fn solve(&mut self) -> usize {
        let regs = &mut self.registers;

        loop {
            let mut changed = false;

            for (out, (a, b, op)) in &self.asm {
                if let (Some(&a), Some(&b), None) = (regs.get(a), regs.get(b), regs.get(out)) {
                    changed = true;
                    regs.insert(out.to_string(), op(a, b));
                }
            }

            if !changed {
                break;
            }
        }

        let z_regs: String = {
            let mut sorted_keys: Vec<_> = self.registers.keys().cloned().collect();
            sorted_keys.sort(); // Sort the keys alphabetically

            sorted_keys
                .into_iter()
                .filter_map(|k| {
                    if let Some('z') = k.chars().nth(0) {
                        self.registers
                            .get(&k)
                            .map(|&val| if val { '1' } else { '0' })
                    } else {
                        None
                    }
                })
                .rev()
                .collect()
        };

        usize::from_str_radix(&z_regs, 2).unwrap()
    }
}
