use std::{cell::RefCell, collections::HashSet};

use crate::{
    defs::Solution,
    utils::grid::{Grid, GridCreationItem, GridNode},
};

pub struct Day10Solution {
    input: String,
    grid: Grid<i64>,
}

impl Solution<i64, i64> for Day10Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        let grid = Self::make_grid(&input);
        Self { input, grid }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    // 930 too high
    fn get_part_1_solution(&self) -> i64 {
        let mut trails = vec![];
        for node in self.grid.scan() {
            if node.value != 0 {
                continue;
            }

            let mut current_trail = vec![node];
            let mut nines_visited = HashSet::new();
            Self::trail_walk(
                node,
                &self.grid,
                &mut trails,
                &mut current_trail,
                &mut nines_visited,
            );
        }

        trails.len() as i64
    }

    fn get_part_2_solution(&self) -> i64 {
        let mut trails = vec![];
        for node in self.grid.scan() {
            if node.value != 0 {
                continue;
            }

            let mut current_trail = vec![node];
            Self::trail_walk_p2(node, &self.grid, &mut trails, &mut current_trail);
        }

        trails.len() as i64
    }
}

impl Day10Solution {
    pub fn make_grid(s: &str) -> Grid<i64> {
        let creation_iter = s.chars().map(|c| {
            if c == '\n' {
                GridCreationItem::Break
            } else {
                GridCreationItem::Item(c.to_string().parse().unwrap())
            }
        });

        Grid::from_iter(creation_iter)
    }

    pub fn trail_walk<'a>(
        node: &'a GridNode<i64>,
        grid: &'a Grid<i64>,
        result: &mut Vec<Vec<&'a GridNode<i64>>>,
        current_trail: &mut Vec<&'a GridNode<i64>>,
        nine_visited: &mut HashSet<(isize, isize)>,
    ) {
        let cv = node.value;

        if cv >= 9 {
            if nine_visited.insert((node.x, node.y)) {
                result.push(current_trail.clone());
            }
            return;
        }
        for (dir, around) in node.plus_walk(grid) {
            if around.value == cv + 1 {
                let mut ct = current_trail.clone();
                ct.push(around);
                Self::trail_walk(around, grid, result, &mut ct, nine_visited);
            }
        }
    }

    pub fn trail_walk_p2<'a>(
        node: &'a GridNode<i64>,
        grid: &'a Grid<i64>,
        result: &mut Vec<Vec<&'a GridNode<i64>>>,
        current_trail: &mut Vec<&'a GridNode<i64>>,
    ) {
        let cv = node.value;

        if cv >= 9 {
            result.push(current_trail.clone());

            return;
        }
        for (dir, around) in node.plus_walk(grid) {
            if around.value == cv + 1 {
                let mut ct = current_trail.clone();
                ct.push(around);
                Self::trail_walk_p2(around, grid, result, &mut ct);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::defs::Solution;

    use super::Day10Solution;

    #[test]
    fn p1() {
        // 89010123
        // 78121874
        // 87430965
        // 96549874
        // 45678903
        // 32019012
        // 01329801
        // 10456732
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let g = Day10Solution::make_grid(input);
        let s = Day10Solution {
            input: input.into(),
            grid: g,
        };

        let sol1 = s.get_part_1_solution();
        println!("{sol1}");
    }

    #[test]
    fn p1_2() {
        let input = "0123
1234
8765
9876";

        let g = Day10Solution::make_grid(input);
        let s = Day10Solution {
            input: input.into(),
            grid: g,
        };

        let sol1 = s.get_part_1_solution();
        println!("{sol1}");
    }
}
