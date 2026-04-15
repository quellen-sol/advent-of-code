use std::collections::HashMap;

use itertools::Itertools;

use crate::defs::Solution;

pub struct Day1Solution {
    input: String,
}

impl Solution<i32, i32> for Day1Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i32 {
        let (left_list, right_list, _) = self.get_lists();

        let mut total_distance = 0;
        for (left_num, right_num) in left_list.into_iter().zip(right_list.into_iter()) {
            let distance = (left_num - right_num).abs();
            total_distance += distance;
        }
        total_distance
    }

    fn get_part_2_solution(&self) -> i32 {
        let (left_list, _, similarities) = self.get_lists();

        left_list
            .iter()
            .map(|num| {
                let amt = similarities.get(num).unwrap_or(&0);

                num * amt
            })
            .sum()
    }
}

impl Day1Solution {
    fn get_lists(&self) -> (Vec<i32>, Vec<i32>, HashMap<i32, i32>) {
        let lines = self.get_lines();
        let mut left_list = Vec::new();
        let mut right_list = Vec::new();
        // similarities = how many times a left number appears in the right
        let mut similarities = HashMap::new();
        let items = lines.map(|l| {
            l.split("   ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect_vec()
        });

        for item in items {
            let left_num = item[0];
            let right_num = item[1];
            let similarity_entry = similarities.entry(right_num).or_insert(0);
            *similarity_entry += 1;
            left_list.push(left_num);
            right_list.push(right_num);
        }

        left_list.sort();
        right_list.sort();

        (left_list, right_list, similarities)
    }
}
