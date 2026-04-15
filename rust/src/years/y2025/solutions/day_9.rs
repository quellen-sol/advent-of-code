use crate::{defs::Solution, utils::point::Point2D};

pub struct Day9Solution {
    input: String,
    points: Vec<Point2D>,
}

impl Solution<i64, i64> for Day9Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        let points = Self::build_points(&input);
        Self { input, points }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        let mut largest_area = 0;

        for p_x in &self.points {
            for p_y in &self.points {
                if p_x == p_y {
                    continue;
                }

                let area = p_x.gridlike_corner_rect_area(p_y);
                if area > largest_area {
                    largest_area = area;
                }
            }
        }

        largest_area
    }

    fn get_part_2_solution(&self) -> i64 {
        todo!()
    }
}

impl Day9Solution {
    pub fn build_points(input: &str) -> Vec<Point2D> {
        let mut points = Vec::new();

        for line in input.lines() {
            let mut p = line.split(',');
            let x = p.next().unwrap().parse().unwrap();
            let y = p.next().unwrap().parse().unwrap();

            let point = Point2D { x, y };
            points.push(point);
        }

        points
    }
}
