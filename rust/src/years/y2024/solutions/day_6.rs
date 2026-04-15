use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

use crate::{
    defs::Solution,
    utils::grid::{Grid, GridCreationItem, GridDirection},
};

pub struct Day6Solution {
    input: String,
    grid: Grid<RefCell<char>>,
}

impl Solution<i32, i32> for Day6Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        let grid = Self::make_grid(&input);
        Self { input, grid }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i32 {
        let starting_pos = self.grid.scan().find(|n| *n.value.borrow() == '^').unwrap();
        let mut visited = HashMap::new();
        let mut starting_hashset = HashSet::new();
        starting_hashset.insert(starting_pos.y);
        visited.insert(starting_pos.x, starting_hashset);
        let mut current_direction = GridDirection::North;
        let mut current_node = starting_pos;

        let mut walks = 0;
        'outer: loop {
            let mut iter = current_node.direction_iter(&self.grid, current_direction.clone());
            for node in iter {
                if *node.value.borrow() == '#' {
                    current_direction = current_direction.rotate_right();
                    walks += 1;
                    continue 'outer;
                }

                let mut entry = visited.entry(node.x).or_insert(HashSet::new());
                entry.insert(node.y);
                current_node = node;
            }

            // We exited the map, break and calc.
            break;
        }

        let unique_points = visited.values().fold(0, |acc, curr| acc + curr.len());

        unique_points as i32
    }

    fn get_part_2_solution(&self) -> i32 {
        let num_obs = self
            .grid
            .scan()
            .filter(|n| *n.value.borrow() == '#')
            .count();
        let starting_pos = self.grid.scan().find(|n| *n.value.borrow() == '^').unwrap();
        let mut valid_pos = 0;

        'outer: for replace_node in self.grid.scan() {
            if *replace_node.value.borrow() == '#' {
                continue;
            }

            let old_value = replace_node.value.replace('#');

            let mut visited = HashSet::new();
            let mut current_direction = GridDirection::North;
            let mut current_node = starting_pos;
            visited.insert((current_node.x, current_node.y, current_direction.clone()));

            let mut steps = 0;
            'inner: loop {
                let mut iter = current_node.direction_iter(&self.grid, current_direction.clone());
                for node_inner in iter {
                    let did_visit_position =
                        visited.contains(&(node_inner.x, node_inner.y, current_direction.clone()));

                    if did_visit_position {
                        // We're in a cycle
                        valid_pos += 1;
                        replace_node.value.replace(old_value);
                        continue 'outer;
                    } else {
                        visited.insert((node_inner.x, node_inner.y, current_direction.clone()));
                    }

                    if *node_inner.value.borrow() == '#' {
                        current_direction = current_direction.rotate_right();
                        continue 'inner;
                    }

                    current_node = node_inner;
                }

                replace_node.value.replace(old_value);

                // We exited the map, break.
                break;
            }
        }

        valid_pos
    }
}

impl Day6Solution {
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
