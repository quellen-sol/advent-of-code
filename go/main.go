package main

import (
	"advent/years"
	"flag"
	"fmt"
	"os"
	"time"
)

func main() {
	year := flag.Int("year", 0, "puzzle year")
	day := flag.Int("day", 0, "puzzle day")
	input := flag.String("input", "", "path to the input file")
	bench := flag.Float64("bench", 0, "seconds to spend measuring setup and each part; runs each one once if left out")
	flag.Parse()

	budget := time.Duration(*bench * float64(time.Second))
	if !years.Run(*year, *day, *input, budget) {
		fmt.Fprintf(os.Stderr, "No Go solution for %d day %d\n", *year, *day)
		os.Exit(1)
	}
}
