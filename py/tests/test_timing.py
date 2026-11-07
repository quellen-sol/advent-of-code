import unittest

from py.timing import HEADER, Stats, format_duration, format_row, measure, summarize

SHORT_BUDGET_SECONDS = 0.005

EXAMPLE_STATS = Stats(
    median=1_500_000,
    min=1_000_000,
    mean=2_000_000,
    stddev=250_000,
    cold=9_000_000,
    runs=42,
)


class Counter:
    def __init__(self) -> None:
        self.count = 0

    def increment(self) -> int:
        self.count += 1
        return self.count


class TimingTests(unittest.TestCase):
    def test_durations_scale_to_a_readable_unit(self) -> None:
        cases = [
            (0, "0 ns"),
            (999, "999 ns"),
            (1_000, "1.000 µs"),
            (1_234_567, "1.235 ms"),
            (999_999_000, "999.999 ms"),
            (1_500_000_000, "1.500 s"),
            (65_000_000_000, "65.000 s"),
        ]

        for nanos, expected in cases:
            self.assertEqual(format_duration(nanos), expected)

    def test_stats_summarize_an_odd_number_of_samples(self) -> None:
        self.assertEqual(
            summarize([3, 1, 2], 9),
            Stats(median=2, min=1, mean=2, stddev=1, cold=9, runs=3),
        )

    def test_median_of_an_even_number_of_samples_is_the_middle_pair_averaged(self) -> None:
        self.assertEqual(summarize([4, 1, 3, 2], 0).median, 2.5)

    def test_a_single_sample_has_no_deviation(self) -> None:
        self.assertEqual(summarize([5], 5).stddev, 0)

    def test_rows_match_the_shared_layout(self) -> None:
        self.assertEqual(
            format_row("Part 1", EXAMPLE_STATS, "1132", False),
            "Part 1    1.500 ms  1132",
        )
        self.assertEqual(format_row("Setup", EXAMPLE_STATS, "", False), "Setup     1.500 ms")
        self.assertEqual(
            format_row("Part 1", EXAMPLE_STATS, "1132", True),
            "Part 1    1.500 ms    1.000 ms    2.000 ms  250.000 µs    9.000 ms       42  1132",
        )

    def test_header_lines_up_with_the_columns(self) -> None:
        row = format_row("Part 1", EXAMPLE_STATS, "1132", True)
        self.assertEqual(HEADER.index("answer"), row.index("1132"))

    def test_no_budget_means_one_run(self) -> None:
        counter = Counter()

        answer, stats = measure(lambda: None, lambda _: counter.increment() and 7, 0)

        self.assertEqual(answer, "7")
        self.assertEqual(counter.count, 1)
        self.assertEqual(stats.runs, 1)
        self.assertEqual(stats.median, stats.cold)

    def test_every_run_gets_a_fresh_subject(self) -> None:
        answer, stats = measure(Counter, Counter.increment, SHORT_BUDGET_SECONDS)

        self.assertEqual(answer, "1")
        self.assertGreater(stats.runs, 1)

    def test_a_changing_answer_is_an_error(self) -> None:
        counter = Counter()

        with self.assertRaisesRegex(RuntimeError, "the answer changed between runs"):
            measure(lambda: None, lambda _: counter.increment(), SHORT_BUDGET_SECONDS)


if __name__ == "__main__":
    unittest.main()
