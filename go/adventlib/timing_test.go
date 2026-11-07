package adventlib

import (
	"strings"
	"testing"
	"time"
)

const shortBudget = 5 * time.Millisecond

func noSubject() struct{} {
	return struct{}{}
}

func TestDurationsScaleToAReadableUnit(t *testing.T) {
	cases := []struct {
		nanos    float64
		expected string
	}{
		{0, "0 ns"},
		{999, "999 ns"},
		{1_000, "1.000 µs"},
		{1_234_567, "1.235 ms"},
		{999_999_000, "999.999 ms"},
		{1_500_000_000, "1.500 s"},
		{65_000_000_000, "65.000 s"},
	}

	for _, c := range cases {
		if got := formatDuration(c.nanos); got != c.expected {
			t.Errorf("formatDuration(%v) = %q, want %q", c.nanos, got, c.expected)
		}
	}
}

func TestStatsSummarizeAnOddNumberOfSamples(t *testing.T) {
	got := summarize([]float64{3, 1, 2}, 9)
	expected := stats{median: 2, min: 1, mean: 2, stddev: 1, cold: 9, runs: 3}
	if got != expected {
		t.Errorf("got %+v, want %+v", got, expected)
	}
}

func TestMedianOfAnEvenNumberOfSamplesIsTheMiddlePairAveraged(t *testing.T) {
	if got := summarize([]float64{4, 1, 3, 2}, 0).median; got != 2.5 {
		t.Errorf("got %v, want 2.5", got)
	}
}

func TestASingleSampleHasNoDeviation(t *testing.T) {
	if got := summarize([]float64{5}, 5).stddev; got != 0 {
		t.Errorf("got %v, want 0", got)
	}
}

var exampleStats = stats{
	median: 1_500_000,
	min:    1_000_000,
	mean:   2_000_000,
	stddev: 250_000,
	cold:   9_000_000,
	runs:   42,
}

func TestRowsMatchTheSharedLayout(t *testing.T) {
	cases := []struct {
		label    string
		answer   string
		bench    bool
		expected string
	}{
		{"Part 1", "1132", false, "Part 1    1.500 ms  1132"},
		{"Setup", "", false, "Setup     1.500 ms"},
		{"Part 1", "1132", true, "Part 1    1.500 ms    1.000 ms    2.000 ms  250.000 µs    9.000 ms       42  1132"},
	}

	for _, c := range cases {
		if got := formatRow(c.label, exampleStats, c.answer, c.bench); got != c.expected {
			t.Errorf("got %q, want %q", got, c.expected)
		}
	}
}

func TestHeaderLinesUpWithTheColumns(t *testing.T) {
	row := formatRow("Part 1", exampleStats, "1132", true)
	// Compared in runes, because µ takes two bytes.
	answerColumn := len([]rune(row[:strings.Index(row, "1132")]))
	if got := strings.Index(header, "answer"); got != answerColumn {
		t.Errorf("header's answer is at %d, the row's is at %d", got, answerColumn)
	}
}

func TestNoBudgetMeansOneRun(t *testing.T) {
	calls := 0
	answer, s := measure(noSubject, func(struct{}) int {
		calls++
		return 7
	}, 0)

	if answer != "7" {
		t.Errorf("got answer %q, want 7", answer)
	}
	if calls != 1 {
		t.Errorf("ran %d times, want 1", calls)
	}
	if s.runs != 1 || s.median != s.cold {
		t.Errorf("got %+v, want one run whose median is the cold run", s)
	}
}

func TestEveryRunGetsAFreshSubject(t *testing.T) {
	prepare := func() *int { return new(int) }
	answer, s := measure(prepare, func(subject *int) int {
		*subject++
		return *subject
	}, shortBudget)

	if answer != "1" {
		t.Errorf("got answer %q, want 1", answer)
	}
	if s.runs <= 1 {
		t.Errorf("got %d runs, want more than 1", s.runs)
	}
}

func TestAChangingAnswerIsAnError(t *testing.T) {
	defer func() {
		message, ok := recover().(string)
		if !ok || !strings.Contains(message, "the answer changed between runs") {
			t.Errorf("got panic %q, want a changed answer", message)
		}
	}()

	calls := 0
	measure(noSubject, func(struct{}) int {
		calls++
		return calls
	}, shortBudget)
}
