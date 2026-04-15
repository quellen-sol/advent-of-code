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
        let mut dial = 50;
        let mut zeros = 0;

        for line in self.get_lines() {
            match Rotation::from_str(line) {
                Rotation::Left(amt) => {
                    dial = (dial - amt) % 100;
                }
                Rotation::Right(amt) => {
                    dial = (dial + amt) % 100;
                }
            }

            if dial == 0 {
                zeros += 1;
            }
        }

        zeros
    }

    fn get_part_2_solution(&self) -> i64 {
        let mut dial = 50;
        let mut zeros = 0;

        for line in self.get_lines() {
            match Rotation::from_str(line) {
                Rotation::Left(amt) => {
                    for _ in 0..amt {
                        dial -= 1;
                        if dial == 0 {
                            zeros += 1;
                        } else if dial == -1 {
                            dial = 99;
                        }
                    }
                }
                Rotation::Right(amt) => {
                    for _ in 0..amt {
                        dial += 1;
                        if dial == 100 {
                            zeros += 1;
                            dial = 0;
                        }
                    }
                }
            }
        }

        zeros
    }
}

impl Day1Solution {}

enum Rotation {
    Left(i64),
    Right(i64),
}

impl Rotation {
    #[inline]
    pub fn from_str(s: &str) -> Self {
        let (dir, amt) = {
            let mut c_iter = s.chars();
            let dir = c_iter.next().unwrap();
            let rest = c_iter.collect::<String>();

            (dir, rest.parse::<i64>().unwrap())
        };

        match dir {
            'L' => Self::Left(amt),
            'R' => Self::Right(amt),
            _ => {
                panic!("Not possible");
            }
        }
    }
}
