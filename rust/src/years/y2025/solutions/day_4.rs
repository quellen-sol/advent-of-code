use std::{cell::RefCell, collections::HashSet};

use crate::{
    defs::Solution,
    utils::grid::{Grid, GridCreationItem},
};

pub struct Day4Solution {
    input: String,
    grid: Grid<RefCell<char>>,
}

impl Solution<i64, usize> for Day4Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        let grid = Self::build_grid(&input);
        Self { input, grid }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let mut liftable = 0;
        for item in self.grid.scan() {
            if *item.value.borrow() != '@' {
                continue;
            }

            let mut surrounding = 0;
            for (_, neighbor) in item.neighbors(&self.grid) {
                if *neighbor.value.borrow() == '@' {
                    surrounding += 1;
                }
            }

            if surrounding < 4 {
                liftable += 1;
            }
        }

        liftable
    }

    fn get_part_2_solution(&self) -> usize {
        let mut liftable = HashSet::new();
        let mut total_removed = 0;
        loop {
            for item in self.grid.scan() {
                if *item.value.borrow() != '@' {
                    continue;
                }

                let mut surrounding = 0;
                for (_, neighbor) in item.neighbors(&self.grid) {
                    if *neighbor.value.borrow() == '@' {
                        surrounding += 1;
                    }
                }

                if surrounding < 4 {
                    liftable.insert((item.x, item.y));
                }
            }

            total_removed += liftable.len();

            for (r_x, r_y) in liftable.iter() {
                *self.grid.get_node(*r_x, *r_y).unwrap().value.borrow_mut() = '.';
            }

            if liftable.is_empty() {
                break;
            }

            liftable.clear();
        }

        total_removed
    }
}

impl Day4Solution {
    fn build_grid(input: &str) -> Grid<RefCell<char>> {
        let creation_iter = input.chars().map(|c| {
            if c == '\n' {
                GridCreationItem::Break
            } else {
                GridCreationItem::Item(RefCell::new(c))
            }
        });

        Grid::from_iter(creation_iter)
    }
}
