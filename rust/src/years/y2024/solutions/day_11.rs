use std::{thread, time::Duration};

use itertools::Itertools;

use crate::defs::Solution;

pub struct Day11Solution {
    input: String,
}

impl Solution<i64, i64> for Day11Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let mut stones = self
            .input
            .split_whitespace()
            .map(|s| Stone(s.parse().unwrap()))
            .collect_vec();

        for i in 0..25 {
            stones = Self::blink(stones);
        }

        stones.len() as i64
    }

    // 0
    // 1
    // 2024
    // 20 24
    // 2 0 2 4
    // 0 -> 4 after 4

    // 2
    // 4048
    // 40 48
    // 4 0 4 8

    // 4
    // 8096
    // 80 96
    // 8 0 9 6

    // 8
    // 16192
    // 32772608
    // 3277 2608
    // 32 77 26 8 !!!!

    fn get_part_2_solution(&self) -> i64 {
        let mut stones = self
            .input
            .split_whitespace()
            .map(|s| Stone(s.parse().unwrap()))
            .collect_vec();

        let mut cycles = vec![];
        let max_compute = 77;
        for i in 0..=max_compute {
            let mut intermediate = vec![];
            let mut c_c = vec![Stone(i)];
            let mut cycle_over = false;
            let mut steps = 0;
            while !cycle_over {
                steps += 1;
                intermediate.push(c_c.clone());
                c_c = Self::blink(c_c);
                // // debug print
                // let s = c_c.iter().map(|s| s.0).join(" ");
                // thread::sleep(Duration::from_millis(1000));
                // println!("{s}");
                cycle_over = steps == 75
                    || c_c.iter().all(|s| s.0 < max_compute) // only need up to what we're computing
                    || c_c.iter().any(|s| s.0 == i); // catch repeats of self (which will never finish, like "8")
            }

            intermediate.push(c_c.clone());
            cycles.push(Cycle {
                init: i,
                intermediates: intermediate,
                steps,
            });
        }

        println!("All cycles computed!");

        let mut idx = 0;
        loop {
            let stone = &stones[idx];
            let mut cycle = cycles.get(stone.0 as usize);
            let mut s = vec![stone.clone()];
            let mut early_steps = 0;
            while cycle.is_none() {
                s = Self::blink(s);
                break;
            }
            break;
        }

        0
    }
}

impl Day11Solution {
    pub fn blink(stones: Vec<Stone>) -> Vec<Stone> {
        let mut new = vec![];
        for stone in stones {
            let val = stone.0;
            let vs = val.to_string();
            if val == 0 {
                new.push(Stone(1));
                continue;
            }

            if vs.len() % 2 == 0 {
                let mid = vs.len() / 2;
                let (left, right) = vs.split_at(mid);
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                new.push(Stone(left));
                new.push(Stone(right));
                continue;
            }

            new.push(Stone(val * 2024));
        }

        new
    }
}

#[derive(Clone)]
pub struct Stone(i64);

#[derive(Clone)]
pub struct Cycle {
    init: i64,
    steps: i64,
    intermediates: Vec<Vec<Stone>>,
}

impl Cycle {
    pub fn traverse(
        &self,
        all_cycles: &Vec<Cycle>,
        current_steps: &mut i64,
        max_steps: i64,
    ) -> i64 {
        let to_take = (*current_steps - max_steps).min(self.steps);
        let result = &self.intermediates[to_take as usize - 1];

        if to_take == self.steps {
            // Hit end of cycle, need to traverse rest
            let mut total_len = 0;
            for s in result {
                let c = &all_cycles[s.0 as usize];
                let len_done = c.traverse(all_cycles, current_steps, max_steps);
                total_len += len_done;
            }

            total_len
        } else {
            result.len() as i64
        }
    }
}
