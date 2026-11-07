import assert from "node:assert/strict";
import { test } from "node:test";

import {
    formatDuration,
    formatRow,
    HEADER,
    measure,
    Stats,
    summarize,
} from "./timing";

const SHORT_BUDGET_SECONDS = 0.005;

const exampleStats: Stats = {
    median: 1_500_000,
    min: 1_000_000,
    mean: 2_000_000,
    stddev: 250_000,
    cold: 9_000_000,
    runs: 42,
};

test("durations scale to a readable unit", () => {
    const cases: [number, string][] = [
        [0, "0 ns"],
        [999, "999 ns"],
        [1_000, "1.000 µs"],
        [1_234_567, "1.235 ms"],
        [999_999_000, "999.999 ms"],
        [1_500_000_000, "1.500 s"],
        [65_000_000_000, "65.000 s"],
    ];

    for (const [nanos, expected] of cases) {
        assert.equal(formatDuration(nanos), expected);
    }
});

test("stats summarize an odd number of samples", () => {
    assert.deepEqual(summarize([3, 1, 2], 9), {
        median: 2,
        min: 1,
        mean: 2,
        stddev: 1,
        cold: 9,
        runs: 3,
    });
});

test("median of an even number of samples is the middle pair averaged", () => {
    assert.equal(summarize([4, 1, 3, 2], 0).median, 2.5);
});

test("a single sample has no deviation", () => {
    assert.equal(summarize([5], 5).stddev, 0);
});

test("rows match the shared layout", () => {
    assert.equal(
        formatRow("Part 1", exampleStats, "1132", false),
        "Part 1    1.500 ms  1132",
    );
    assert.equal(
        formatRow("Setup", exampleStats, "", false),
        "Setup     1.500 ms",
    );
    assert.equal(
        formatRow("Part 1", exampleStats, "1132", true),
        "Part 1    1.500 ms    1.000 ms    2.000 ms  250.000 µs    9.000 ms       42  1132",
    );
});

test("header lines up with the columns", () => {
    const row = formatRow("Part 1", exampleStats, "1132", true);
    assert.equal(HEADER.indexOf("answer"), row.indexOf("1132"));
});

test("no budget means one run", () => {
    let calls = 0;
    const { answer, stats } = measure(
        () => undefined,
        () => {
            calls++;
            return 7;
        },
        0,
    );

    assert.equal(answer, "7");
    assert.equal(calls, 1);
    assert.equal(stats.runs, 1);
    assert.equal(stats.median, stats.cold);
});

test("every run gets a fresh subject", () => {
    const { answer, stats } = measure(
        () => ({ count: 0 }),
        (subject) => ++subject.count,
        SHORT_BUDGET_SECONDS,
    );

    assert.equal(answer, "1");
    assert.ok(stats.runs > 1);
});

test("a changing answer is an error", () => {
    let calls = 0;
    assert.throws(
        () =>
            measure(
                () => undefined,
                () => ++calls,
                SHORT_BUDGET_SECONDS,
            ),
        /the answer changed between runs/,
    );
});
