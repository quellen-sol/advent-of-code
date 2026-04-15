#![allow(unused)]

use std::collections::HashSet;

pub enum GridCreationItem<T> {
    Item(T),
    Break,
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub inner_data: Vec<Vec<GridNode<T>>>,
}

impl<T> Grid<T> {
    pub fn new(inner_data: Vec<Vec<GridNode<T>>>) -> Self {
        Self { inner_data }
    }

    pub fn from_iter(iter: impl Iterator<Item = GridCreationItem<T>>) -> Self {
        let mut inner_data = Vec::new();
        let mut current_row = Vec::new();
        let mut x = 0;
        let mut y = 0;

        for item in iter {
            match item {
                GridCreationItem::Item(item) => {
                    current_row.push(GridNode { value: item, x, y });
                    x += 1;
                }
                GridCreationItem::Break => {
                    inner_data.push(current_row);
                    current_row = Vec::new();
                    x = 0;
                    y += 1;
                }
            }
        }

        if !current_row.is_empty() {
            inner_data.push(current_row);
        }

        Self { inner_data }
    }

    pub fn get_node(&self, x: isize, y: isize) -> Option<&GridNode<T>> {
        self.inner_data.get(y as usize)?.get(x as usize)
    }

    pub fn scan(&self) -> GridRowIterator<T> {
        GridRowIterator {
            grid: self,
            current_position: (0, 0),
            current_rev_position: (
                self.num_columns() as isize - 1,
                self.num_rows() as isize - 1,
            ),
        }
    }

    pub fn traverse_row(&self, y: isize) -> GridNodeDirectionIterator<T> {
        let node = self.get_node(0, y).unwrap();
        node.direction_iter(self, GridDirection::East)
    }

    pub fn traverse_column(&self, x: isize) -> GridNodeDirectionIterator<T> {
        let node = self.get_node(x, 0).unwrap();
        node.direction_iter(self, GridDirection::South)
    }

    pub fn iter_rows(&self) -> GridRowIterator<T> {
        self.scan()
    }

    pub fn iter_columns(&self) -> GridColumnIterator<T> {
        GridColumnIterator {
            grid: self,
            current_position: (0, 0),
            current_rev_position: (
                self.num_columns() as isize - 1,
                self.num_rows() as isize - 1,
            ),
        }
    }

    // `slope = (rise, run)`
    pub fn iter_by_slope(
        &self,
        start_position: (isize, isize),
        slope: (isize, isize),
    ) -> GridSlopeIterator<T> {
        GridSlopeIterator {
            current_pos: start_position,
            grid: self,
            slope,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.inner_data.len()
    }

    pub fn num_columns(&self) -> usize {
        self.inner_data.first().unwrap_or(&vec![]).len()
    }
}

pub struct GridColumnIterator<'a, T> {
    grid: &'a Grid<T>,
    current_position: (isize, isize),     // (x, y)
    current_rev_position: (isize, isize), // (x, y)
}

impl<'a, T> Iterator for GridColumnIterator<'a, T> {
    type Item = &'a GridNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let num_rows = self.grid.num_rows();
        let (ref mut x, ref mut y) = self.current_position;
        let node = self.grid.get_node(*x, *y)?;
        *y += 1;
        if *y as usize >= num_rows {
            *x += 1;
            *y = 0;
        }
        Some(node)
    }
}

impl<'a, T> DoubleEndedIterator for GridColumnIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let num_rows = self.grid.num_rows();
        let (ref mut x, ref mut y) = self.current_rev_position;
        let node = self.grid.get_node(*x, *y)?;
        *y -= 1;
        if *y < 0 {
            *y = num_rows as isize - 1;
            *x -= 1;
        }
        Some(node)
    }
}

pub struct GridRowIterator<'a, T> {
    grid: &'a Grid<T>,
    current_position: (isize, isize),     // (x, y)
    current_rev_position: (isize, isize), // (x, y)
}

impl<'a, T> Iterator for GridRowIterator<'a, T> {
    type Item = &'a GridNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (ref mut x, ref mut y) = self.current_position;
        let node = self.grid.get_node(*x, *y)?;
        *x += 1;
        if *x as usize >= self.grid.inner_data[*y as usize].len() {
            *x = 0;
            *y += 1;
        }
        Some(node)
    }
}

