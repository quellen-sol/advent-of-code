package adventlib

import (
	"advent/types"
	"time"
)

// RunSolution times setup and both parts of a solution. It takes a constructor
// rather than a value because each run gets a new instance, so a part that
// mutates its solution can't change what the next run measures.
func RunSolution[S types.Solution[D1, D2], D1 types.Number, D2 types.Number](
	newSolution func() S,
	inputPath string,
	budget time.Duration,
) {
	create := func() S {
		s := newSolution()
		if err := s.LoadInput(inputPath); err != nil {
			panic("Unable to load input: " + err.Error())
		}
		return s
	}

	t := newTimer(budget)
	timeRow(t, "Setup", func() struct{} { return struct{}{} }, func(struct{}) string {
		create()
		return ""
	})
	timeRow(t, "Part 1", create, func(s S) D1 { return s.RunPart1() })
	timeRow(t, "Part 2", create, func(s S) D2 { return s.RunPart2() })
	t.printTotal()
}
