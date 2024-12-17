use std::sync::LazyLock;

use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use regex::Regex;

const PATTERN: &str =
    r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ((\d,?)+)";

static REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(PATTERN).expect("invalid regex provided"));

#[derive(Debug, Clone, Copy, FromPrimitive)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug, Clone, Copy)]
enum ExecRes {
    Jmp,
    Print(usize),
    None,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub reg_a: usize,
    pub reg_b: usize,
    pub reg_c: usize,

    // instruction_counter, or as I call it: Program Counter (pc)
    pub pc: usize,

    pub code: Vec<usize>,
}

impl Program {
    pub fn parse(input: &str) -> Option<Self> {
        let cap = REGEX.captures(input)?;

        let reg_a = cap.get(1)?.as_str().parse::<usize>().ok()?;
        let reg_b = cap.get(2)?.as_str().parse::<usize>().ok()?;
        let reg_c = cap.get(3)?.as_str().parse::<usize>().ok()?;

        let code = cap
            .get(4)?
            .as_str()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|c| c as usize)
            .collect_vec();

        Some(Self {
            reg_a,
            reg_b,
            reg_c,

            pc: 0,

            code,
        })
    }

    fn fetch(&self) -> Option<(usize, usize)> {
        let opcode = *self.code.get(self.pc)?;
        let operand = *self.code.get(self.pc + 1)?;

        Some((opcode, operand))
    }

    fn decode(&self, opcode: usize) -> Opcode {
        Opcode::from_usize(opcode).expect("invalid opcode")
    }

    fn get_combo_operand(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("invalid combo operajd"),
        }
    }

    fn execute(&mut self, opcode: Opcode, operand: usize) -> ExecRes {
        use crate::Opcode::*; // makes it easier to write rules

        match opcode {
            Adv => {
                let op2 = self.get_combo_operand(operand);
                let ans = self.reg_a / 2usize.pow(op2 as u32);

                self.reg_a = ans;
            }
            Bxl => self.reg_b ^= operand,
            Bst => {
                let op2 = self.get_combo_operand(operand);
                self.reg_b = op2 % 8;
            }
            Jnz => {
                if self.reg_a > 0 {
                    self.pc = operand;
                    return ExecRes::Jmp;
                }
            }
            Bxc => self.reg_b ^= self.reg_c,
            Out => {
                let op2 = self.get_combo_operand(operand);
                let ans = op2 % 8;
                return ExecRes::Print(ans);
            }
            Bdv => {
                let op2 = self.get_combo_operand(operand);
                let ans = self.reg_a / 2usize.pow(op2 as u32);

                self.reg_b = ans;
            }
            Cdv => {
                let op2 = self.get_combo_operand(operand);
                let ans = self.reg_a / 2usize.pow(op2 as u32);

                self.reg_c = ans;
            }
        }

        ExecRes::None
    }

    pub fn run(&mut self) -> Vec<usize> {
        let mut out_arr = vec![];

        while let Some((opcode_num, operand)) = self.fetch() {
            let opcode = self.decode(opcode_num);

            match self.execute(opcode, operand) {
                ExecRes::Print(out) => {
                    out_arr.push(out);
                    self.pc += 2
                }
                ExecRes::None => self.pc += 2,
                _ => (),
            }
        }

        out_arr
    }
}