impl<'a, T> DoubleEndedIterator for GridRowIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let num_cols = self.grid.num_columns();
        let (ref mut x, ref mut y) = self.current_rev_position;
        let node = self.grid.get_node(*x, *y)?;
        *x -= 1;
        if *x < 0 {
            *x = num_cols as isize - 1;
            *y -= 1;
        }

        Some(node)
    }
}

#[derive(Debug, Hash)]
pub struct GridNode<T> {
    pub value: T,
    pub x: isize,
    pub y: isize,
}

impl<T: Clone> Clone for GridNode<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            x: self.x,
            y: self.y,
        }
    }
}

impl<T: PartialEq> PartialEq for GridNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Eq> Eq for GridNode<T> {}

impl<T> GridNode<T> {
    pub fn add_direction(&self, direction: &GridDirection) -> (isize, isize) {
        let (dx, dy) = direction.get_offset();
        (self.x + dx, self.y + dy)
    }

    pub fn get_node_in_direction<'a>(
        &self,
        grid: &'a Grid<T>,
        direction: &GridDirection,
    ) -> Option<&'a GridNode<T>> {
        let new_dir = self.add_direction(direction);
        grid.get_node(new_dir.0, new_dir.1)
    }

    pub fn neighbors<'a>(&'a self, grid: &'a Grid<T>) -> GridNodeNeighborsIterator<'a, T> {
        GridNodeNeighborsIterator {
            grid,
            node: self,
            idx: 0,
        }
    }

    pub fn plus_walk<'a>(
        &'a self,
        grid: &'a Grid<T>,
    ) -> impl Iterator<Item = (GridDirection, &'a GridNode<T>)> {
        GridPlusIterator {
            grid,
            node: self,
            idx: 0,
        }
    }

    pub fn diagonal_walk<'a>(
        &'a self,
        grid: &'a Grid<T>,
    ) -> impl Iterator<Item = (GridDirection, &'a GridNode<T>)> {
        self.neighbors(grid).enumerate().filter_map(
            |(idx, n)| {
                if idx % 2 == 1 {
                    Some(n)
                } else {
                    None
                }
            },
        )
    }

    /// First node of this iteration is NOT the originating node this iterator was created from,
    /// but the next one in the direction specified
    pub fn direction_iter<'a>(
        &'a self,
        grid: &'a Grid<T>,
        direction: GridDirection,
    ) -> GridNodeDirectionIterator<'a, T> {
        GridNodeDirectionIterator {
            direction,
            grid,
            current_node: self,
        }
    }

    pub fn try_get_direction(&self, other: &Self) -> Option<GridDirection> {
        let diff_x = other.x.checked_sub(self.x)?;
        if diff_x.abs() > 1 {
            return None;
        }

        let diff_y = other.y.checked_sub(self.y)?;
        if diff_y.abs() > 1 {
            return None;
        }

        GridDirection::from_offset((diff_x, diff_y))
    }

    /// Returns the Manhattan Distance
    pub fn distance(&self, other: &Self) -> isize {
        (other.y - self.y).abs() + (other.x - self.x).abs()
    }

    /// returns (rise, run)
    pub fn slope(&self, other: &Self) -> (isize, isize) {
        (other.y - self.y, other.x - self.x)
    }

    /// `slope = (rise, run)`
    pub fn iter_node_by_slope<'a>(
        &self,
        grid: &'a Grid<T>,
        slope: (isize, isize),
    ) -> GridSlopeIterator<'a, T> {
        GridSlopeIterator {
            current_pos: (self.x, self.y),
            slope,
            grid,
        }
    }
}

pub struct GridNodeNeighborsIterator<'a, T> {
    grid: &'a Grid<T>,
    node: &'a GridNode<T>,
    idx: usize,
}

impl<'a, T> Iterator for GridNodeNeighborsIterator<'a, T> {
    type Item = (GridDirection, &'a GridNode<T>);

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < 8 {
            let next_direction = self.direction_from_idx()?;
            let node = self.node.get_node_in_direction(self.grid, &next_direction);
            if let Some(node) = node {
                self.idx += 1;
                return Some((next_direction, node));
            }
            self.idx += 1;
        }
        None
    }
}

impl<'a, T> GridNodeNeighborsIterator<'a, T> {
    pub fn direction_from_idx(&self) -> Option<GridDirection> {
        match self.idx {
            0 => Some(GridDirection::North),
            1 => Some(GridDirection::NorthEast),
            2 => Some(GridDirection::East),
            3 => Some(GridDirection::SouthEast),
            4 => Some(GridDirection::South),
            5 => Some(GridDirection::SouthWest),
            6 => Some(GridDirection::West),
            7 => Some(GridDirection::NorthWest),
            _ => None,
        }
    }
}

