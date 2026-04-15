use std::f32::consts::E;

use itertools::Itertools;
use regex::Regex;

use crate::defs::Solution;

pub struct Day7Solution {
    input: String,
}

impl Solution<i64, i64> for Day7Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let r = Regex::new(r"(\d+):\s(.*)").unwrap();
        let mut acc = 0;
        for line in self.get_lines() {
            let cap = r.captures(line).unwrap();
            let val = &cap[1];
            let nums = &cap[2];

            let value = val.parse().unwrap();
            let nums = nums
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|v| v.parse().unwrap())
                .collect_vec();

            let equation = Equation { value, nums };

            if equation.is_possible_p1() {
                acc += equation.value;
            }
        }

        acc
    }

    fn get_part_2_solution(&self) -> i64 {
        let r = Regex::new(r"(\d+):\s(.*)").unwrap();
        let mut acc = 0;
        for line in self.get_lines() {
            let cap = r.captures(line).unwrap();
            let val = &cap[1];
            let nums = &cap[2];

            let value = val.parse().unwrap();
            let nums = nums
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|v| v.parse().unwrap())
                .collect_vec();

            let equation = Equation { value, nums };

            if equation.is_possible_p2() {
                acc += equation.value;
            }
        }

        acc
    }
}

impl Day7Solution {}

struct Equation {
    value: i64,
    nums: Vec<i64>,
}

impl Equation {
    pub fn is_possible_p1(&self) -> bool {
        let operations = self.nums.len() - 1;
        if operations == 0 && self.value == self.nums[0] {
            return true;
        }
        let mut op_vec = vec![];
        for ops in 0..operations {
            op_vec.push('+');
        }

        'op_loop: loop {
            let mut acc = 0;
            for (i, v) in self.nums.iter().enumerate() {
                if i == 0 {
                    acc = *v;
                    continue;
                }

                let op_i = i - 1;
                let op = op_vec[op_i];

                match op {
                    '+' => {
                        acc += *v;
                    }
                    '*' => {
                        acc *= *v;
                    }
                    _ => {
                        panic!("lol")
                    }
                }
            }

            if self.value == acc {
                return true;
            }

            let mut mut_idx = 0;
            loop {
                let char_at = op_vec[mut_idx];
                if char_at == '*' {
                    mut_idx += 1;
                    if mut_idx >= op_vec.len() {
                        // Done, we tried all combinations of operators
                        break 'op_loop;
                    }
                } else {
                    op_vec[mut_idx] = '*';
                    if mut_idx > 0 {
                        for i in 0..mut_idx {
                            op_vec[i] = '+';
                        }
                    }
                    break;
                }
            }
        }

        false
    }

    pub fn is_possible_p2(&self) -> bool {
        let operations = self.nums.len() - 1;
        if operations == 0 && self.value == self.nums[0] {
            return true;
        }
        let mut op_vec = vec![];
        for ops in 0..operations {
            op_vec.push("+");
        }

        'op_loop: loop {
            let mut acc = 0;
            for (i, v) in self.nums.iter().enumerate() {
                if i == 0 {
                    acc = *v;
                    continue;
                }

                let op_i = i - 1;
                let op = op_vec[op_i];

                match op {
                    "+" => {
                        acc += *v;
                    }
                    "*" => {
                        acc *= *v;
                    }
                    "||" => {
                        acc = format!("{acc}{v}").parse().unwrap();
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }

            if self.value == acc {
                return true;
            }

            let mut mut_idx = 0;
            loop {
                let char_at = op_vec[mut_idx];
                if char_at == "*" {
                    mut_idx += 1;
                    if mut_idx >= op_vec.len() {
                        // Done, we tried all combinations of operators
                        break 'op_loop;
                    }
                } else {
                    let new_char = match char_at {
                        "+" => "||",
                        "||" => "*",
                        _ => unreachable!(),
                    };
                    op_vec[mut_idx] = new_char;
                    if mut_idx > 0 {
                        for i in 0..mut_idx {
                            op_vec[i] = "+";
                        }
                    }
                    break;
                }
            }
        }

        false
    }
}
