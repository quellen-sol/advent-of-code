use std::{fmt::Display, fs, str::Lines, time::Instant};

pub trait Solution<T1: Display, T2: Display> {
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

    fn run_both_solutions(&self) {
        let now = Instant::now();
        let solution_1 = self.get_part_1_solution();
        let elapsed_1 = now.elapsed();

        println!(
            "Solution 1: {} took {:.3}ms",
            solution_1,
            elapsed_1.as_secs_f64() * 1000.0
        );

        let now = Instant::now();
        let solution_2 = self.get_part_2_solution();
        let elapsed_2 = now.elapsed();

        println!(
            "Solution 2: {} took {:.3}ms",
            solution_2,
            elapsed_2.as_secs_f64() * 1000.0
        );
    }
}
