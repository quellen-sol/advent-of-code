#[macro_export]
macro_rules! make_and_run_solution {
    ($day:expr, $year:expr) => {
        ::paste::paste! {
            let solution = years::[<y $year>]::solutions::[<Day $day Solution>]::new(&format!("./src/years/y{}/inputs/{}-input.txt", $year, $day));
            solution.run_both_solutions();
        }
    };
}
