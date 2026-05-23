package y2025

import (
	"testing"
)

func TestPart2(t *testing.T) {
	s := Day1Solution{
		input: "R50\nL299",
	}
	expected := 3

	d2s := s.RunPart2()
	if d2s != uint(expected) {
		t.Errorf("Result was not %v! Got %v", expected, d2s)
	}
}
