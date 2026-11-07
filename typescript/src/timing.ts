import { ISolution } from "./solution";

// Every language implements these same rules and output, so that their numbers
// can be compared. See "Timing" in the README before changing one of them.

/** Parts fast enough to reach this stop early instead of using the whole budget. */
const MAX_RUNS = 100_000;

/** The share of the budget spent on warm-up runs, which aren't recorded. */
const WARMUP_SHARE = 0.2;

export const HEADER =
    "            median         min        mean      stddev        cold     runs  answer";

/** All times are in nanoseconds. */
export interface Stats {
    median: number;
    min: number;
    mean: number;
    stddev: number;
    cold: number;
    runs: number;
}

function nowNanos(): number {
    return performance.now() * 1e6;
}

export function summarize(samples: number[], cold: number): Stats {
    const sorted = [...samples].sort((a, b) => a - b);

    const runs = sorted.length;
    const middle = Math.floor(runs / 2);
    const median =
        runs % 2 === 0
            ? (sorted[middle - 1]! + sorted[middle]!) / 2
            : sorted[middle]!;

    const mean = sorted.reduce((sum, sample) => sum + sample, 0) / runs;
    const squaredError = sorted.reduce(
        (sum, sample) => sum + (sample - mean) ** 2,
        0,
    );
    const stddev = Math.sqrt(squaredError / Math.max(runs - 1, 1));

    return { median, min: sorted[0]!, mean, stddev, cold, runs };
}

export function formatDuration(nanos: number): string {
    if (nanos < 1e3) {
        return `${nanos.toFixed(0)} ns`;
    }
    if (nanos < 1e6) {
        return `${(nanos / 1e3).toFixed(3)} µs`;
    }
    if (nanos < 1e9) {
        return `${(nanos / 1e6).toFixed(3)} ms`;
    }
    return `${(nanos / 1e9).toFixed(3)} s`;
}

export function formatRow(
    label: string,
    stats: Stats,
    answer: string,
    bench: boolean,
): string {
    const cell = (nanos: number) => formatDuration(nanos).padStart(10);

    let row = `${label.padEnd(6)}  ${cell(stats.median)}`;
    if (bench) {
        const times = [stats.min, stats.mean, stats.stddev, stats.cold]
            .map(cell)
            .join("  ");
        row += `  ${times}  ${String(stats.runs).padStart(7)}`;
    }
    return `${row}  ${answer}`.trimEnd();
}

/**
 * Times `run` on a fresh value from `prepare` each time, keeping `prepare`
 * outside the timer. A zero budget means a single run.
 */
export function measure<S>(
    prepare: () => S,
    run: (subject: S) => unknown,
    budgetSeconds: number,
): { answer: string; stats: Stats } {
    const timeRun = () => {
        const subject = prepare();
        const start = nowNanos();
        const result = run(subject);
        const elapsed = nowNanos() - start;
        return { elapsed, result: String(result) };
    };

    const started = nowNanos();
    const { elapsed: cold, result: answer } = timeRun();

    const budget = budgetSeconds * 1e9;
    const samples: number[] = [];
    while (nowNanos() - started < budget && samples.length < MAX_RUNS) {
        const warmingUp = nowNanos() - started < budget * WARMUP_SHARE;
        const { elapsed, result } = timeRun();
        if (result !== answer) {
            throw new Error(
                `the answer changed between runs: ${answer}, then ${result}`,
            );
        }
        if (!warmingUp) {
            samples.push(elapsed);
        }
    }

    if (samples.length === 0) {
        samples.push(cold);
    }
    return { answer, stats: summarize(samples, cold) };
}

class Timer {
    private total = 0;

    constructor(private readonly budgetSeconds: number) {
        if (this.isBench) {
            console.log(HEADER);
        }
    }

    private get isBench(): boolean {
        return this.budgetSeconds > 0;
    }

    /** Prints as soon as it finishes, so a later throw can't hide the row. */
    time<S>(label: string, prepare: () => S, run: (subject: S) => unknown) {
        const { answer, stats } = measure(prepare, run, this.budgetSeconds);
        console.log(formatRow(label, stats, answer, this.isBench));
        this.total += stats.median;
    }

    printTotal() {
        console.log(
            `${"Total".padEnd(6)}  ${formatDuration(this.total).padStart(10)}`,
        );
    }
}

/**
 * Each run gets a new instance from `create`, so a part that mutates its
 * solution can't change what the next run measures.
 */
export function runSolution(
    create: () => ISolution<unknown, unknown>,
    budgetSeconds: number,
) {
    const timer = new Timer(budgetSeconds);
    timer.time(
        "Setup",
        () => undefined,
        () => {
            create();
            return "";
        },
    );
    timer.time("Part 1", create, (solution) => solution.part1());
    timer.time("Part 2", create, (solution) => solution.part2());
    timer.printTotal();
}
