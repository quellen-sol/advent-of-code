use std::collections::HashSet;

use crate::defs::Solution;

pub struct Day3Solution {
    input: String,
}

impl Solution<usize, usize> for Day3Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        for c in self.get_input().chars() {
            match c {
                '^' => {
                    y += 1;
                }
                'v' => {
                    y -= 1;
                }
                '<' => {
                    x -= 1;
                }
                '>' => {
                    x += 1;
                }
                _ => {
                    panic!("Not possible");
                }
            }

            visited.insert((x, y));
        }

        visited.len()
    }

    fn get_part_2_solution(&self) -> usize {
        let mut sx = 0;
        let mut sy = 0;
        let mut rx = 0;
        let mut ry = 0;
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        for (i, c) in self.get_input().chars().enumerate() {
            let is_robot = i % 2 == 1;
            match c {
                '^' => {
                    if is_robot {
                        ry += 1;
                    } else {
                        sy += 1;
                    }
                }
                'v' => {
                    if is_robot {
                        ry -= 1;
                    } else {
                        sy -= 1;
                    }
                }
                '<' => {
                    if is_robot {
                        rx -= 1;
                    } else {
                        sx -= 1;
                    }
                }
                '>' => {
                    if is_robot {
                        rx += 1;
                    } else {
                        sx += 1;
                    }
                }
                _ => {
                    panic!("Not possible");
                }
            }

            visited.insert((sx, sy));
            visited.insert((rx, ry));
        }

        visited.len()
    }
}

impl Day3Solution {}

#[cfg(test)]
mod tests {
    use crate::defs::Solution;

    use super::Day3Solution;

    #[test]
    fn day2() {
        let input = "^>v<";
        let solution = Day3Solution {
            input: input.into(),
        };

        let answer = solution.get_part_2_solution();
        assert_eq!(answer, 3);
    }
}
