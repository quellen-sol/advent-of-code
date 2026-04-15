use std::cmp::Ordering;

use itertools::Itertools;

use crate::defs::Solution;

pub struct Day2Solution {
    input: String,
}

impl Solution<i32, i32> for Day2Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i32 {
        self.get_lines()
            .map(|line| {
                let report = Report(line);
                report
            })
            .filter(|report| report.report_safe_p1())
            .count() as i32
    }

    fn get_part_2_solution(&self) -> i32 {
        self.get_lines()
            .map(|line| {
                let report = Report(line);
                report
            })
            .filter(|report| report.report_safe_p2(None))
            .count() as i32
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
enum MaybeBool {
    True,
    False,
    Maybe,
}

pub struct Report<'a>(&'a str);

impl<'a> Report<'a> {
    pub fn get_levels(&self) -> impl Iterator<Item = i32> + 'a {
        self.0.split(' ').map(|num| num.parse().unwrap())
    }

    pub fn report_safe_p1(&self) -> bool {
        let mut all_decreasing = MaybeBool::Maybe;
        let mut all_increasing = MaybeBool::Maybe;
        let levels = self.get_levels();
        let mut last_level: Option<i32> = None;

        for level in levels {
            if let Some(last) = last_level {
                let diff = (level - last).abs();
                if !(1..=3).contains(&diff) {
                    return false;
                }
                let ord = level.cmp(&last);
                match ord {
                    Ordering::Greater => {
                        if all_decreasing == MaybeBool::True {
                            return false;
                        }

                        all_increasing = MaybeBool::True;
                    }
                    Ordering::Less => {
                        if all_increasing == MaybeBool::True {
                            return false;
                        }

                        all_decreasing = MaybeBool::True;
                    }
                    Ordering::Equal => {
                        return false;
                    }
                }
            }

            last_level = Some(level);
        }

        true
    }

    pub fn report_safe_p2(&self, remove: Option<usize>) -> bool {
        let mut all_decreasing = MaybeBool::Maybe;
        let mut all_increasing = MaybeBool::Maybe;
        let levels = self.get_levels().collect_vec();
        let levels_len = levels.len();
        let levels = levels.into_iter();
        let mut last_level: Option<i32> = None;
        let mut will_remove = false;

        for (idx, level) in levels.enumerate() {
            if let Some(remove_idx) = remove {
                if remove_idx == idx {
                    continue;
                }
            }
            if let Some(last) = last_level {
                let diff = (level - last).abs();
                if !(1..=3).contains(&diff) {
                    // if we did remove, return hard false
                    // if so, return result of report + remove
                    will_remove = true;
                }
                let ord = level.cmp(&last);
                match ord {
                    Ordering::Greater => {
                        if all_decreasing == MaybeBool::True {
                            will_remove = true;
                        }

                        all_increasing = MaybeBool::True;
                    }
                    Ordering::Less => {
                        if all_increasing == MaybeBool::True {
                            will_remove = true;
                        }

                        all_decreasing = MaybeBool::True;
                    }
                    Ordering::Equal => {
                        will_remove = true;
                    }
                }
            }

            if will_remove {
                if remove.is_none() {
                    // Try removing one at all positions
                    return (0..levels_len).any(|i| self.report_safe_p2(Some(i)));
                } else {
                    // Already removed, so return false
                    return false;
                }
            }

            last_level = Some(level);
        }

        true
    }
}
