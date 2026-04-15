use std::collections::HashSet;

use crate::defs::Solution;

pub struct Day3Solution {
    input: String,
}

impl Solution<i32, u128> for Day3Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i32 {
        let mut jotalges = Vec::new();
        for line in self.get_lines() {
            let mut maxes: Vec<(i32, Option<i32>)> = Vec::new();
            for (i, x) in line
                .chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .enumerate()
            {
                for m in maxes.iter_mut() {
                    match m.1.as_mut() {
                        Some(v) => {
                            if x > *v {
                                *v = x;
                            }
                        }
                        None => {
                            m.1.replace(x);
                        }
                    }
                }

                maxes.push((x, None));
            }

            let max_joltage = maxes
                .iter()
                .filter_map(|m| {
                    let (first_dig, second_dig) = (m.0, m.1?);
                    Some(first_dig * 10 + second_dig)
                })
                .max();

            let Some(max_joltage) = max_joltage else {
                continue;
            };

            jotalges.push(max_joltage);
        }

        jotalges.iter().sum()
    }

    fn get_part_2_solution(&self) -> u128 {
        let mut total_joltage = 0;
        for line in self.get_lines() {
            let len = line.len();
            for (i, c) in line.chars().enumerate() {}
        }

        total_joltage
    }
}

impl Day3Solution {}
