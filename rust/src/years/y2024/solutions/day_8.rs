use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

use crate::{
    defs::Solution,
    utils::{
        grid::{Grid, GridCreationItem, GridNode},
        math::gcd,
    },
};

pub struct Day8Solution {
    input: String,
    grid: Grid<RefCell<char>>,
}

impl Solution<i32, i32> for Day8Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        let grid = Self::make_grid(&input);
        Self { input, grid }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i32 {
        let freq_groups: HashMap<char, Vec<&GridNode<RefCell<char>>>> =
            self.grid.scan().fold(HashMap::new(), |mut acc, curr| {
                let char_val = *curr.value.borrow();
                if char_val == '.' {
                    return acc;
                }
                let entry = acc.entry(char_val).or_default();
                entry.push(curr);

                acc
            });

        let mut visited = HashSet::new();

        let unique_anti_points = freq_groups.iter().fold(0, |mut acc, g| {
            for perm in g.1.iter().permutations(2) {
                let p1 = perm[0];
                let p2 = perm[1];
                let (rise, run) = p1.slope(p2);
                // The other permutation will capture the other
                let (new_y, new_x) = (p2.y + rise, p2.x + run);

                if self.grid.get_node(new_x, new_y).is_some() && visited.insert((new_x, new_y)) {
                    acc += 1;
                }
            }
            acc
        });

        unique_anti_points
    }

    fn get_part_2_solution(&self) -> i32 {
        let freq_groups: HashMap<char, Vec<&GridNode<RefCell<char>>>> =
            self.grid.scan().fold(HashMap::new(), |mut acc, curr| {
                let char_val = *curr.value.borrow();
                if char_val == '.' {
                    return acc;
                }
                let entry = acc.entry(char_val).or_default();
                entry.push(curr);

                acc
            });

        let mut visited = HashSet::new();

        let unique_anti_points = freq_groups.iter().fold(0, |mut acc, g| {
            for perm in g.1.iter().permutations(2) {
                let p1 = perm[0];
                let p2 = perm[1];
                let (mut rise, mut run) = p1.slope(p2);
                let s_gcd = gcd(rise as i64, run as i64) as isize;
                rise /= s_gcd;
                run /= s_gcd;

                let (mut new_y, mut new_x) = (p1.y, p1.x);
                while self.grid.get_node(new_x, new_y).is_some() {
                    if visited.insert((new_x, new_y)) {
                        acc += 1;
                    }
                    (new_y, new_x) = (new_y + rise, new_x + run);
                }
            }
            acc
        });

        unique_anti_points
    }
}

impl Day8Solution {
    pub fn make_grid(s: &str) -> Grid<RefCell<char>> {
        let creation_iter = s.chars().map(|c| {
            if c == '\n' {
                GridCreationItem::Break
            } else {
                GridCreationItem::Item(RefCell::new(c))
            }
        });

        Grid::from_iter(creation_iter)
    }
}
