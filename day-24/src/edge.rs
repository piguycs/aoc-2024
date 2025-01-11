// fixed edge case
#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;

pub const LAST_Z: &str = "z45";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn op(&self) -> fn(bool, bool) -> bool {
        match self {
            Op::And => |a, b| a && b,
            Op::Or => |a, b| a || b,
            Op::Xor => |a, b| a ^ b,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Formula {
    a: String,
    b: String,
    out: String,
    op: Op,
}

impl Formula {
    pub fn read(e: &str) -> Self {
        let mut iter = e.split_whitespace().tuples();
        let (a, op, b, _, out) = iter.next().unwrap();

        let op = match op {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => unreachable!(),
        };

        Self {
            a: a.to_string(),
            b: b.to_string(),
            out: out.to_string(),
            op,
        }
    }

    // If the output of a gate is z, then the operation has to be XOR unless it is the last bit
    fn rule_1(&self) -> bool {
        if self.out.starts_with("z") && self.out != LAST_Z {
            self.op == Op::Xor
        } else {
            true
        }
    }

    // If the output of a gate is not z and the inputs are not x, y then it has to be AND / OR, but not XOR
    #[allow(clippy::nonminimal_bool)] // either auto fix it or dont suggest me bs
    fn rule_2(&self) -> bool {
        if !self.out.starts_with("z")
            && !(self.a.starts_with("x") || self.a.starts_with("y"))
            && !(self.b.starts_with("x") || self.b.starts_with("y"))
        {
            self.op != Op::Xor
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asm {
    registers: HashMap<String, bool>,
    asm: Vec<Formula>,
}

impl Asm {
    pub fn parse(input: &str) -> Self {
        let (reg, actions) = input.split_once("\n\n").unwrap();

        let registers = reg
            .lines()
            .map(|e| e.split_once(": ").unwrap())
            .map(|(val, bin)| (val.to_string(), bin == "1"))
            .collect();

        let asm = actions.lines().map(Formula::read).collect();

        Self { registers, asm }
    }

    pub fn solve(&self) -> String {
        let mut r1_breakers = vec![];
        let mut r2_breakers = vec![];

        for formula in &self.asm {
            if !formula.rule_1() {
                r1_breakers.push(formula);
            } else if !formula.rule_2() {
                r2_breakers.push(formula);
            }
        }

        let mut r1_sorted = vec![];
        let mut r2_sorted = vec![];

        // r1b/r2b means rule1/2 breaker
        for r2b in &r2_breakers {
            let r1_out = self.follow_r2_breaker(&r2b.out).unwrap();
            if let Some(r1b) = r1_breakers.iter().find(|e| e.out == r1_out) {
                let mut n1 = r1b.to_owned().clone();
                n1.out = r2b.out.clone();
                r1_sorted.push(n1);

                let mut n2 = r2b.to_owned().clone();
                n2.out = r1b.out.clone();
                r2_sorted.push(n2);
            }
        }

        r1_sorted.append(&mut r2_sorted);
        r1_sorted.sort_by(|a, b| a.out.cmp(&b.out));

        let mut r3_breakers: Vec<_> = self
            .asm
            .iter()
            .filter(|form| {
                if (form.a.contains('x')
                    || form.b.contains('x')
                    || form.a.contains('y')
                    || form.b.contains('y'))
                    && form.op == Op::Xor
                {
                    !self.asm.iter().any(|other_form| {
                        (other_form.a == form.out || other_form.b == form.out)
                            && other_form.op == Op::Xor
                    })
                } else {
                    false
                }
            })
            .filter(|form| !form.a.contains("00"))
            .cloned()
            .collect();

        let mut r4_breakers: Vec<_> = self
            .asm
            .iter()
            .filter(|form| {
                if (form.a.contains('x')
                    || form.b.contains('x')
                    || form.a.contains('y')
                    || form.b.contains('y'))
                    && form.op == Op::And
                {
                    !self.asm.iter().any(|other_form| {
                        (other_form.a == form.out || other_form.b == form.out)
                            && other_form.op == Op::Or
                    })
                } else {
                    false
                }
            })
            .filter(|form| !form.a.contains("00"))
            .cloned()
            .collect();

        r1_sorted.append(&mut r3_breakers);
        r1_sorted.append(&mut r4_breakers);

        let out = r1_sorted
            .iter()
            .map(|e| e.out.clone())
            .unique()
            .sorted_by(|a, b| a.cmp(b))
            .join(",");

        out
    }

    fn follow_r2_breaker(&self, c: &str) -> Option<String> {
        let x: Vec<_> = self
            .asm
            .iter()
            .filter(|&form| form.a == c || form.b == c)
            .collect();

        if let Some(form) = x.iter().find(|&&form| form.out.starts_with('z')) {
            let index = form.out[1..].parse::<usize>().unwrap() - 1;
            return Some(format!("z{:02}", index));
        }

        for form in &x {
            if let Some(result) = self.follow_r2_breaker(&form.out) {
                return Some(result);
            }
        }

        None
    }
}
