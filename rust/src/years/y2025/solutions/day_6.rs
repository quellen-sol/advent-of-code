use crate::defs::Solution;

pub struct Day6Solution {
    input: String,
}

impl Solution<i64, i64> for Day6Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let mut total = 0;
        let mut values: Vec<Vec<i64>> = Vec::new();

        for line in self.get_lines() {
            for (i, item) in line.split_whitespace().enumerate() {
                let parse_a = item.parse::<i64>().ok();

                match parse_a {
                    Some(value) => {
                        let entry = values.get_mut(i);
                        match entry {
                            Some(v) => {
                                v.push(value);
                            }
                            None => {
                                let new = vec![value];
                                values.push(new);
                            }
                        }
                    }
                    None => {
                        // Op character
                        match item {
                            "*" => {
                                total += values[i].iter().product::<i64>();
                            }
                            "+" => {
                                total += values[i].iter().sum::<i64>();
                            }
                            _ => {
                                panic!("Not possible");
                            }
                        }
                    }
                }
            }
        }

        total
    }

    fn get_part_2_solution(&self) -> i64 {
        todo!()
    }
}

impl Day6Solution {
    pub fn build_right_left_values(values: Vec<i64>) -> Vec<i64> {
        let mut new_values = Vec::new();

        let max_len_iter = values.iter().map(|v| (*v as f64).log10() + 1.0);
        let mut max_digits = 1.0;
        for digit in max_len_iter {
            if digit > max_digits {
                max_digits = digit;
            }
        }

        new_values
    }
}
