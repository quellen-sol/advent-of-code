use std::collections::HashSet;

use regex::Regex;

use crate::defs::Solution;

pub struct Day5Solution {
    input: String,
    ranges: Vec<QRange>,
    values: Vec<i64>,
}

impl Solution<i64, i64> for Day5Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        let (ranges, values) = Self::build_inners(&input);
        Self {
            input,
            ranges,
            values,
        }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let mut fresh_ids = 0;
        let merged_ranges = self.get_merged_ranges();

        for value in self.values.iter() {
            if merged_ranges.iter().any(|r| r.contains_value(*value)) {
                fresh_ids += 1;
            }
        }

        fresh_ids
    }

    fn get_part_2_solution(&self) -> i64 {
        self.get_merged_ranges().iter().map(|r| r.size()).sum()
    }
}

impl Day5Solution {
    pub fn build_inners(input: &str) -> (Vec<QRange>, Vec<i64>) {
        let range_reg = Regex::new(r"(\d+)\-(\d+)").unwrap();
        let mut building_values = false;
        let mut ranges = Vec::new();
        let mut values = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                building_values = true;
                continue;
            }

            if !building_values {
                let caps = range_reg.captures(line).unwrap();
                let range = QRange {
                    lower: caps[1].parse().unwrap(),
                    upper: caps[2].parse().unwrap(),
                };
                ranges.push(range);
            } else {
                values.push(line.parse().unwrap());
            }
        }

        (ranges, values)
    }

    pub fn get_merged_ranges(&self) -> HashSet<QRange> {
        let mut ranges_copy = self
            .ranges
            .iter()
            .map(|r| r.clone())
            .collect::<HashSet<_>>();

        loop {
            let mut to_remove = Vec::new();
            let mut to_insert = None;
            for r_x in ranges_copy.iter() {
                for r_y in ranges_copy.iter() {
                    if r_x == r_y {
                        continue;
                    }

                    let merged = r_x.merge(r_y);
                    if let Some(merged) = merged {
                        to_remove.push(r_x.clone());
                        to_remove.push(r_y.clone());
                        to_insert.replace(merged);
                        break;
                    }
                }

                if to_insert.is_some() {
                    break;
                }
            }

            if to_remove.is_empty() {
                break;
            }

            for r in to_remove {
                ranges_copy.remove(&r);
            }

            if let Some(r) = to_insert {
                ranges_copy.insert(r);
            }
        }

        ranges_copy
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct QRange {
    lower: i64,
    upper: i64,
}

impl QRange {
    pub fn contains_value(&self, value: i64) -> bool {
        value >= self.lower && value <= self.upper
    }

    pub fn contains_range(&self, other: &Self) -> bool {
        self.contains_value(other.lower) || self.contains_value(other.upper)
    }

    pub fn merge(&self, other: &Self) -> Option<Self> {
        if !self.contains_range(other) {
            return None;
        }

        return Some(Self {
            lower: self.lower.min(other.lower),
            upper: self.upper.max(other.upper),
        });
    }

    pub fn size(&self) -> i64 {
        self.upper - self.lower + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merging_1() {
        let r1 = QRange {
            lower: 0,
            upper: 10,
        };

        let r2 = QRange {
            lower: 5,
            upper: 10,
        };

        let merged = r1.merge(&r2).unwrap();

        assert_eq!(merged.lower, 0);
        assert_eq!(merged.upper, 10);
    }

    #[test]
    fn merging_2() {
        let r1 = QRange {
            lower: 0,
            upper: 10,
        };

        let r2 = QRange {
            lower: 10,
            upper: 12,
        };

        let merged = r1.merge(&r2).unwrap();

        assert_eq!(merged.lower, 0);
        assert_eq!(merged.upper, 12);
    }

    #[test]
    fn merging_3() {
        let r1 = QRange {
            lower: 0,
            upper: 10,
        };

        let r2 = QRange {
            lower: 11,
            upper: 12,
        };

        let merged = r1.merge(&r2);

        assert!(merged.is_none());
    }
}
