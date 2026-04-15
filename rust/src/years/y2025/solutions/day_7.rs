use std::cell::RefCell;

use crate::{
    defs::Solution,
    utils::grid::{Grid, GridCreationItem, GridDirection},
};

pub struct Day7Solution {
    input: String,
    grid: Grid<RefCell<char>>,
}

impl Solution<i64, i64> for Day7Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        let grid = Self::build_grid(&input);
        Self { input, grid }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let mut total_splits= 0;
        for item in self.grid.scan() {
            match *item.value.borrow() {
                'S' => {
                    let south = item
                        .get_node_in_direction(&self.grid, &GridDirection::South)
                        .unwrap();
                    *south.value.borrow_mut() = '|';
                }
                '|' => {
                    let south = item
                        .get_node_in_direction(&self.grid, &GridDirection::South);

                    let Some(s) = south else {
                        continue;
                    };

                    let s_val = *s.value.borrow();

                    match s_val {
                        '.' => {
                            *s.value.borrow_mut() = '|';
                        }
                        '^' => {
                            total_splits += 1;
                            let east = s.get_node_in_direction(&self.grid, &GridDirection::East);
                            let west = s.get_node_in_direction(&self.grid, &GridDirection::West);

                            if let Some(east) = east {
                                *east.value.borrow_mut() = '|';
                            }

                            if let Some(west) = west {
                                *west.value.borrow_mut() = '|';
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        total_splits
    }

    fn get_part_2_solution(&self) -> i64 {
        todo!()
    }
}

impl Day7Solution {
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
