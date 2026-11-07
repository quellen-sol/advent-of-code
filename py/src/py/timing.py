"""Every language implements these same rules and output, so that their numbers
can be compared. See "Timing" in the README before changing one of them."""

from collections.abc import Callable
from dataclasses import dataclass
from statistics import fmean, median, stdev
from time import perf_counter_ns

from py.solution import Solution

# Parts fast enough to reach this stop early instead of using the whole budget.
MAX_RUNS = 100_000

# The share of the budget spent on warm-up runs, which aren't recorded.
WARMUP_SHARE = 0.2

HEADER = "            median         min        mean      stddev        cold     runs  answer"


@dataclass
class Stats:
    """All times are in nanoseconds."""

    median: float
    min: float
    mean: float
    stddev: float
    cold: float
    runs: int


def summarize(samples: list[float], cold: float) -> Stats:
    return Stats(
        median=median(samples),
        min=min(samples),
        mean=fmean(samples),
        stddev=stdev(samples) if len(samples) > 1 else 0.0,
        cold=cold,
        runs=len(samples),
    )


def format_duration(nanos: float) -> str:
    if nanos < 1e3:
        return f"{nanos:.0f} ns"
    if nanos < 1e6:
        return f"{nanos / 1e3:.3f} µs"
    if nanos < 1e9:
        return f"{nanos / 1e6:.3f} ms"
    return f"{nanos / 1e9:.3f} s"


def format_row(label: str, stats: Stats, answer: str, bench: bool) -> str:
    row = f"{label:<6}  {format_duration(stats.median):>10}"
    if bench:
        times = (stats.min, stats.mean, stats.stddev, stats.cold)
        row += "".join(f"  {format_duration(time):>10}" for time in times)
        row += f"  {stats.runs:>7}"
    return f"{row}  {answer}".rstrip()


def measure[S](
    prepare: Callable[[], S], run: Callable[[S], object], budget_seconds: float
) -> tuple[str, Stats]:
    """Times `run` on a fresh value from `prepare` each time, keeping `prepare`
    outside the timer. A zero budget means a single run."""

    def time_run() -> tuple[int, str]:
        subject = prepare()
        start = perf_counter_ns()
        result = run(subject)
        elapsed = perf_counter_ns() - start
        return elapsed, str(result)

    started = perf_counter_ns()
    cold, answer = time_run()

    budget = budget_seconds * 1e9
    samples: list[float] = []
    while perf_counter_ns() - started < budget and len(samples) < MAX_RUNS:
        warming_up = perf_counter_ns() - started < budget * WARMUP_SHARE
        elapsed, result = time_run()
        if result != answer:
            raise RuntimeError(f"the answer changed between runs: {answer}, then {result}")
        if not warming_up:
            samples.append(elapsed)

    return answer, summarize(samples or [cold], cold)


class Timer:
    def __init__(self, budget_seconds: float) -> None:
        self.budget_seconds = budget_seconds
        self.bench = budget_seconds > 0
        self.total = 0.0
        if self.bench:
            print(HEADER)

    def time[S](self, label: str, prepare: Callable[[], S], run: Callable[[S], object]) -> None:
        """Prints as soon as it finishes, so a later raise can't hide the row."""
        answer, stats = measure(prepare, run, self.budget_seconds)
        print(format_row(label, stats, answer, self.bench))
        self.total += stats.median

    def print_total(self) -> None:
        print(f"{'Total':<6}  {format_duration(self.total):>10}")


def run_solution(create: Callable[[], Solution], budget_seconds: float) -> None:
    """Each run gets a new instance from `create`, so a part that mutates its
    solution can't change what the next run measures."""

    def setup(_: None) -> str:
        create()
        return ""

    timer = Timer(budget_seconds)
    timer.time("Setup", lambda: None, setup)
    timer.time("Part 1", create, lambda solution: solution.part1())
    timer.time("Part 2", create, lambda solution: solution.part2())
    timer.print_total()
