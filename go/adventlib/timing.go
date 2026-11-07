package adventlib

import (
	"fmt"
	"math"
	"slices"
	"strings"
	"time"
)

// Every language implements these same rules and output, so that their numbers
// can be compared. See "Timing" in the README before changing one of them.

const (
	// Parts fast enough to reach this stop early instead of using the whole budget.
	maxRuns = 100_000

	// The share of the budget spent on warm-up runs, which aren't recorded.
	warmupShare = 0.2

	header = "            median         min        mean      stddev        cold     runs  answer"
)

// stats holds times in nanoseconds.
type stats struct {
	median float64
	min    float64
	mean   float64
	stddev float64
	cold   float64
	runs   int
}

func summarize(samples []float64, cold float64) stats {
	sorted := slices.Sorted(slices.Values(samples))

	runs := len(sorted)
	middle := runs / 2
	median := sorted[middle]
	if runs%2 == 0 {
		median = (sorted[middle-1] + sorted[middle]) / 2
	}

	sum := 0.0
	for _, sample := range sorted {
		sum += sample
	}
	mean := sum / float64(runs)

	squaredError := 0.0
	for _, sample := range sorted {
		squaredError += (sample - mean) * (sample - mean)
	}
	stddev := math.Sqrt(squaredError / float64(max(runs-1, 1)))

	return stats{median, sorted[0], mean, stddev, cold, runs}
}

func formatDuration(nanos float64) string {
	switch {
	case nanos < 1e3:
		return fmt.Sprintf("%.0f ns", nanos)
	case nanos < 1e6:
		return fmt.Sprintf("%.3f µs", nanos/1e3)
	case nanos < 1e9:
		return fmt.Sprintf("%.3f ms", nanos/1e6)
	default:
		return fmt.Sprintf("%.3f s", nanos/1e9)
	}
}

func formatRow(label string, s stats, answer string, bench bool) string {
	row := fmt.Sprintf("%-6s  %10s", label, formatDuration(s.median))
	if bench {
		row += fmt.Sprintf(
			"  %10s  %10s  %10s  %10s  %7d",
			formatDuration(s.min),
			formatDuration(s.mean),
			formatDuration(s.stddev),
			formatDuration(s.cold),
			s.runs,
		)
	}
	return strings.TrimRight(row+"  "+answer, " ")
}

// measure times run on a fresh value from prepare each time, keeping prepare
// outside the timer. A zero budget means a single run.
func measure[S any, T any](prepare func() S, run func(S) T, budget time.Duration) (string, stats) {
	timeRun := func() (float64, string) {
		subject := prepare()
		start := time.Now()
		result := run(subject)
		elapsed := time.Since(start)
		return float64(elapsed.Nanoseconds()), fmt.Sprint(result)
	}

	started := time.Now()
	cold, answer := timeRun()

	warmup := time.Duration(float64(budget) * warmupShare)
	samples := []float64{}
	for time.Since(started) < budget && len(samples) < maxRuns {
		warmingUp := time.Since(started) < warmup
		elapsed, result := timeRun()
		if result != answer {
			panic(fmt.Sprintf("the answer changed between runs: %s, then %s", answer, result))
		}
		if !warmingUp {
			samples = append(samples, elapsed)
		}
	}

	if len(samples) == 0 {
		samples = append(samples, cold)
	}
	return answer, summarize(samples, cold)
}

type timer struct {
	budget time.Duration
	total  float64
}

func newTimer(budget time.Duration) *timer {
	t := &timer{budget: budget}
	if t.isBench() {
		fmt.Println(header)
	}
	return t
}

func (t *timer) isBench() bool {
	return t.budget > 0
}

func (t *timer) printTotal() {
	fmt.Printf("%-6s  %10s\n", "Total", formatDuration(t.total))
}

// timeRow is a function because Go methods can't take type parameters. It
// prints as soon as it finishes, so a later panic can't hide the row.
func timeRow[S any, T any](t *timer, label string, prepare func() S, run func(S) T) {
	answer, s := measure(prepare, run, t.budget)
	fmt.Println(formatRow(label, s, answer, t.isBench()))
	t.total += s.median
}
