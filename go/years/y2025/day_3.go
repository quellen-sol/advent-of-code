package y2025

import (
	"os"
	"strconv"
	"strings"
)

type Day3Solution struct {
	input string
}

func (s *Day3Solution) LoadInput() error {
	v, err := os.ReadFile("./inputs/2025/3.txt")
	if err != nil {
		return err
	}

	s.input = string(v)
	return nil
}

func (s *Day3Solution) RunPart1() uint {
	var totalJoltage uint = 0
	for line := range strings.Lines(s.input) {
		maxes := [][2]int{}
		for _, r := range strings.Trim(line, "\n") {
			cnum, err := strconv.Atoi(string(r))
			if err != nil {
				panic("Bad conversion from " + string(r) + " to integer")
			}

			for j, m := range maxes {
				cmax := m[1]
				if (cmax != -1 && cnum > cmax) || cmax == -1 {
					maxes[j][1] = cnum
				}
			}

			maxes = append(maxes, [2]int{cnum, -1})
		}

		// Calc max
		var maxJoltage uint = 0
		for _, vals := range maxes {
			if vals[1] == -1 {
				continue
			}
			value := vals[0] * 10 + vals[1]
			if value > int(maxJoltage) {
				maxJoltage = uint(value)
			}
		}

		totalJoltage += maxJoltage
	}
	return totalJoltage
}

func (s *Day3Solution) RunPart2() uint {
	return 0
}
