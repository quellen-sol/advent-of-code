package adventlib

import (
	"advent/types"
	"fmt"
	"time"
)

func RunSolution[D1 types.Number, D2 types.Number](s types.Solution[D1, D2]) {
	err := s.LoadInput()
	if err != nil {
		panic("Unable to load input: " + err.Error())
	}
	d1Start := time.Now()
	d1Result := s.RunPart1()
	d1Elapsed := time.Since(d1Start)
	fmt.Println("Part 1 solution:", d1Result)
	fmt.Println("Elapsed:", d1Elapsed)

	d2Start := time.Now()
	d2Result := s.RunPart2()
	d2Elapsed := time.Since(d2Start)
	fmt.Println("Part 2 solution:", d2Result)
	fmt.Println("Elapsed:", d2Elapsed)

	totalTime := d1Elapsed + d2Elapsed

	fmt.Println("Total time:", totalTime)
}
