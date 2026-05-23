package y2025

import (
	"math"
	"os"
	"strconv"
	"strings"
)

type Dial struct {
	position int
}

func (d *Dial) TurnLeft(by int) {
	d.position = (d.position - by) % 100
	if d.position < 0 {
		d.position += 100
	}
}

func (d *Dial) TurnRight(by int) {
	d.position = (d.position + by) % 100
}

type Day1Solution struct {
	input string
}

func (s *Day1Solution) LoadInput() error {
	v, err := os.ReadFile("./inputs/2025/1.txt")
	if err != nil {
		return err
	}

	s.input = string(v)
	return nil
}

func (s *Day1Solution) RunPart1() uint {
	var num_zeros uint = 0
	dial := Dial{position: 50}
	for line := range strings.Lines(s.input) {
		line := strings.Trim(line, "\n")
		dir := string(line[0])
		num, err := strconv.Atoi(string(line[1:]))
		if err != nil {
			panic(err.Error())
		}
		switch dir {
		case "L":
			dial.TurnLeft(num)
		case "R":
			dial.TurnRight(num)
		}
		if dial.position == 0 {
			num_zeros++
		}
	}

	return num_zeros
}

func (s *Day1Solution) RunPart2() uint {
	var num_zeros uint = 0
	dial := 50
	for line := range strings.Lines(s.input) {
		line := strings.Trim(line, "\n")
		dir := string(line[0])
		num, err := strconv.Atoi(string(line[1:]))
		if err != nil {
			panic(err.Error())
		}
		switch dir {
		case "L":
			oldDial := dial
			dial -= num
			if dial < 0 {
				if oldDial != 0 {
					num_zeros++
				}
				quo := math.Floor(math.Abs(float64(dial) / 100.0))
				num_zeros += uint(quo)
				dial = dial % 100
				if dial < 0 {
					dial += 100
				}
			} else if dial == 0 {
				num_zeros++
			}
		case "R":
			dial += num
			quo := math.Floor(float64(dial) / 100.0)
			dial = dial % 100
			num_zeros += uint(quo)
		}
	}

	return num_zeros
}
