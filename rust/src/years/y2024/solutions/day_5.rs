use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

use crate::defs::Solution;

pub struct Day5Solution {
    input: String,
}

impl Solution<i32, i32> for Day5Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i32 {
        let (rules, reports) = self.get_sections();

        let mut hm = HashMap::new();
        for rule in rules.lines() {
            let v = rule
                .split('|')
                .map(|item| item.parse().unwrap())
                .collect_vec();
            let a: u8 = v[0];
            let b: u8 = v[1];

            let entry = hm.entry(b).or_insert(HashSet::new());
            entry.insert(a);
        }

        let mut valid_report_mids = vec![];

        for report in reports.lines() {
            let mut valid_report = true;
            let split_vec = report
                .trim()
                .split(',')
                .map(|n| n.parse::<u8>().unwrap())
                .collect_vec();
            let outer_iter = split_vec.iter().enumerate();

            for (idx, num) in outer_iter {
                let Some(hm_entry) = hm.get(num) else {
                    continue;
                };

                let inner_iter = split_vec.iter().skip(idx + 1);
                for cmp_num in inner_iter {
                    if hm_entry.contains(cmp_num) {
                        // num has broken a rule
                        valid_report = false;
                        break;
                    }
                }

                if !valid_report {
                    break;
                }
            }

            if valid_report {
                let len = split_vec.len();
                let mid_item = split_vec[len / 2];
                valid_report_mids.push(mid_item);
            }
        }

        valid_report_mids.into_iter().map(|v| v as i32).sum()
    }

    fn get_part_2_solution(&self) -> i32 {
        let (rules, reports) = self.get_sections();

        let mut hm = HashMap::new();
        for rule in rules.lines() {
            let v = rule
                .split('|')
                .map(|item| item.parse().unwrap())
                .collect_vec();
            let a: u8 = v[0];
            let b: u8 = v[1];

            let entry = hm.entry(b).or_insert(HashSet::new());
            entry.insert(a);
        }

        let mut mid_sum = 0;

        for report in reports.lines() {
            let mut use_this = false;
            let mut report_vec = report
                .trim()
                .split(',')
                .map(|n| {
                    // let ref_val = RefCell::new(n.parse::<u8>().unwrap());
                    // ref_val

                    n.parse::<u8>().unwrap()
                })
                .collect_vec();

            loop {
                let mut to_swap = None;
                for (o_idx, outer) in report_vec.iter().enumerate() {
                    let Some(entry) = hm.get(outer) else {
                        continue;
                    };
                    for (i_idx, inner) in report_vec.iter().enumerate().skip(o_idx + 1) {
                        if entry.contains(inner) {
                            use_this = true;
                            to_swap = Some((o_idx, i_idx));
                            break;
                        }
                    }

                    if to_swap.is_some() {
                        break;
                    }
                }

                if let Some((a_idx, b_idx)) = to_swap {
                    report_vec.swap(a_idx, b_idx);
                } else {
                    break;
                }
            }

            if use_this {
                mid_sum += (report_vec[report_vec.len() / 2] as i32);
            }
        }

        mid_sum
    }
}

impl Day5Solution {
    pub fn get_sections(&self) -> (&str, &str) {
        let mut s = self.input.split("\n\n");
        let one = s.next().unwrap();
        let two = s.next().unwrap();

        (one, two)
    }
}

pub struct Report(Vec<u8>);
