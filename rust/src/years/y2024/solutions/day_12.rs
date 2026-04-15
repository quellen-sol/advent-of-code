use std::collections::{HashMap, HashSet};

use crate::{
    defs::Solution,
    utils::grid::{Grid, GridCreationItem, GridDirection, GridNode},
};

pub struct Day12Solution {
    input: String,
    grid: Grid<char>,
}

impl Solution<i64, i64> for Day12Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        let grid = Self::make_grid(&input);
        Self { input, grid }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let mut total_price = 0;
        let mut visited = HashSet::new();
        for node in self.grid.scan() {
            if !visited.insert((node.x, node.y)) {
                continue;
            }

            let mut group = vec![];
            self.fill_group(node, &mut visited, &mut group);

            let mut perim = 0;
            for n in group.iter() {
                let num_neighbors = n
                    .plus_walk(&self.grid)
                    .filter(|(d, a)| group.contains(a))
                    .count();
                let this_perim = 4 - num_neighbors;
                perim += this_perim;
            }

            total_price += (group.len() * perim) as i64;
        }

        total_price
    }

    // 764_544 too low
    // 1_467_080 too high
    // 1_407_934 too high
    fn get_part_2_solution(&self) -> i64 {
        let mut total_price = 0;
        let mut visited = HashSet::new();
        let mut group_sides = vec![];
        for node in self.grid.scan() {
            if !visited.insert((node.x, node.y)) {
                continue;
            }

            let mut group = vec![];
            self.fill_group(node, &mut visited, &mut group);

            // Every group[0] is the top-leftmost. So start moving right
            // Direction is a stack. Right -> Down -> Left -> Up
            // If all of the sudden we can move in the direction below (i.e., Up -> Left), do it
            // Until we reach node_0 again

            let take_while_predicate = |node: &&GridNode<char>| group.contains(node);

            let first = group.first().unwrap();
            let mut dir_stack = vec![];
            let mut dir = GridDirection::East;
            let mut curr_node = *first;
            let mut sides = 1;
            let mut curr_iter = curr_node.direction_iter(&self.grid, dir.clone());

            let mut possible_outer_nodes = vec![];

            loop {
                if curr_node == *first && dir == GridDirection::North {
                    // We've hit the beginning, while trying to travel up. This is the end.
                    // println!("{curr_node:?} is equal to {first:?}, and we're going north. RIP");
                    break;
                }

                // Before stepping again, can we move in the direction below in priority?
                if let Some(d_dir) = dir_stack.last() {
                    let n_n = curr_node.get_node_in_direction(&self.grid, d_dir);
                    if let Some(n_n) = n_n {
                        if group.contains(&n_n) {
                            // Set the new iter and direction
                            sides += 1;
                            dir = dir_stack.pop().unwrap();
                            curr_iter = curr_node.direction_iter(&self.grid, dir.clone());
                        } else {
                            // Record this for later. We'll check if this current group is inside another
                            possible_outer_nodes.push(n_n);
                        }
                    }
                }

                let next_node = curr_iter.next();

                if let Some(next_node) = next_node {
                    if group.contains(&next_node) {
                        // println!("Got next: {next_node:?}");
                        curr_node = next_node;
                        continue;
                    } else {
                        // Record this for later. We'll check if this current group is inside another
                        possible_outer_nodes.push(next_node);
                    }
                }

                // Can't keep going.
                sides += 1;
                dir_stack.push(dir.clone());
                dir = dir.rotate_right();
                curr_iter = curr_node.direction_iter(&self.grid, dir.clone());
            }

            group_sides.push((group, sides, possible_outer_nodes));
        }

        let mut changes = vec![];
        for (group, sides, outers) in group_sides.iter() {
            // Does this group touch the bounds at any point? If so, ignore
            let grid_max_y = (self.grid.num_rows() - 1) as isize;
            let grid_max_x = (self.grid.num_columns() - 1) as isize;
            let hits_boundary = group
                .iter()
                .any(|n| n.x >= grid_max_x || n.y >= grid_max_y || n.x <= 0 || n.y <= 0);

            if hits_boundary {
                continue;
            }

            let mut outer_group = None;
            let mut outer_matches = true;
            for outer in outers {
                let relevant_group = group_sides
                    .iter()
                    .enumerate()
                    .find(|(idx, (g, ..))| g != group && g.contains(outer));

                if let Some(ref r_g) = relevant_group {
                    if let Some(ref o_g) = outer_group {
                        if r_g != o_g {
                            // Touches more than two groups, ignore
                            outer_matches = false;
                            break;
                        }
                    } else {
                        outer_group = relevant_group;
                    }
                }
            }

            if outer_matches {
                let Some((idx, (_, sides, ..))) = outer_group else {
                    continue;
                };
                changes.push((idx, *sides));
            }
        }

        // Apply all changes
        for (c_idx, change_sides) in changes {
            let entry = group_sides.get_mut(c_idx).unwrap();

            entry.1 += change_sides;
        }

        for (g, sides, ..) in group_sides {
            total_price += (g.len() * sides) as i64;
        }

        total_price
    }
}

impl Day12Solution {
    pub fn make_grid(s: &str) -> Grid<char> {
        let creation_iter = s.chars().map(|c| {
            if c == '\n' {
                GridCreationItem::Break
            } else {
                GridCreationItem::Item(c)
            }
        });

        Grid::from_iter(creation_iter)
    }

    pub fn fill_group<'a>(
        &'a self,
        node: &'a GridNode<char>,
        visited: &mut HashSet<(isize, isize)>,
        curr_group: &mut Vec<&'a GridNode<char>>,
    ) {
        curr_group.push(node);
        visited.insert((node.x, node.y));

        for (_, neigh) in node.plus_walk(&self.grid) {
            if neigh.value != node.value {
                // irrelevant
                continue;
            }

            if !visited.insert((neigh.x, neigh.y)) {
                continue;
            }

            self.fill_group(neigh, visited, curr_group);
        }
    }

    pub fn downgrade_dir(dir: &GridDirection) -> Option<GridDirection> {
        match dir {
            GridDirection::East => None,
            _ => dir.rotate_left().into(),
        }
    }

    pub fn upgrade_dir(dir: &GridDirection) -> Option<GridDirection> {
        dir.rotate_right().into()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        defs::Solution,
        utils::grid::{Grid, GridCreationItem},
    };

    use super::Day12Solution;

    fn grid() -> Grid<char> {
        let s = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let creation_iter = s.chars().map(|c| {
            if c == '\n' {
                GridCreationItem::Break
            } else {
                GridCreationItem::Item(c)
            }
        });

        Grid::from_iter(creation_iter)
    }

    #[test]
    fn grouping() {
        let g = Day12Solution {
            grid: grid(),
            input: "".into(),
        };

        let node = g.grid.get_node(0, 0).unwrap();
        let mut v = HashSet::new();
        let mut c_g = vec![];
        g.fill_group(node, &mut v, &mut c_g);

        println!("{c_g:?}");
    }

    #[test]
    fn part_2() {
        let g = Day12Solution {
            grid: grid(),
            input: "".into(),
        };

        let sol = g.get_part_2_solution();

        println!("{sol}");
    }
}