pub struct GridPlusIterator<'a, T> {
    grid: &'a Grid<T>,
    node: &'a GridNode<T>,
    idx: usize,
}

impl<'a, T> GridPlusIterator<'a, T> {
    pub fn direction_from_idx(&self) -> Option<GridDirection> {
        match self.idx {
            0 => Some(GridDirection::North),
            1 => Some(GridDirection::East),
            2 => Some(GridDirection::South),
            3 => Some(GridDirection::West),
            _ => None,
        }
    }
}

impl<'a, T> Iterator for GridPlusIterator<'a, T> {
    type Item = (GridDirection, &'a GridNode<T>);

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < 4 {
            let next_direction = self.direction_from_idx()?;
            let node = self.node.get_node_in_direction(self.grid, &next_direction);
            if let Some(node) = node {
                self.idx += 1;
                return Some((next_direction, node));
            }
            self.idx += 1;
        }
        None
    }
}

/// First node of this iteration is NOT the originating node this iterator was created from,
/// but the next one in the direction specified
pub struct GridNodeDirectionIterator<'a, T> {
    grid: &'a Grid<T>,
    current_node: &'a GridNode<T>,
    direction: GridDirection,
}

impl<'a, T> Iterator for GridNodeDirectionIterator<'a, T> {
    type Item = &'a GridNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self
            .current_node
            .get_node_in_direction(self.grid, &self.direction)?;
        self.current_node = node;
        Some(node)
    }
}

pub struct GridSlopeIterator<'a, T> {
    grid: &'a Grid<T>,
    slope: (isize, isize),
    current_pos: (isize, isize),
}

impl<'a, T> Iterator for GridSlopeIterator<'a, T> {
    type Item = &'a GridNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (ref mut c_x, ref mut c_y) = self.current_pos;
        let node = self.grid.get_node(*c_x, *c_y)?;
        let (rise, run) = &self.slope;
        *c_y += *rise;
        *c_x += *run;

        Some(node)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GridDirection {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl GridDirection {
    pub fn get_offset(&self) -> (isize, isize) {
        match self {
            GridDirection::North => (0, -1),
            GridDirection::South => (0, 1),
            GridDirection::East => (1, 0),
            GridDirection::West => (-1, 0),
            GridDirection::NorthEast => (1, -1),
            GridDirection::NorthWest => (-1, -1),
            GridDirection::SouthEast => (1, 1),
            GridDirection::SouthWest => (-1, 1),
        }
    }

    pub fn from_offset(offset: (isize, isize)) -> Option<GridDirection> {
        match offset {
            (0, -1) => Some(GridDirection::North),
            (0, 1) => Some(GridDirection::South),
            (1, 0) => Some(GridDirection::East),
            (-1, 0) => Some(GridDirection::West),
            (1, -1) => Some(GridDirection::NorthEast),
            (-1, -1) => Some(GridDirection::NorthWest),
            (1, 1) => Some(GridDirection::SouthEast),
            (-1, 1) => Some(GridDirection::SouthWest),
            _ => None,
        }
    }

    pub fn opposite(&self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::South,
            GridDirection::South => GridDirection::North,
            GridDirection::East => GridDirection::West,
            GridDirection::West => GridDirection::East,
            GridDirection::NorthEast => GridDirection::SouthWest,
            GridDirection::NorthWest => GridDirection::SouthEast,
            GridDirection::SouthEast => GridDirection::NorthWest,
            GridDirection::SouthWest => GridDirection::NorthEast,
        }
    }

    pub fn rotate_right(&self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::East,
            GridDirection::East => GridDirection::South,
            GridDirection::South => GridDirection::West,
            GridDirection::West => GridDirection::North,
            GridDirection::NorthEast => GridDirection::SouthEast,
            GridDirection::NorthWest => GridDirection::NorthEast,
            GridDirection::SouthEast => GridDirection::SouthWest,
            GridDirection::SouthWest => GridDirection::NorthWest,
        }
    }

