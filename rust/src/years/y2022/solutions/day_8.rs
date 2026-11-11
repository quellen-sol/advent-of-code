use std::{cell::RefCell, collections::HashMap};

use itertools::Itertools;

use crate::{
    defs::Solution,
    utils::grid::{Grid, GridCreationItem, GridDirection, GridNode, GridNodeDirectionIterator},
};

pub struct Day8Solution {
    input: String,
}

impl Solution<i64, i64> for Day8Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let grid = self.build_grid();
        let mut visible = 0;
        let grid_max_x = (grid.num_rows() - 1) as isize;
        let grid_max_y = (grid.num_columns() - 1) as isize;
        for node in grid.scan() {
            if node.x == 0 || node.x >= grid_max_x || node.y == 0 || node.y == grid_max_y {
                visible += 1;
                continue;
            }

            let north_iter = node.direction_iter(&grid, GridDirection::North);
            let south_iter = node.direction_iter(&grid, GridDirection::South);
            let east_iter = node.direction_iter(&grid, GridDirection::East);
            let west_iter = node.direction_iter(&grid, GridDirection::West);

            let findings = VisibilityFindings {
                from_north: Self::check_node_visibility(*node.value.borrow(), north_iter),
                from_south: Self::check_node_visibility(*node.value.borrow(), south_iter),
                from_east: Self::check_node_visibility(*node.value.borrow(), east_iter),
                from_west: Self::check_node_visibility(*node.value.borrow(), west_iter),
            };

            if findings.is_visible() {
                visible += 1;
            }
        }

        visible
    }

    fn get_part_2_solution(&self) -> i64 {
        let grid = self.build_grid();
        let mut max_score = 0;
        for node in grid.scan() {
            let north_iter = node.direction_iter(&grid, GridDirection::North);
            let south_iter = node.direction_iter(&grid, GridDirection::South);
            let east_iter = node.direction_iter(&grid, GridDirection::East);
            let west_iter = node.direction_iter(&grid, GridDirection::West);

            let findings = VisibilityFindings {
                from_north: Self::check_node_visibility(*node.value.borrow(), north_iter),
                from_south: Self::check_node_visibility(*node.value.borrow(), south_iter),
                from_east: Self::check_node_visibility(*node.value.borrow(), east_iter),
                from_west: Self::check_node_visibility(*node.value.borrow(), west_iter),
            };

            let score = findings.total_score();
            if score > max_score {
                max_score = score;
            }
        }

        max_score
    }
}

impl Day8Solution {
    fn build_grid(&self) -> Grid<RefCell<i64>> {
        Grid::from_iter(self.input.chars().map(|c| match c {
            '\n' => GridCreationItem::Break,
            ch => GridCreationItem::Item(RefCell::new(ch.to_digit(10).unwrap() as i64)),
        }))
    }

    fn check_node_visibility<'a, T: Ord>(
        cmp_val: T,
        mut iter: GridNodeDirectionIterator<'a, RefCell<T>>,
    ) -> (VisibilityStatus, i64) {
        let mut score = 0;
        for node in iter {
            score += 1;
            if *node.value.borrow() >= cmp_val {
                return (VisibilityStatus::NotVisible, score)
            }
        }

        (VisibilityStatus::Visible, score)
    }
}

struct VisibilityFindings {
    from_north: (VisibilityStatus, i64),
    from_south: (VisibilityStatus, i64),
    from_east: (VisibilityStatus, i64),
    from_west: (VisibilityStatus, i64),
}

impl VisibilityFindings {
    pub fn is_visible(&self) -> bool {
        matches!(self.from_north.0, VisibilityStatus::Visible)
            || matches!(self.from_south.0, VisibilityStatus::Visible)
            || matches!(self.from_east.0, VisibilityStatus::Visible)
            || matches!(self.from_west.0, VisibilityStatus::Visible)
    }

    pub fn total_score(&self) -> i64 {
        self.from_north.1 * self.from_south.1 * self.from_east.1 * self.from_west.1
    }
}

enum VisibilityStatus {
    Visible,
    NotVisible,
    Unknown,
}
