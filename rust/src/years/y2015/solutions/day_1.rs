use crate::defs::Solution;

pub struct Day1Solution {
    input: String,
}

impl Solution<i64, i64> for Day1Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        self.get_input()
            .chars()
            .map(|v| match v {
                '(' => 1,
                ')' => -1,
                _ => panic!("Shouldn't be anything else"),
            })
            .sum()
    }

    fn get_part_2_solution(&self) -> i64 {
        let mut cnt = 0;
        for (i, c) in self.get_input().chars().enumerate() {
            match c {
                '(' => {
                    cnt += 1;
                }
                ')' => {
                    cnt -= 1;
                }
                _ => {
                    panic!("Not possible");
                }
            }

            if cnt < 0 {
                return i as i64 + 1;
            }
        }

        panic!("Should not be here?");
    }
}

impl Day1Solution {}
