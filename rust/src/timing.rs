//! Every language implements these same rules and output, so that their numbers
//! can be compared. See "Timing" in the README before changing one of them.

use std::{
    fmt::Display,
    hint::black_box,
    time::{Duration, Instant},
};

/// Parts fast enough to reach this stop early instead of using the whole budget.
const MAX_RUNS: usize = 100_000;

/// The share of the budget spent on warm-up runs, which aren't recorded.
const WARMUP_SHARE: f64 = 0.2;

const HEADER: &str =
    "            median         min        mean      stddev        cold     runs  answer";

/// All times are in nanoseconds.
#[derive(Debug, PartialEq)]
struct Stats {
    median: f64,
    min: f64,
    mean: f64,
    stddev: f64,
    cold: f64,
    runs: usize,
}

impl Stats {
    fn new(mut samples: Vec<f64>, cold: f64) -> Self {
        samples.sort_by(f64::total_cmp);

        let runs = samples.len();
        let middle = runs / 2;
        let median = if runs.is_multiple_of(2) {
            (samples[middle - 1] + samples[middle]) / 2.0
        } else {
            samples[middle]
        };

        let mean = samples.iter().sum::<f64>() / runs as f64;
        let squared_error: f64 = samples.iter().map(|sample| (sample - mean).powi(2)).sum();
        let stddev = (squared_error / (runs - 1).max(1) as f64).sqrt();

        Self {
            median,
            min: samples[0],
            mean,
            stddev,
            cold,
            runs,
        }
    }
}

fn format_duration(nanos: f64) -> String {
    if nanos < 1e3 {
        format!("{nanos:.0} ns")
    } else if nanos < 1e6 {
        format!("{:.3} µs", nanos / 1e3)
    } else if nanos < 1e9 {
        format!("{:.3} ms", nanos / 1e6)
    } else {
        format!("{:.3} s", nanos / 1e9)
    }
}

fn format_row(label: &str, stats: &Stats, answer: &str, bench: bool) -> String {
    let mut row = format!("{label:<6}  {:>10}", format_duration(stats.median));
    if bench {
        row += &format!(
            "  {:>10}  {:>10}  {:>10}  {:>10}  {:>7}",
            format_duration(stats.min),
            format_duration(stats.mean),
            format_duration(stats.stddev),
            format_duration(stats.cold),
            stats.runs
        );
    }
    format!("{row}  {answer}").trim_end().to_string()
}

/// Times `run` on a fresh value from `prepare` each time, keeping `prepare`
/// outside the timer. A zero budget means a single run.
fn measure<S, T: Display>(
    mut prepare: impl FnMut() -> S,
    run: impl Fn(&S) -> T,
    budget: Duration,
) -> (String, Stats) {
    let mut time_run = || {
        let subject = prepare();
        let start = Instant::now();
        // Without these, the optimizer could hoist a pure part out of the loop
        // or move it past the end of the timer.
        let result = black_box(run(black_box(&subject)));
        let elapsed = start.elapsed();
        (elapsed.as_nanos() as f64, result.to_string())
    };

    let started = Instant::now();
    let (cold, answer) = time_run();

    let warmup = budget.mul_f64(WARMUP_SHARE);
    let mut samples = vec![];
    while started.elapsed() < budget && samples.len() < MAX_RUNS {
        let warming_up = started.elapsed() < warmup;
        let (elapsed, result) = time_run();
        if result != answer {
            panic!("the answer changed between runs: {answer}, then {result}");
        }
        if !warming_up {
            samples.push(elapsed);
        }
    }

    if samples.is_empty() {
        samples.push(cold);
    }
    (answer, Stats::new(samples, cold))
}

pub struct Timer {
    budget: Duration,
    total: f64,
}

impl Timer {
    pub fn new(budget: Duration) -> Self {
        let timer = Self { budget, total: 0.0 };
        if timer.is_bench() {
            println!("{HEADER}");
        }
        timer
    }

    fn is_bench(&self) -> bool {
        !self.budget.is_zero()
    }

