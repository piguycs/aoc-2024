#![allow(unused)]

use std::collections::HashMap;

enum Gate {
    And,
    Or,
    Xor,
}

struct Operation {
    in1: String,
    in2: String,
    out: String,
    op: Gate,
}

impl Operation {
    fn parse(i: &str) -> Self {
        // "ntg XOR fgs -> mjb";
        // 0    1   2   3  4
        let splits: Vec<_> = i.splitn(5, " ").collect();

        let in1 = splits[0].to_string();
        let in2 = splits[2].to_string();

        let out = splits[4].to_string();

        let op = match splits[1] {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!(),
        };

        Self { in1, in2, out, op }
    }
}

struct Data {
    gates: HashMap<String, usize>,
    ops: Vec<Operation>,
}

impl Data {
    fn parse(input: &str) -> Self {
        let sections: Vec<_> = input.splitn(2, "\n\n").collect();

        let gates = sections[0]
            .lines()
            .map(|e| e.splitn(2, ": ").collect::<Vec<_>>())
            .map(|split| (split[0].to_string(), split[1].parse::<usize>().unwrap()))
            .collect();

        let ops = sections[1].lines().map(Operation::parse).collect();

        Data { gates, ops }
    }

    fn calculate(&mut self) -> usize {
        for Operation { in1, in2, out, op } in &self.ops {
            self.gates.entry(in1.to_string()).or_insert_with(|| 1);
            //
        }

        0
    }
}

fn main() {
    let input = include_str!("../../input0.txt");
    let mut data = Data::parse(input);
    data.calculate();
}
