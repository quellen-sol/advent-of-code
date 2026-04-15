use crate::defs::Solution;

pub struct Day5Solution {
    input: String,
}

impl Solution<i64, i64> for Day5Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        todo!()
    }

    fn get_part_2_solution(&self) -> i64 {
        todo!()
    }
}

impl Day5Solution {}