    pub fn rotate_left(&self) -> GridDirection {
        self.rotate_right().rotate_right().rotate_right()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_grid() -> Grid<char> {
        let string = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let creation_iter = string.chars().map(|c| {
            if c == '\n' {
                GridCreationItem::Break
            } else {
                GridCreationItem::Item(c)
            }
        });
        Grid::from_iter(creation_iter)
    }

    #[test]
    fn test_grid_from_iter() {
        let grid = make_grid();
        let node = grid.get_node(0, 9).unwrap();
        assert_eq!(node.value, 'M');
    }

    #[test]
    fn test_grid_get_node() {
        let grid = make_grid();
        let node = grid.get_node(0, 0).unwrap();
        assert_eq!(node.value, 'M');

        let node = grid.get_node(1, 1).unwrap();
        assert_eq!(node.value, 'S');
    }

    #[test]
    fn test_grid_neighbors() {
        let grid = make_grid();
        let mut neighbors = grid.get_node(0, 0).unwrap().neighbors(&grid);
        let next_neighbor = neighbors.next().unwrap();
        assert_eq!(next_neighbor.1.value, 'M');

        let next_neighbor = neighbors.next().unwrap();
        assert_eq!(next_neighbor.1.value, 'S');

        let next_neighbor = neighbors.next().unwrap();
        assert_eq!(next_neighbor.1.value, 'M');

        let next_neighbor = neighbors.next();
        assert!(next_neighbor.is_none());
    }

    #[test]
    fn test_directional_iter() {
        let grid = make_grid();
        let node = grid.get_node(0, 0).unwrap();
        let mut iter = node.direction_iter(&grid, GridDirection::South);

        let next_node = iter.next().unwrap();
        assert_eq!(next_node.value, 'M');

        let next_node = iter.next().unwrap();
        assert_eq!(next_node.value, 'A');

        let next_node = iter.next().unwrap();
        assert_eq!(next_node.value, 'M');

        let next_node = iter.next().unwrap();
        assert_eq!(next_node.value, 'X');
    }

    #[test]
    fn test_grid_iter_columns() {
        let grid = make_grid();
        let mut iter = grid.iter_columns();
        let first_node = iter.next().unwrap();
        assert_eq!(first_node.value, 'M');

        let second_node = iter.next().unwrap();
        assert_eq!(second_node.value, 'M');

        let third_node = iter.next().unwrap();
        assert_eq!(third_node.value, 'A');

        let fourth_node = iter.next().unwrap();
        assert_eq!(fourth_node.value, 'M');
    }

    #[test]
    fn test_grid_iter_columns_rev() {
        let grid = make_grid();
        let mut iter = grid.iter_columns().rev();

        let first_node = iter.next().unwrap();
        assert_eq!(first_node.value, 'X');

        let second_node = iter.next().unwrap();
        assert_eq!(second_node.value, 'M');

        let third_node = iter.next().unwrap();
        assert_eq!(third_node.value, 'A');

        let fourth_node = iter.next().unwrap();
        assert_eq!(fourth_node.value, 'S');
    }

    #[test]
    fn test_grid_iter_rows_rev() {
        let grid = make_grid();
        let mut iter = grid.iter_rows().rev();

        let first_node = iter.next().unwrap();
        assert_eq!(first_node.value, 'X');

        let second_node = iter.next().unwrap();
        assert_eq!(second_node.value, 'S');

        let third_node = iter.next().unwrap();
        assert_eq!(third_node.value, 'A');

        let fourth_node = iter.next().unwrap();
        assert_eq!(fourth_node.value, 'M');
    }

    #[test]
    fn test_grid_plus_walk() {
        let grid = make_grid();
        let node = grid.get_node(1, 1).unwrap();
        let mut iter = node.plus_walk(&grid);
        let next_node = iter.next().unwrap();
        assert_eq!(next_node.1.value, 'M');

        let next_node = iter.next().unwrap();
        assert_eq!(next_node.1.value, 'A');

        let next_node = iter.next().unwrap();
        assert_eq!(next_node.1.value, 'M');

        let next_node = iter.next().unwrap();
        assert_eq!(next_node.1.value, 'M');

        let next_node = iter.next();
        assert!(next_node.is_none());
    }

    #[test]
    fn test_grid_plus_walk_2() {
        let grid = make_grid();
        let node = grid.get_node(0, 0).unwrap();
        let mut iter = node.plus_walk(&grid);
        let next_node = iter.next().unwrap();
        assert_eq!(next_node.1.value, 'M');

        let next_node = iter.next().unwrap();
        assert_eq!(next_node.1.value, 'M');

        let next_node = iter.next();
        assert!(next_node.is_none());
    }
}
