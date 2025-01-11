#![feature(let_chains)]
pub mod edge;

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

    pub fn solve(&mut self) {
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
    }

    fn get(&self, c: char) -> usize {
        let z_regs: String = {
            let mut sorted_keys: Vec<_> = self.registers.keys().cloned().collect();
            sorted_keys.sort(); // Sort the keys alphabetically

            sorted_keys
                .into_iter()
                .filter_map(|k| {
                    if let Some(wire) = k.chars().nth(0)
                        && wire == c
                    {
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

    pub fn get_z(&self) -> usize {
        self.get('z')
    }

    // https://www.reddit.com/r/adventofcode/comments/1hla5ql/2024_day_24_part_2_a_guide_on_the_idea_behind_the/
    pub fn untangle(&self) -> String {
        // Find non-XOR gates that output to z, except the last one
        let nxz: Vec<_> = self
            .asm
            .iter()
            .filter(|(out, (_, _, op))| {
                out.starts_with('z') && *out != "z45" && (*op as usize) != (xor as usize)
            })
            .collect();

        // Find XOR gates that don't use x/y inputs and don't output to z
        let xnz: Vec<_> = self
            .asm
            .iter()
            .filter(|(out, (a, b, op))| {
                !out.starts_with('z')
                    && !a.starts_with('x')
                    && !a.starts_with('y')
                    && !b.starts_with('x')
                    && !b.starts_with('y')
                    && (*op as usize) == (xor as usize)
            })
            .collect();

        let z_val = self.get_z();

        // Calculate number of trailing zeros
        let x_val = self.get('x');
        let y_val = self.get('y');
        let false_carry = ((x_val + y_val) ^ z_val).trailing_zeros().to_string();

        // Collect and sort results
        let mut results: Vec<String> = Vec::new();

        // Add nxz outputs
        results.extend(nxz.into_iter().map(|(out, _)| out.clone()));

        // Add xnz outputs
        results.extend(xnz.into_iter().map(|(out, _)| out.clone()));

        // Add gates with matching false_carry
        results.extend(
            self.asm
                .iter()
                .filter(|(_, (a, b, _))| a.ends_with(&false_carry) && b.ends_with(&false_carry))
                .map(|(out, _)| out.clone()),
        );

        results.sort();
        results.join(",")
    }
}
