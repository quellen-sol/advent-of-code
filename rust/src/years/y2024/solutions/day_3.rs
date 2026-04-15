use regex::Regex;

use crate::defs::Solution;

pub struct Day3Solution {
    input: String,
}

impl Solution<i32, i32> for Day3Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i32 {
        let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut sum = 0;
        for (_, [a, b]) in r.captures_iter(self.get_input()).map(|caps| caps.extract()) {
            let a: i32 = a.parse().unwrap();
            let b: i32 = b.parse().unwrap();

            let r = a * b;
            sum += r;
        }

        sum
    }

    fn get_part_2_solution(&self) -> i32 {
        let mut enabled = true;
        let mut sum = 0;
        let cmd_reg = Regex::new(r"((do|don't|mul)\(([\d,]*)\))").unwrap();
        for x in cmd_reg.captures_iter(self.get_input()) {
            let cmd = x.get(2).expect("Bad regex").as_str();
            let nums = x.get(3).map(|m| m.as_str());

            match cmd {
                "do" => {
                    enabled = true;
                }
                "don't" => {
                    enabled = false;
                }
                "mul" => {
                    if !enabled {
                        continue;
                    }
                    let nums = nums.unwrap();

                    // RUST BUG (TODO: Report if not already)
                    let result = nums
                        .trim()
                        .split(',')
                        .map(|v| v.parse::<i32>().unwrap())
                        .product::<i32>();

                    sum += result;
                }
                _ => {
                    continue;
                }
            }
        }

        sum
    }
}
