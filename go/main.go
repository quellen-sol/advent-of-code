package main

import (
	"advent/years"
	"flag"
	"fmt"
	"os"
)

func main() {
	year := flag.Int("year", 0, "puzzle year")
	day := flag.Int("day", 0, "puzzle day")
	input := flag.String("input", "", "path to the input file")
	flag.Parse()

	if !years.Run(*year, *day, *input) {
		fmt.Fprintf(os.Stderr, "No Go solution for %d day %d\n", *year, *day)
		os.Exit(1)
	}
}