    /// Prints as soon as it finishes, so a later `todo!()` can't hide the row.
    pub fn time<S, T: Display>(
        &mut self,
        label: &str,
        prepare: impl FnMut() -> S,
        run: impl Fn(&S) -> T,
    ) {
        let (answer, stats) = measure(prepare, run, self.budget);
        println!("{}", format_row(label, &stats, &answer, self.is_bench()));
        self.total += stats.median;
    }

    pub fn print_total(&self) {
        println!("{:<6}  {:>10}", "Total", format_duration(self.total));
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    const SHORT_BUDGET: Duration = Duration::from_millis(5);

    #[test]
    fn durations_scale_to_a_readable_unit() {
        let cases = [
            (0.0, "0 ns"),
            (999.0, "999 ns"),
            (1_000.0, "1.000 µs"),
            (1_234_567.0, "1.235 ms"),
            (999_999_000.0, "999.999 ms"),
            (1_500_000_000.0, "1.500 s"),
            (65_000_000_000.0, "65.000 s"),
        ];

        for (nanos, expected) in cases {
            assert_eq!(format_duration(nanos), expected);
        }
    }

    #[test]
    fn stats_summarize_an_odd_number_of_samples() {
        assert_eq!(
            Stats::new(vec![3.0, 1.0, 2.0], 9.0),
            Stats {
                median: 2.0,
                min: 1.0,
                mean: 2.0,
                stddev: 1.0,
                cold: 9.0,
                runs: 3,
            }
        );
    }

    #[test]
    fn median_of_an_even_number_of_samples_is_the_middle_pair_averaged() {
        assert_eq!(Stats::new(vec![4.0, 1.0, 3.0, 2.0], 0.0).median, 2.5);
    }

    #[test]
    fn a_single_sample_has_no_deviation() {
        assert_eq!(Stats::new(vec![5.0], 5.0).stddev, 0.0);
    }

    fn example_stats() -> Stats {
        Stats {
            median: 1_500_000.0,
            min: 1_000_000.0,
            mean: 2_000_000.0,
            stddev: 250_000.0,
            cold: 9_000_000.0,
            runs: 42,
        }
    }

    #[test]
    fn rows_match_the_shared_layout() {
        let stats = example_stats();

        assert_eq!(
            format_row("Part 1", &stats, "1132", false),
            "Part 1    1.500 ms  1132"
        );
        assert_eq!(format_row("Setup", &stats, "", false), "Setup     1.500 ms");
        assert_eq!(
            format_row("Part 1", &stats, "1132", true),
            "Part 1    1.500 ms    1.000 ms    2.000 ms  250.000 µs    9.000 ms       42  1132"
        );
    }

    #[test]
    fn header_lines_up_with_the_columns() {
        let row = format_row("Part 1", &example_stats(), "1132", true);
        // Compared in chars, because µ takes two bytes.
        let answer_column = row.chars().count() - "1132".len();
        assert_eq!(HEADER.find("answer"), Some(answer_column));
    }

    #[test]
    fn no_budget_means_one_run() {
        let calls = Cell::new(0);
        let run = |_: &()| {
            calls.set(calls.get() + 1);
            7
        };

        let (answer, stats) = measure(|| (), run, Duration::ZERO);

        assert_eq!(answer, "7");
        assert_eq!(calls.get(), 1);
        assert_eq!(stats.runs, 1);
        assert_eq!(stats.median, stats.cold);
    }

    #[test]
    fn every_run_gets_a_fresh_subject() {
        let run = |subject: &Cell<i32>| {
            subject.set(subject.get() + 1);
            subject.get()
        };

        let (answer, stats) = measure(|| Cell::new(0), run, SHORT_BUDGET);

        assert_eq!(answer, "1");
        assert!(stats.runs > 1);
    }

    #[test]
    #[should_panic(expected = "the answer changed between runs")]
    fn a_changing_answer_is_an_error() {
        let calls = Cell::new(0);
        let run = |_: &()| {
            calls.set(calls.get() + 1);
            calls.get()
        };

        measure(|| (), run, SHORT_BUDGET);
    }
}
