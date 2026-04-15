use std::ops::RangeInclusive;

use regex::Regex;

use crate::{defs::Solution, utils::strings::StringChunksIter};

pub struct Day2Solution {
    input: String,
}

impl Solution<i64, i64> for Day2Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        self.get_ranges()
            .into_iter()
            .map(|r| r.find_invalids_p1())
            .sum()
    }

    fn get_part_2_solution(&self) -> i64 {
        self.get_ranges()
            .into_iter()
            .map(|r| r.find_invalids_p2())
            .sum()
    }
}

impl Day2Solution {
    pub fn get_ranges(&self) -> Vec<QRange> {
        let reg = Regex::new(r"(\d+)\-(\d+)").unwrap();
        let mut ranges = Vec::new();
        for range in self.input.split(',') {
            let Some(caps) = reg.captures(range) else {
                continue;
            };
            let lower_s = &caps[1];
            let lower = lower_s.parse().unwrap();
            let upper_s = &caps[2];
            let upper = upper_s.parse().unwrap();

            ranges.push(QRange {
                lower_s: lower_s.to_string(),
                upper_s: upper_s.to_string(),
                lower,
                upper,
            });
        }

        ranges
    }
}

#[derive(Debug)]
pub struct QRange {
    lower: i64,
    upper: i64,
    lower_s: String,
    upper_s: String,
}

impl QRange {
    pub fn find_invalids_p1(&self) -> i64 {
        let mut invalids = 0;
        for i in self.lower..=self.upper {
            if i < 10 {
                continue;
            }
            let i_s = i.to_string();
            let i_l = i_s.len();
            if i_l % 2 == 1 {
                continue;
            }

            let mid = i_l / 2;
            let mut c_i = i_s.chars();
            let first_slice = c_i.clone().take(mid).collect::<String>();
            let second_slice = c_i.skip(mid).take(mid).collect::<String>();
            if first_slice == second_slice {
                invalids += i;
            }
        }

        invalids
    }

    pub fn find_invalids_p2(&self) -> i64 {
        let mut invalids = 0;

        for i in self.lower..=self.upper {
            if i < 10 {
                continue;
            }

            let i_s = i.to_string();
            let i_l = i_s.len();

            let mut is_invalid = false;
            for j in 1..=(i_l / 2) {
                let num_chunks = i_l / j;
                if num_chunks <= 1 {
                    continue;
                }
                let s = i_s.chars().take(j).collect::<String>();
                is_invalid = StringChunksIter::new(&i_s, j).all(|c| c == s);

                if is_invalid {
                    break;
                }
            }

            if is_invalid {
                invalids += i;
            }
        }

        invalids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2() {
        let range = QRange {
            lower: 10,
            upper: 12,
            lower_s: "10".into(),
            upper_s: "12".into(),
        };

        assert_eq!(range.find_invalids_p2(), 11);
    }
}
