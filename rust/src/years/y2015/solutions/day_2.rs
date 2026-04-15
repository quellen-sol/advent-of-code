use regex::Regex;

use crate::defs::Solution;

struct Prism {
    l: u64,
    w: u64,
    h: u64,
}

impl Prism {
    pub fn surface_area(&self) -> u64 {
        2 * self.l * self.w + 2 * self.h * self.l + 2 * self.h * self.w
    }

    pub fn smallest_side_area(&self) -> u64 {
        (self.l * self.w).min(self.h * self.l).min(self.h * self.w)
    }

    pub fn smallest_perimeter(&self) -> u64 {
        (self.l + self.w).min(self.h + self.l).min(self.h + self.w) * 2
    }

    pub fn volume(&self) -> u64 {
        self.l * self.w * self.h
    }
}

pub struct Day2Solution {
    input: String,
}

impl Solution<u64, u64> for Day2Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> u64 {
        self.make_prisms()
            .map(|p| p.surface_area() + p.smallest_side_area())
            .sum()
    }

    fn get_part_2_solution(&self) -> u64 {
        self.make_prisms()
            .map(|p| p.smallest_perimeter() + p.volume())
            .sum()
    }
}

impl Day2Solution {
    fn make_prisms(&self) -> impl Iterator<Item = Prism> + '_ {
        let r = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
        self.get_input().lines().filter_map(move |l| {
            let caps = r.captures(l)?;
            let l = caps[1].parse().unwrap();
            let w = caps[2].parse().unwrap();
            let h = caps[3].parse().unwrap();

            return Some(Prism { l, w, h });
        })
    }
}
