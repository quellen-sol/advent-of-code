use std::collections::{HashMap, HashSet};

use crate::defs::Solution;

pub struct Day9Solution {
    input: String,
}

impl Solution<i64, i64> for Day9Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let mut v = vec![];

        let mut id = 0;
        for (idx, n) in self.input.chars().enumerate() {
            let is_free_space = idx % 2 == 1;
            let n = n.to_string().parse::<i64>().unwrap();

            for i in 0..n {
                if is_free_space {
                    v.push(Block::Empty(n));
                } else {
                    v.push(Block::Value((n, id)));
                }
            }

            if is_free_space {
                id += 1;
            }
        }

        let mut apply_idx = 0;
        'outer: for (idx, val) in v.clone().iter().enumerate().rev() {
            if apply_idx > idx {
                break;
            }

            if matches!(*val, Block::Empty(_)) {
                continue;
            }

            loop {
                let v_item = v.get(apply_idx).unwrap();
                match v_item {
                    Block::Value(_) => {
                        apply_idx += 1;
                        if apply_idx > idx {
                            break 'outer;
                        }
                        continue;
                    }
                    Block::Empty(_) => {
                        v.swap(apply_idx, idx);
                        break;
                    }
                }
            }
        }

        let score = v.iter().enumerate().fold(0, |mut acc, (idx, curr)| {
            let Block::Value((_, val)) = curr else {
                return acc;
            };

            acc += *val * (idx as i64);
            acc
        });

        score
    }

    fn get_part_2_solution(&self) -> i64 {
        let mut v = vec![];

        let mut id = 0;
        for (idx, n) in self.input.chars().enumerate() {
            let is_free_space = idx % 2 == 1;
            let amt = n.to_string().parse::<i64>().unwrap();

            if is_free_space {
                if amt > 0 {
                    v.push(Block::Empty(amt));
                }
            } else {
                v.push(Block::Value((amt, id)));
            }

            if is_free_space {
                id += 1;
            }
        }

        let mut r_idx = v.len() - 1;

        'right_search: loop {
            let r_block = &v[r_idx];
            let mut affected_idx = None;
            let mut original_idx = None;
            let mut difference = 0;
            match r_block {
                Block::Empty(_) => {}
                Block::Value((r_amt, r_id)) => {
                    'left_search: for (l_idx, l_block) in v.iter().enumerate() {
                        if l_idx > r_idx {
                            break 'left_search;
                        }
                        match l_block {
                            Block::Empty(l_amt) => {
                                if l_amt >= r_amt {
                                    affected_idx = Some(l_idx);
                                    original_idx = Some(r_idx);
                                    difference = l_amt - r_amt;
                                    break 'left_search;
                                }
                            }
                            Block::Value(_) => {}
                        }
                    }

                    let block = r_block.clone();

                    if let Some(a_i) = affected_idx {
                        if difference > 0 {
                            let empty = v.get_mut(a_i).unwrap();
                            *empty = Block::Empty(difference);
                            r_idx += 1;
                        } else {
                            v.remove(a_i);
                        }

                        v.insert(a_i, block);
                    }

                    if let Some(mut o_i) = original_idx {
                        if difference > 0 {
                            o_i += 1;
                        }
                        let item = v.get_mut(o_i).unwrap();
                        let amt = item.amt();
                        *item = Block::Empty(amt);
                    }
                }
            }

            if r_idx <= 0 {
                break 'right_search;
            }

            r_idx -= 1;
        }

        let mut score = 0;
        let mut idx = 0;
        for block in v {
            match block {
                Block::Empty(amt) => {
                    idx += amt;
                }
                Block::Value((amt, id)) => {
                    for i in 0..amt {
                        score += idx * id;
                        idx += 1;
                    }
                }
            }
        }

        score
    }
}

impl Day9Solution {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
    /// Amt of empty spaces
    Empty(i64),
    /// (amt, value)
    Value((i64, i64)),
}

impl Block {
    pub fn amt(&self) -> i64 {
        match self {
            Block::Value((amt, v)) => *amt,
            Block::Empty(amt) => *amt,
        }
    }

    pub fn id(&self) -> i64 {
        match self {
            Block::Value((_, id)) => *id,
            _ => panic!("lol"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::defs::Solution;

    use super::Day9Solution;

    #[test]
    fn part_2() {
        let input = "2333133121414131402";
        let day_9 = Day9Solution {
            input: input.into(),
        };

        let answer = day_9.get_part_2_solution();
        println!("{answer}");
    }
}
