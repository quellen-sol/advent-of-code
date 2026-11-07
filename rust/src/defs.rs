use std::{fmt::Display, fs, hint::black_box, str::Lines, time::Duration};

use crate::timing::Timer;

pub trait Solution<T1: Display, T2: Display>: Sized {
    fn new(input_path: &str) -> Self;
    fn get_part_1_solution(&self) -> T1;
    fn get_part_2_solution(&self) -> T2;
    fn get_input(&self) -> &str;
    fn get_lines(&self) -> Lines {
        self.get_input().lines()
    }

    fn load_input(input_path: &str) -> String {
        fs::read_to_string(input_path).unwrap()
    }

    /// Each run gets a new instance, because a part can mutate its solution
    /// through a `RefCell`, which would change what the next run measures.
    fn run_both_solutions(input_path: &str, budget: Duration) {
        let create = || Self::new(input_path);

        let mut timer = Timer::new(budget);
        timer.time(
            "Setup",
            || (),
            |_| {
                black_box(create());
                ""
            },
        );
        timer.time("Part 1", create, Self::get_part_1_solution);
        timer.time("Part 2", create, Self::get_part_2_solution);
        timer.print_total();
    }
}
