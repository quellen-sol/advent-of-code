use crate::{
    defs::Solution,
    utils::grid::{Grid, GridCreationItem, GridDirection},
};

pub struct Day4Solution {
    input: String,
    grid: Grid<char>,
}

impl Solution<i32, i32> for Day4Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        let grid = Self::build_grid(&input);
        Self { input, grid }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i32 {
        let scan = self.grid.scan();
        let mut num_xmases = 0;

        for node in scan {
            if node.value != 'X' {
                continue;
            }

            for (current_direction, neigh) in node.neighbors(&self.grid) {
                if neigh.value == 'M' {
                    // Start of an "XMAS"?
                    // Start iterating in the current direction to find an A and S
                    let mut dir_iter = neigh.direction_iter(&self.grid, current_direction);

                    if dir_iter.next().is_some_and(|v| v.value == 'A')
                        && dir_iter.next().is_some_and(|v| v.value == 'S')
                    {
                        num_xmases += 1;
                    }
                }
            }
        }

        num_xmases
    }

    fn get_part_2_solution(&self) -> i32 {
        let mut valid_xs = 0;
        let scan = self.grid.scan();

        for node in scan {
            if node.value == 'A' {
                let nw = node.get_node_in_direction(&self.grid, &GridDirection::NorthWest);
                let ne = node.get_node_in_direction(&self.grid, &GridDirection::NorthEast);
                let sw = node.get_node_in_direction(&self.grid, &GridDirection::SouthWest);
                let se = node.get_node_in_direction(&self.grid, &GridDirection::SouthEast);

                if nw.is_none() || ne.is_none() || sw.is_none() || se.is_none() {
                    // an X is not possible
                    continue;
                }

                let nw = nw.unwrap();
                let ne = ne.unwrap();
                let sw = sw.unwrap();
                let se = se.unwrap();

                let first_part_of_x_valid = (nw.value == 'M' || nw.value == 'S')
                    && (se.value == 'M' || se.value == 'S')
                    && (nw.value != se.value);

                let second_part_of_x_valid = (ne.value == 'M' || ne.value == 'S')
                    && (sw.value == 'M' || sw.value == 'S')
                    && (ne.value != sw.value);

                if first_part_of_x_valid && second_part_of_x_valid {
                    valid_xs += 1;
                }
            }
        }

        valid_xs
    }
}

impl Day4Solution {
    fn build_grid(input: &str) -> Grid<char> {
        let creation_iter = input.chars().map(|c| {
            if c == '\n' {
                GridCreationItem::Break
            } else {
                GridCreationItem::Item(c)
            }
        });

        Grid::from_iter(creation_iter)
    }
}
